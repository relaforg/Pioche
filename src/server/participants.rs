use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::server::{draw_kind::DrawKindDto, error::AppError};

pub const MAX_PARTICIPANTS: usize = 100;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantDto {
    pub id: i32,
    pub name: String,
    pub form_completed_at: Option<DateTime<FixedOffset>>,
}

pub struct Participants(Vec<String>);

impl Participants {
    pub fn parse(kind: DrawKindDto, raw: Vec<String>) -> Result<Self, AppError> {
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

#[cfg(feature = "ssr")]
mod convert {
    use crate::{entities::participants, server::participants::ParticipantDto};

    impl From<participants::Model> for ParticipantDto {
        fn from(value: participants::Model) -> Self {
            ParticipantDto {
                id: value.id,
                name: value.name,
                form_completed_at: value.form_completed_at,
            }
        }
    }
}

pub fn normalize_participant(raw: &str) -> Option<String> {
    let name = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    (!name.is_empty()).then_some(name)
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
