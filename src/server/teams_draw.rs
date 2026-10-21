#[cfg(feature = "ssr")]
use std::collections::{BTreeSet, HashMap};

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use rand::seq::SliceRandom;
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
    entities::{draws, exclusions, participants, team_settings, teams},
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
    let ids = update(&txn, &draw, participants, exclusions, team_mode, team_value).await?;
    draw_teams(&txn, &draw, ids.as_slice()).await?;
    txn.commit().await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}

#[cfg(feature = "ssr")]
async fn update(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
    participants: Participants,
    exclusions: Exclusions,
    team_mode: Option<TeamModeDto>,
    team_value: Option<i16>,
) -> Result<Vec<i32>, AppError> {
    let ids = update_participants(db, draw, participants).await?;
    update_exclusions(db, draw, exclusions, ids.as_slice()).await?;
    update_team_settings(db, draw, team_mode, team_value).await?;
    Ok(ids)
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
async fn draw_teams(
    db: &impl ConnectionTrait,
    draw: &draws::Model,
    ids: &[i32],
) -> Result<(), AppError> {
    let settings = team_settings::Entity::find()
        .filter(team_settings::Column::DrawId.eq(draw.id))
        .one(db)
        .await?
        .ok_or(AppError::Internal)?;

    let participants = draw.find_related(participants::Entity).all(db).await?;
    let exclusions = exclusions::Entity::find()
        .filter(exclusions::Column::ParticipantA.is_in(ids))
        .all(db)
        .await?;

    let mode = settings.mode;
    let value = settings.value;

    if participants.len() == 0 {
        return Err(AppError::Invalid("Liste des participants vide".into()));
    }

    if value <= 0 {
        return Err(AppError::Invalid("La value est nulle".into()));
    }

    let team_count = match mode {
        crate::entities::sea_orm_active_enums::TeamMode::Size => {
            (participants.len() / value as usize) as usize
        }
        crate::entities::sea_orm_active_enums::TeamMode::Count => value as usize,
    };

    if team_count > participants.len() {
        return Err(AppError::Invalid(
            "Plus d'équipes que de participants".into(),
        ));
    }

    let mut idx = HashMap::new();
    for (i, p) in participants.iter().enumerate() {
        idx.insert(p.id, i);
    }

    let mut excl: Vec<Vec<usize>> = vec![Vec::new(); participants.len()];

    for e in exclusions {
        let a = *idx.get(&e.participant_a).ok_or(AppError::Internal)?;
        let b = *idx.get(&e.participant_b).ok_or(AppError::Internal)?;
        excl[a].push(b);
        excl[b].push(a);
    }

    let team_of = {
        let mut rng = rand::rng();
        let mut order: Vec<usize> = (0..participants.len()).collect();
        order.shuffle(&mut rng);
        order.sort_by_key(|&i| std::cmp::Reverse(excl[i].len()));
        let caps: Vec<usize> = (0..team_count)
            .map(|t| {
                participants.len() / team_count + usize::from(t < participants.len() % team_count)
            })
            .collect();

        let mut team_of = vec![None; participants.len()];
        let mut sizes = vec![0; caps.len()];
        if !place(&order, &excl, &caps, &mut team_of, &mut sizes, &mut rng) {
            return Err(AppError::Invalid(
                "Impossible de former les équipes avec ces exclusions".into(),
            ));
        }

        team_of
    };

    teams::Entity::delete_many()
        .filter(teams::Column::DrawId.eq(draw.id))
        .exec(db)
        .await?;

    // Creer les teams en base et reccuperer les indices (l'ordre n'a pas d'inportance)
    let team_ids = teams::Entity::insert_many((0..team_count).map(|i| teams::ActiveModel {
        name: Set(format!("Équipe {}", i + 1)),
        draw_id: Set(draw.id),
        position: Set(i as i16),
        ..Default::default()
    }))
    .exec_with_returning_keys(db)
    .await?;

    // Update tous team_id des participants
    let mut members: Vec<Vec<i32>> = vec![Vec::new(); team_count];
    for (p, t) in participants.iter().zip(team_of) {
        members[t.ok_or(AppError::Internal)?].push(p.id);
    }

    for (team_id, ids) in team_ids.into_iter().zip(members) {
        participants::Entity::update_many()
            .col_expr(participants::Column::TeamId, Expr::value(team_id))
            .filter(participants::Column::Id.is_in(ids))
            .exec(db)
            .await?;
    }

    Ok(())
}

#[cfg(feature = "ssr")]
fn place(
    order: &[usize],
    excl: &[Vec<usize>],
    caps: &[usize],
    team_of: &mut [Option<usize>],
    sizes: &mut [usize],
    rng: &mut impl rand::Rng,
) -> bool {
    use rand::seq::SliceRandom;

    let [p, rest @ ..] = order else {
        return true;
    };
    let p = *p;

    let mut teams: Vec<usize> = (0..caps.len()).collect();
    teams.shuffle(rng);

    for t in teams {
        if sizes[t] == caps[t] {
            continue;
        }
        if excl[p].iter().any(|&q| team_of[q] == Some(t)) {
            continue;
        }

        team_of[p] = Some(t);
        sizes[t] += 1;

        if place(rest, excl, caps, team_of, sizes, rng) {
            return true;
        }

        team_of[p] = None;
        sizes[t] -= 1;
    }

    false
}
