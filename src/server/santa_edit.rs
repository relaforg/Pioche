//! Modifications d'un Secret Santa, possibles uniquement avant le tirage.

use chrono::NaiveDate;
use leptos::prelude::*;

use crate::server::{error::AppError, exclusions::ExclusionDto};
#[cfg(feature = "ssr")]
use crate::entities::draws;

/// Le tirage de l'utilisateur courant, s'il est encore modifiable.
#[cfg(feature = "ssr")]
async fn editable_draw(
    db: &impl sea_orm::ConnectionTrait,
    draw_id: i32,
) -> Result<draws::Model, AppError> {
    use crate::entities::sea_orm_active_enums::DrawKind;
    use crate::server::session::require_user;
    use sea_orm::EntityTrait;

    let user = require_user()?;
    let draw = draws::Entity::find_by_id(draw_id)
        .one(db)
        .await?
        .ok_or(AppError::Internal)?;

    if draw.owner_id != user.id || draw.kind != DrawKind::SecretSanta {
        return Err(AppError::Unauthorized);
    }
    if draw.drawn_at.is_some() {
        return Err(AppError::Invalid(
            "Le tirage a déjà été effectué : il n'est plus modifiable".into(),
        ));
    }

    Ok(draw)
}

#[server]
pub async fn add_santa_participant(draw_id: i32, name: String) -> Result<(), AppError> {
    use crate::entities::participants;
    use crate::server::participants::{parse_name, MAX_PARTICIPANTS};
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, ModelTrait};

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    let draw = editable_draw(&db, draw_id).await?;
    let name = parse_name(&name)?;

    let existing = draw.find_related(participants::Entity).all(&db).await?;
    if existing.len() >= MAX_PARTICIPANTS {
        return Err(AppError::Invalid(format!(
            "{MAX_PARTICIPANTS} participants maximum"
        )));
    }
    let key = name.to_lowercase();
    if existing.iter().any(|p| p.name.to_lowercase() == key) {
        return Err(AppError::Invalid(format!("'{name}' est déjà dans la liste")));
    }

    participants::ActiveModel {
        draw_id: Set(draw.id),
        name: Set(name),
        ..Default::default()
    }
    .insert(&db)
    .await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}

#[server]
pub async fn remove_santa_participant(participant_id: i32) -> Result<(), AppError> {
    use crate::entities::participants;
    use crate::server::draw_kind::DrawKindDto;
    use sea_orm::{
        ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    };

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    let participant = participants::Entity::find_by_id(participant_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;
    let draw = editable_draw(&db, participant.draw_id).await?;

    let count = participants::Entity::find()
        .filter(participants::Column::DrawId.eq(draw.id))
        .count(&db)
        .await?;
    let min = DrawKindDto::SecretSanta.min_participants();
    if count as usize <= min {
        return Err(AppError::Invalid(format!(
            "Il faut au moins {min} participants"
        )));
    }

    // Ses exclusions partent avec lui (ON DELETE CASCADE).
    participant.delete(&db).await?;

    leptos_axum::redirect(&format!("/tirage/{}", draw.id));
    Ok(())
}

#[server]
pub async fn update_santa_settings(
    draw_id: i32,
    #[server(default)] exclusions: Vec<ExclusionDto>,
    budget: Option<f64>,
    date: Option<NaiveDate>,
) -> Result<(), AppError> {
    use crate::entities::{exclusions, participants, santa_settings};
    use crate::server::{
        draw_kind::DrawKindDto, exclusions::Exclusions, participants::Participants,
        secret_santa_option::SecretSantaOptions,
    };
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait,
        IntoActiveModel, ModelTrait, QueryFilter, TransactionTrait,
    };

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    let draw = editable_draw(&db, draw_id).await?;
    let options = SecretSantaOptions::new(budget, date)?;

    let participants = draw
        .find_related(participants::Entity)
        .order_by_id_asc()
        .all(&db)
        .await?;
    let ids: Vec<i32> = participants.iter().map(|p| p.id).collect();
    let names = Participants::parse(
        DrawKindDto::SecretSanta,
        participants.into_iter().map(|p| p.name).collect(),
    )?;
    let exclusions_parsed = Exclusions::parse(exclusions, &names)?;

    let txn = db.begin().await?;

    let mut settings = santa_settings::Entity::find_by_id(draw.id)
        .one(&txn)
        .await?
        .ok_or(AppError::Internal)?
        .into_active_model();
    settings.budget_cents = Set(options.budget_cents);
    settings.exchange_date = Set(options.exchange_date);
    settings.update(&txn).await?;

    // Le formulaire renvoie la liste complète : on remplace plutôt que de calculer un diff.
    exclusions::Entity::delete_many()
        .filter(exclusions::Column::ParticipantA.is_in(ids.clone()))
        .exec(&txn)
        .await?;
    exclusions::Entity::insert_many(exclusions_parsed.into_active_models(&ids))
        .exec(&txn)
        .await?;

    txn.commit().await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}
