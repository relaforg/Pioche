use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::{draw_kind::DrawKindDto, error::AppError};

pub const MAX_PARTICIPANTS: usize = 100;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParticipantDto {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
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

    pub fn position(&self, raw: &str) -> Option<usize> {
        let key = normalize_participant(raw)?.to_lowercase();
        self.0.iter().position(|n| n.to_lowercase() == key)
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
                email: value.email,
                form_completed_at: value.form_completed_at,
            }
        }
    }
}

#[server]
pub async fn update_participant_email(participant_id: i32, email: String) -> Result<(), AppError> {
    use crate::entities::{draws, participants};
    use crate::server::session::require_user;
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel,
    };

    let user = require_user()?;
    let email = parse_email(&email)?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let (participant, draw) = participants::Entity::find_by_id(participant_id)
        .find_also_related(draws::Entity)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;
    let draw = draw.ok_or(AppError::Internal)?;

    if draw.owner_id != user.id {
        leptos_axum::redirect("/");
        return Ok(());
    }
    // Les emails se saisissent entre le tirage et l'envoi des liens.
    if draw.drawn_at.is_none() {
        return Err(AppError::Invalid("Lance d'abord le tirage".into()));
    }
    if draw.links_sent_at.is_some() {
        return Err(AppError::Invalid(
            "Les liens sont envoyés : les emails ne sont plus modifiables".into(),
        ));
    }

    let mut participant = participant.into_active_model();
    participant.email = Set(email);
    participant.update(&db).await?;

    leptos_axum::redirect(&format!("/tirage/{}", draw.id));
    Ok(())
}

/// Champ vide = on efface l'email. Validation volontairement légère :
/// la seule vraie preuve qu'un email existe, c'est d'y envoyer quelque chose.
pub fn parse_email(raw: &str) -> Result<Option<String>, AppError> {
    let email = raw.trim();
    if email.is_empty() {
        return Ok(None);
    }
    let valid = email.len() <= 254
        && !email.contains(char::is_whitespace)
        && email
            .split_once('@')
            .is_some_and(|(local, domain)| !local.is_empty() && domain.contains('.'));
    if !valid {
        return Err(AppError::Invalid(format!("'{email}' n'est pas un email valide")));
    }
    Ok(Some(email.to_lowercase()))
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
