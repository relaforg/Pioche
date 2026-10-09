use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::error::AppError;

pub const MAX_PARTICIPANTS: usize = 100;

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DrawKind {
    SecretSanta,
    Teams,
}

impl DrawKind {
    pub fn form_value(self) -> &'static str {
        match self {
            DrawKind::SecretSanta => "SecretSanta",
            DrawKind::Teams => "Teams",
        }
    }

    pub fn min_participants(self) -> usize {
        match self {
            DrawKind::SecretSanta => 3,
            DrawKind::Teams => 2,
        }
    }
}

pub struct Participants(Vec<String>);

impl Participants {
    pub fn parse(kind: DrawKind, raw: Vec<String>) -> Result<Self, AppError> {
        let names = parse_names(raw)?;

        if names.len() < kind.min_participants() {
            return Err(AppError::Invalid("Pas assez de participants".into()));
        }

        Ok(Participants(names))
    }

    pub fn into_inner(self) -> Vec<String> {
        self.0
    }
}

pub struct Exclusion((String, String));

impl Exclusion {
    pub fn new(couple: (String, String)) -> Result<Self, AppError> {
        if couple.0 == couple.1 {
            return Err(AppError::Invalid(
                "Une exclusion ne peux pas contenir 2 fois la même personne".into(),
            ));
        }
        Ok(Exclusion(couple))
    }
}

pub fn parse_name(raw: &str) -> Result<String, AppError> {
    let Some(name) = normalize_participant(raw) else {
        return Err(AppError::Invalid("Un prénom est vide".into()));
    };
    if name.chars().count() > 50 {
        return Err(AppError::Invalid(
            "Prénom trop long (50 character max)".into(),
        ));
    }
    Ok(name)
}

pub fn parse_names<I>(raw: I) -> Result<Vec<String>, AppError>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut seen = HashSet::new();
    let mut names = Vec::new();

    for r in raw {
        if names.len() == MAX_PARTICIPANTS {
            return Err(AppError::Invalid("Trop de participants".into()));
        }
        let name = parse_name(r.as_ref())?;
        if !seen.insert(name.to_lowercase()) {
            return Err(AppError::Invalid(format!(
                "Les doublons ne sont pas autorisés : {name}"
            )));
        }
        names.push(name);
    }

    Ok(names)
}

pub fn normalize_participant(raw: &str) -> Option<String> {
    let name = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    (!name.is_empty()).then_some(name)
}

#[server]
pub async fn add_draw(
    name: String,
    kind: DrawKind,
    #[server(default)] participants: Vec<String>,
) -> Result<i32, AppError> {
    use crate::server::session::require_user;
    use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};

    let user = require_user()?;
    let participants = Participants::parse(kind, participants)?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    Ok(23)
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
