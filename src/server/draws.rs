use chrono::{DateTime, FixedOffset, NaiveDate};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sea_orm::{ActiveModelTrait, ConnectionTrait};
use serde::{Deserialize, Serialize};

use crate::server::{
    draw_kind::DrawKindDto, error::AppError, exclusions::ExclusionDto,
    participants::ParticipantDto, question_kind::QuestionKindDto, team_mode::TeamModeDto,
};
#[cfg(feature = "ssr")]
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
