use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::{draw_kind::DrawKindDto, error::AppError, participants::ParticipantDto};

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
) -> Result<i32, AppError> {
    // use crate::server::session::require_user;
    // use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
    //
    // let user = require_user()?;
    // let participants = Participants::parse(kind, participants)?;
    // let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    Ok(23)
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
