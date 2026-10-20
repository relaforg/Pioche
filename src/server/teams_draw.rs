#[cfg(feature = "ssr")]
use std::collections::{BTreeSet, HashMap};

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait,
    ExprTrait, IntoActiveModel, ModelTrait, QueryFilter,
};

use crate::server::{
    draw_kind::DrawKindDto, error::AppError, exclusions::ExclusionDto, participants::Participants,
    team_mode::TeamModeDto,
};
#[cfg(feature = "ssr")]
use crate::{
    entities::{draws, exclusions, participants, team_settings},
    server::exclusions::Exclusions,
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
    use sea_orm::{DatabaseConnection, EntityTrait, TransactionTrait};

    let user = require_user()?;
    let participants = Participants::parse(DrawKindDto::Teams, participants)?;
    let exclusions = Exclusions::parse(exclusions, &participants)?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let draw = draws::Entity::find_by_id(draw_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;

    if draw.owner_id != user.id {
        leptos_axum::redirect(&format!("/"));
        return Ok(());
    }

    let txn = db.begin().await?;
    update(&txn, draw, participants, exclusions, team_mode, team_value).await?;
    txn.commit().await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}

#[cfg(feature = "ssr")]
async fn update(
    db: &impl ConnectionTrait,
    draw: draws::Model,
    participants: Participants,
    exclusions: Exclusions,
    team_mode: Option<TeamModeDto>,
    team_value: Option<i16>,
) -> Result<(), AppError> {
    let ids = update_participants(db, &draw, participants).await?;
    update_exclusions(db, &draw, exclusions, ids.as_slice()).await?;
    update_team_settings(db, &draw, team_mode, team_value).await?;
    Ok(())
}

#[cfg(feature = "ssr")]
async fn update_team_settings(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
    team_mode: Option<TeamModeDto>,
    team_value: Option<i16>,
) -> Result<(), AppError> {
    let mut settings = team_settings::Entity::find()
        .filter(team_settings::Column::DrawId.eq(draw.id))
        .one(db)
        .await?
        .ok_or(AppError::Internal)?
        .into_active_model();

    if let Some(value) = team_value {
        settings.value = Set(value);
    }

    if let Some(mode) = team_mode {
        settings.mode = Set(mode.into());
    }

    settings.update(db).await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn update_participants(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
    participants: Participants,
) -> Result<Vec<i32>, AppError> {
    let mut participants_back: HashMap<String, i32> = draw
        .find_related(participants::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(|p| (p.name, p.id))
        .collect();

    let mut to_add = Vec::new();
    let participants_front = participants.into_inner();

    let slots: Vec<Option<i32>> = participants_front
        .iter()
        .map(|n| participants_back.remove(n))
        .collect();

    for (i, s) in slots.iter().enumerate() {
        if s.is_none() {
            to_add.push(participants_front[i].clone());
        }
    }

    let participants = participants::Entity::insert_many(
        to_add
            .into_iter()
            .map(|name| participants::ActiveModel {
                name: Set(name),
                draw_id: Set(draw.id),
                ..Default::default()
            })
            .collect::<Vec<participants::ActiveModel>>(),
    )
    .exec_with_returning(db)
    .await?;

    let ids = {
        let mut ids = Vec::new();
        for (i, s) in slots.iter().enumerate() {
            if s.is_none() {
                ids.push(
                    participants
                        .iter()
                        .find(|p| p.name == participants_front[i])
                        .ok_or(AppError::Internal)?
                        .id,
                );
            } else {
                ids.push(s.ok_or(AppError::Internal)?);
            }
        }
        ids
    };

    participants::Entity::delete_many()
        .filter(participants::Column::DrawId.eq(draw.id))
        .filter(participants::Column::Id.is_in(participants_back.into_values()))
        .exec(db)
        .await?;

    Ok(ids)
}

#[cfg(feature = "ssr")]
async fn update_exclusions(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
    exclusions: Exclusions,
    ids: &[i32],
) -> Result<(), AppError> {
    let back: BTreeSet<(i32, i32)> = exclusions::Entity::find()
        .filter(exclusions::Column::ParticipantA.is_in(ids))
        .all(db)
        .await?
        .into_iter()
        .map(|e| (e.participant_a, e.participant_b))
        .collect();

    let front = exclusions.into_id_pairs(ids);

    let to_add = front.difference(&back);
    let to_delete = back.difference(&front);

    exclusions::Entity::insert_many(
        to_add
            .map(|e| exclusions::ActiveModel {
                participant_a: Set(e.0),
                participant_b: Set(e.1),
                ..Default::default()
            })
            .collect::<Vec<exclusions::ActiveModel>>(),
    )
    .exec(db)
    .await?;

    exclusions::Entity::delete_many()
        .filter(
            Expr::tuple([
                Expr::col(exclusions::Column::ParticipantA),
                Expr::col(exclusions::Column::ParticipantB),
            ])
            .in_tuples(to_delete.copied()),
        )
        .exec(db)
        .await?;

    Ok(())
}

#[cfg(feature = "ssr")]
async fn draw_teams(db: &impl ConnectionTrait, draw: &draws::Model) -> Result<(), AppError> {
    let settings = team_settings::Entity::find()
        .filter(team_settings::Column::DrawId.eq(draw.id))
        .one(db)
        .await?
        .ok_or(AppError::Internal)?;

    let mode = settings.mode;
    let value = settings.value;

    Ok(())
}
