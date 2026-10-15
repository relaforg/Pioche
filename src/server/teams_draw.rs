#[cfg(feature = "ssr")]
use std::collections::HashMap;

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sea_orm::{
    ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, QueryFilter,
};

#[cfg(feature = "ssr")]
use crate::entities::{draws, participants};
use crate::server::{
    draw_kind::DrawKindDto, error::AppError, exclusions::ExclusionDto, participants::Participants,
    team_mode::TeamModeDto,
};

#[server]
pub async fn update_and_draw_teams(
    draw_id: i32,
    #[server(default)] participants: Vec<String>,
    #[server(default)] exclusions: Vec<ExclusionDto>,
    team_mode: Option<TeamModeDto>,
    team_value: Option<i16>,
) -> Result<(), AppError> {
    use crate::entities::draws;
    use crate::server::session::require_user;
    use sea_orm::{DatabaseConnection, EntityTrait};

    let user = require_user()?;
    let participants = Participants::parse(DrawKindDto::Teams, participants)?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let draw = draws::Entity::find_by_id(draw_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;

    if draw.owner_id != user.id {
        leptos_axum::redirect(&format!("/"));
        return Ok(());
    }

    update(&db, draw, participants).await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}

#[cfg(feature = "ssr")]
async fn update(
    db: &impl ConnectionTrait,
    draw: draws::Model,
    participants: Participants,
) -> Result<(), AppError> {
    let mut participants_back: HashMap<String, i32> = draw
        .find_related(participants::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.name, p.id))
        .collect();

    let mut to_add = Vec::new();
    let participants_front = participants.into_inner();

    for p in participants_front {
        if participants_back.remove(&p).is_none() {
            to_add.push(p);
        }
    }

    participants::Entity::insert_many(
        to_add
            .into_iter()
            .map(|name| participants::ActiveModel {
                name: Set(name),
                draw_id: Set(draw.id),
                ..Default::default()
            })
            .collect::<Vec<participants::ActiveModel>>(),
    )
    .exec(db)
    .await?;

    participants::Entity::delete_many()
        .filter(participants::Column::DrawId.eq(draw.id))
        .filter(participants::Column::Id.is_in(participants_back.into_values()))
        .exec(db)
        .await?;
    Ok(())
}
