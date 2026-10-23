use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::{
    error::AppError, question_kind::QuestionKindDto, secret_santa_option::SecretSantaOptions,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerDto {
    pub question: QuestionKindDto,
    pub value: String,
}

/// Ce que voit un participant en ouvrant son lien perso.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SantaRevealDto {
    pub draw_name: String,
    pub participant_name: String,
    pub receiver_name: String,
    pub options: SecretSantaOptions,
    /// Questions choisies par l'organisateur, dans l'ordre d'affichage.
    pub questions: Vec<QuestionKindDto>,
    pub my_answers: Vec<AnswerDto>,
    pub my_form_completed: bool,
    /// `None` tant que le binôme n'a pas rempli sa fiche.
    pub receiver_answers: Option<Vec<AnswerDto>>,
}

/// Le participant désigné par le jeton d'un lien perso.
#[cfg(feature = "ssr")]
async fn participant_from_token(
    db: &sea_orm::DatabaseConnection,
    token: &str,
) -> Result<crate::entities::participants::Model, AppError> {
    use crate::entities::participants;
    use crate::server::ssr::session::hash_token;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let invalid = || AppError::Invalid("Ce lien n'est pas (ou plus) valide".into());
    let digest = hash_token(token).ok_or_else(invalid)?;

    participants::Entity::find()
        .filter(participants::Column::AccessToken.eq(hex::encode(digest)))
        .one(db)
        .await?
        .ok_or_else(invalid)
}

/// Questions du tirage, triées dans l'ordre d'affichage.
#[cfg(feature = "ssr")]
async fn draw_questions(
    db: &sea_orm::DatabaseConnection,
    draw_id: i32,
) -> Result<Vec<QuestionKindDto>, AppError> {
    use crate::entities::draw_questions;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let mut questions: Vec<QuestionKindDto> = draw_questions::Entity::find()
        .filter(draw_questions::Column::DrawId.eq(draw_id))
        .all(db)
        .await?
        .into_iter()
        .map(|q| q.question.into())
        .collect();
    questions.sort();
    Ok(questions)
}

/// Réponses d'un participant, limitées aux questions du tirage et dans leur ordre.
#[cfg(feature = "ssr")]
async fn answers_of(
    db: &sea_orm::DatabaseConnection,
    participant_id: i32,
    questions: &[QuestionKindDto],
) -> Result<Vec<AnswerDto>, AppError> {
    use crate::entities::answers;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let mut answers: Vec<AnswerDto> = answers::Entity::find()
        .filter(answers::Column::ParticipantId.eq(participant_id))
        .all(db)
        .await?
        .into_iter()
        .map(|a| AnswerDto {
            question: a.question.into(),
            value: a.value,
        })
        .filter(|a| questions.contains(&a.question))
        .collect();
    answers.sort_by_key(|a| a.question);
    Ok(answers)
}

/// Page publique, sans compte : le jeton du lien suffit à identifier le participant.
#[server]
pub async fn get_santa_reveal(token: String) -> Result<SantaRevealDto, AppError> {
    use crate::entities::{draws, participants, santa_assignments, santa_settings};
    use sea_orm::{DatabaseConnection, EntityTrait};

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    let giver = participant_from_token(&db, &token).await?;

    let assignment = santa_assignments::Entity::find_by_id(giver.id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;
    let receiver = participants::Entity::find_by_id(assignment.receiver_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;
    let draw = draws::Entity::find_by_id(giver.draw_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;
    let options = santa_settings::Entity::find_by_id(draw.id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;

    let questions = draw_questions(&db, draw.id).await?;
    let my_answers = answers_of(&db, giver.id, &questions).await?;
    let receiver_answers = match receiver.form_completed_at {
        Some(_) => Some(answers_of(&db, receiver.id, &questions).await?),
        None => None,
    };

    Ok(SantaRevealDto {
        draw_name: draw.name,
        participant_name: giver.name,
        receiver_name: receiver.name,
        options: options.into(),
        questions,
        my_answers,
        my_form_completed: giver.form_completed_at.is_some(),
        receiver_answers,
    })
}

/// Enregistre la fiche du participant : remplace toutes ses réponses.
#[server]
pub async fn save_santa_answers(
    token: String,
    #[server(default)] answers: Vec<AnswerDto>,
) -> Result<(), AppError> {
    use crate::entities::answers as answers_entity;
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
        IntoActiveModel, QueryFilter, TransactionTrait,
    };

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    let participant = participant_from_token(&db, &token).await?;
    let questions = draw_questions(&db, participant.draw_id).await?;

    let mut rows = Vec::new();
    for AnswerDto { question, value } in answers {
        if !questions.contains(&question) {
            return Err(AppError::Invalid(format!(
                "« {} » ne fait pas partie de ce tirage",
                question.label()
            )));
        }
        if let Some(value) = question.parse_answer(&value)? {
            rows.push(answers_entity::ActiveModel {
                participant_id: Set(participant.id),
                question: Set(question.into()),
                value: Set(value),
                ..Default::default()
            });
        }
    }

    let txn = db.begin().await?;

    answers_entity::Entity::delete_many()
        .filter(answers_entity::Column::ParticipantId.eq(participant.id))
        .exec(&txn)
        .await?;
    if !rows.is_empty() {
        answers_entity::Entity::insert_many(rows).exec(&txn).await?;
    }

    // On garde la date du premier remplissage : c'est celle qu'affiche l'organisateur.
    if participant.form_completed_at.is_none() {
        let mut participant = participant.into_active_model();
        participant.form_completed_at = Set(Some(chrono::Utc::now().fixed_offset()));
        participant.update(&txn).await?;
    }

    txn.commit().await?;

    leptos_axum::redirect(&format!("/s/{token}"));
    Ok(())
}
