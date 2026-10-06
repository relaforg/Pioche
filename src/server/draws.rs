use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::error::AppError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Draw {
    pub id: i32,
    pub owner_id: i32,
    pub kind: DrawKind,
    pub name: String,
    pub share_token: Option<String>,
    pub drawn_at: Option<DateTime<FixedOffset>>,
    pub created_at: DateTime<FixedOffset>,
    pub participants: Vec<Participant>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Participant {
    pub id: i32,
    pub name: String,
    pub form_completed_at: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum DrawKind {
    SecretSanta,
    Teams,
}

#[server]
pub async fn get_user_draw() -> Result<Vec<Draw>, AppError> {
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
        .map(Into::<Draw>::into)
        .collect::<Vec<Draw>>();
    Ok(result)
}

#[cfg(feature = "ssr")]
mod convert {
    use crate::{
        entities::{draws, participants, sea_orm_active_enums},
        server::draws::{Draw, DrawKind, Participant},
    };

    impl From<(draws::Model, Vec<participants::Model>)> for Draw {
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

            Draw {
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

    impl From<participants::Model> for Participant {
        fn from(value: participants::Model) -> Self {
            Participant {
                id: value.id,
                name: value.name,
                form_completed_at: value.form_completed_at,
            }
        }
    }

    impl From<sea_orm_active_enums::DrawKind> for DrawKind {
        fn from(value: sea_orm_active_enums::DrawKind) -> Self {
            match value {
                sea_orm_active_enums::DrawKind::SecretSanta => DrawKind::SecretSanta,
                sea_orm_active_enums::DrawKind::Teams => DrawKind::Teams,
            }
        }
    }
}
