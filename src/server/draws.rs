use chrono::{DateTime, FixedOffset, NaiveDate};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::entities::{draw_questions, draws, participants, santa_settings, team_settings, teams};
use crate::server::{
    draw_kind::DrawKindDto, error::AppError, exclusions::ExclusionDto,
    participants::ParticipantDto, question_kind::QuestionKindDto, team_mode::TeamModeDto,
};
use crate::server::{secret_santa_option::SecretSantaOptions, teams_options::TeamOptions};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrawDto {
    pub id: i32,
    pub owner_id: i32,
    pub kind: DrawKindDto,
    pub name: String,
    pub share_token: Option<String>,
    pub drawn_at: Option<DateTime<FixedOffset>>,
    pub created_at: DateTime<FixedOffset>,
    pub participants: Vec<ParticipantDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamDto {
    pub name: String,
    pub members: Vec<ParticipantDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DrawSettingsDto {
    Teams {
        options: TeamOptions,
        teams: Vec<TeamDto>,
    },
    SecretSanta {
        options: SecretSantaOptions,
        questions: Vec<QuestionKindDto>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrawDetailDto {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub share_token: Option<String>,
    pub drawn_at: Option<DateTime<FixedOffset>>,
    pub participants: Vec<ParticipantDto>,
    pub settings: DrawSettingsDto,
    pub exclusions: Vec<ExclusionDto>,
}

#[server]
pub async fn add_draw(
    name: String,
    kind: DrawKindDto,
    #[server(default)] participants: Vec<String>,
    #[server(default)] exclusions: Vec<ExclusionDto>,
    budget: Option<f64>,
    date: Option<NaiveDate>,
    question_kind: Option<Vec<QuestionKindDto>>,
    team_mode: Option<TeamModeDto>,
    team_value: Option<i16>,
) -> Result<i32, AppError> {
    use crate::entities::{draw_questions, draws, exclusions, participants};
    use crate::server::session::require_user;
    use crate::server::ssr::session::generate_token;
    use crate::server::{
        exclusions::Exclusions, participants::Participants,
        secret_santa_option::SecretSantaOptions, teams_options::TeamOptions,
    };
    use leptos_axum::redirect;
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, TransactionTrait,
    };

    let user = require_user()?;
    let participants = Participants::parse(kind, participants)?;
    let exclusions = Exclusions::parse(exclusions, &participants)?;
    let names = participants.into_inner();
    let (share_token, _) = generate_token();

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let txn = db.begin().await?;

    let draw = draws::ActiveModel {
        owner_id: Set(user.id),
        kind: Set(kind.into()),
        name: Set(name),
        share_token: Set(Some(share_token)),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    let participants =
        participants::Entity::insert_many(names.iter().map(|p| participants::ActiveModel {
            draw_id: Set(draw.id),
            name: Set(p.into()),
            ..Default::default()
        }))
        .exec_with_returning(&txn)
        .await?;

    let ids = names
        .iter()
        .map(|name| participants.iter().find(|p| p.name == *name).map(|m| m.id))
        .collect::<Option<Vec<i32>>>()
        .ok_or(AppError::Internal)?;

    exclusions::Entity::insert_many(exclusions.into_active_models(&ids))
        .exec(&txn)
        .await?;

    match kind {
        DrawKindDto::SecretSanta => {
            insert_secret_santa_options(&txn, draw.id, SecretSantaOptions::new(budget, date)?)
                .await?;
            let questions: Vec<draw_questions::ActiveModel> = question_kind
                .map(|questions| {
                    questions
                        .into_iter()
                        .map(|q| draw_questions::ActiveModel {
                            draw_id: Set(draw.id),
                            question: Set(q.into()),
                            ..Default::default()
                        })
                        .collect()
                })
                .ok_or_else(|| AppError::Invalid("Aucune question selectionnée".into()))?;
            draw_questions::Entity::insert_many(questions)
                .exec(&txn)
                .await?;
        }
        DrawKindDto::Teams => {
            insert_teams_options(&txn, draw.id, TeamOptions::new(team_mode, team_value)?).await?
        }
    };

    txn.commit().await?;
    redirect(format!("/tirage/{}", draw.id).as_str());
    Ok(draw.id)
}

#[cfg(feature = "ssr")]
async fn insert_secret_santa_options(
    txn: &impl ConnectionTrait,
    draw_id: i32,
    options: SecretSantaOptions,
) -> Result<(), AppError> {
    use crate::entities::santa_settings;
    use sea_orm::ActiveValue::Set;

    santa_settings::ActiveModel {
        draw_id: Set(draw_id),
        budget_cents: Set(options.budget_cents),
        exchange_date: Set(options.exchange_date),
        ..Default::default()
    }
    .insert(txn)
    .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn insert_teams_options(
    txn: &impl ConnectionTrait,
    draw_id: i32,
    options: TeamOptions,
) -> Result<(), AppError> {
    use crate::entities::team_settings;
    use sea_orm::ActiveValue::Set;

    team_settings::ActiveModel {
        draw_id: Set(draw_id),
        mode: Set(options.team_mode.into()),
        value: Set(options.team_value),
        ..Default::default()
    }
    .insert(txn)
    .await?;

    Ok(())
}

#[server]
pub async fn get_user_draw() -> Result<Vec<DrawDto>, AppError> {
    use crate::entities::{draws, participants};
    use crate::server::session::require_user;
    use sea_orm::QueryOrder;
    use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};

    let user = require_user()?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let draws = draws::Entity::find()
        .filter(draws::Column::OwnerId.eq(user.id))
        .order_by_desc(draws::Column::CreatedAt)
        .order_by_desc(draws::Column::Id)
        .all(&db)
        .await?;

    let participants = draws
        .load_many(
            participants::Entity::find().order_by_asc(participants::Column::Name),
            &db,
        )
        .await?;

    let result = draws
        .into_iter()
        .zip(participants)
        .map(Into::<DrawDto>::into)
        .collect::<Vec<DrawDto>>();
    Ok(result)
}

#[server]
pub async fn get_draw_by_id(id: i32) -> Result<DrawDetailDto, AppError> {
    use crate::entities::sea_orm_active_enums::DrawKind;
    use crate::entities::{draws, exclusions, participants};
    use crate::server::session::require_user;
    use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

    let user = require_user()?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let Some(draw) = draws::Entity::find_by_id(id)
        .filter(draws::Column::OwnerId.eq(user.id))
        .one(&db)
        .await?
    else {
        return Err(AppError::Invalid("Id tirage invalide".into()));
    };

    let participants = draw
        .find_related(participants::Entity)
        .order_by_id_asc()
        .all(&db)
        .await?;

    let ids: Vec<i32> = participants.iter().map(|p| p.id).collect();

    let exclusions = exclusions::Entity::find()
        .filter(exclusions::Column::ParticipantA.is_in(ids))
        .all(&db)
        .await?;

    let exclusions = exclusions
        .iter()
        .map(|e| {
            let a = participants
                .iter()
                .find(|p| p.id == e.participant_a)
                .ok_or(AppError::Internal)?;
            let b = participants
                .iter()
                .find(|p| p.id == e.participant_b)
                .ok_or(AppError::Internal)?;
            Ok(ExclusionDto {
                a: a.name.clone(),
                b: b.name.clone(),
            })
        })
        .collect::<Result<Vec<ExclusionDto>, AppError>>()?;

    let settings = match draw.kind {
        DrawKind::SecretSanta => load_secret_santa_setting(&db, &draw).await?,
        DrawKind::Teams => load_teams_setting(&db, &draw, participants.as_slice()).await?,
    };

    Ok(DrawDetailDto {
        id,
        owner_id: draw.owner_id,
        name: draw.name,
        share_token: draw.share_token,
        drawn_at: draw.drawn_at,
        participants: participants.into_iter().map(Into::into).collect(),
        settings,
        exclusions,
    })
}

#[cfg(feature = "ssr")]
async fn load_teams_setting(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
    participants: &[participants::Model],
) -> Result<DrawSettingsDto, AppError> {
    let options = team_settings::Entity::find()
        .filter(team_settings::Column::DrawId.eq(draw.id))
        .one(db)
        .await?
        .ok_or(AppError::Internal)?;

    let teams = teams::Entity::find()
        .filter(teams::Column::DrawId.eq(draw.id))
        .all(db)
        .await?
        .iter()
        .map(|t| TeamDto {
            name: t.name.clone(),
            members: participants
                .iter()
                .filter(|p| p.team_id == Some(t.id))
                .cloned()
                .map(Into::into)
                .collect(),
        })
        .collect::<Vec<TeamDto>>();

    Ok(DrawSettingsDto::Teams {
        options: options.into(),
        teams: teams,
    })
}

#[cfg(feature = "ssr")]
async fn load_secret_santa_setting(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
) -> Result<DrawSettingsDto, AppError> {
    let options = santa_settings::Entity::find()
        .filter(santa_settings::Column::DrawId.eq(draw.id))
        .one(db)
        .await?
        .ok_or(AppError::Internal)?;

    let questions = draw_questions::Entity::find()
        .filter(draw_questions::Column::DrawId.eq(draw.id))
        .all(db)
        .await?;

    Ok(DrawSettingsDto::SecretSanta {
        options: options.into(),
        questions: questions
            .iter()
            .map(|q| q.question.clone().into())
            .collect(),
    })
}

#[cfg(feature = "ssr")]
mod convert {
    use crate::{
        entities::{draws, participants},
        server::draws::DrawDto,
    };

    impl From<(draws::Model, Vec<participants::Model>)> for DrawDto {
        fn from((draw, participants): (draws::Model, Vec<participants::Model>)) -> Self {
            let draws::Model {
                id,
                owner_id,
                kind,
                name,
                share_token,
                drawn_at,
                created_at,
            } = draw;

            DrawDto {
                id,
                owner_id,
                kind: kind.into(),
                name,
                share_token,
                drawn_at,
                created_at,
                participants: participants.into_iter().map(Into::into).collect(),
            }
        }
    }
}
