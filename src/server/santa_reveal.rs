use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::server::{error::AppError, secret_santa_option::SecretSantaOptions};

/// Ce que voit un participant en ouvrant son lien perso.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SantaRevealDto {
    pub draw_name: String,
    pub participant_name: String,
    pub receiver_name: String,
    pub options: SecretSantaOptions,
}

/// Page publique, sans compte : le jeton du lien suffit à identifier le participant.
#[server]
pub async fn get_santa_reveal(token: String) -> Result<SantaRevealDto, AppError> {
    use crate::entities::{draws, participants, santa_assignments, santa_settings};
    use crate::server::ssr::session::hash_token;
    use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

    let invalid = || AppError::Invalid("Ce lien n'est pas (ou plus) valide".into());

    let digest = hash_token(&token).ok_or_else(invalid)?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let giver = participants::Entity::find()
        .filter(participants::Column::AccessToken.eq(hex::encode(digest)))
        .one(&db)
        .await?
        .ok_or_else(invalid)?;

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

    Ok(SantaRevealDto {
        draw_name: draw.name,
        participant_name: giver.name,
        receiver_name: receiver.name,
        options: options.into(),
    })
}
