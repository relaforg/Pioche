#[cfg(feature = "ssr")]
use std::collections::HashMap;

use leptos::prelude::*;
#[cfg(feature = "ssr")]
use rand::seq::SliceRandom;
#[cfg(feature = "ssr")]
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait,
    IntoActiveModel, QueryFilter,
};

use crate::server::error::AppError;
#[cfg(feature = "ssr")]
use crate::entities::{draws, exclusions, santa_assignments};

/// Tire les binômes à partir des participants et exclusions saisis à la création.
#[server]
pub async fn draw_santa(draw_id: i32) -> Result<(), AppError> {
    use crate::entities::{participants, sea_orm_active_enums::DrawKind};
    use crate::server::{draw_kind::DrawKindDto, session::require_user};
    use sea_orm::{DatabaseConnection, ModelTrait, TransactionTrait};

    let user = require_user()?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let draw = draws::Entity::find_by_id(draw_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;

    if draw.owner_id != user.id || draw.kind != DrawKind::SecretSanta {
        leptos_axum::redirect("/");
        return Ok(());
    }

    // Les liens perso dépendent du binôme : on ne retire pas un tirage déjà fait.
    if draw.drawn_at.is_some() {
        return Err(AppError::Invalid("Le tirage a déjà été effectué".into()));
    }

    let ids: Vec<i32> = draw
        .find_related(participants::Entity)
        .all(&db)
        .await?
        .into_iter()
        .map(|p| p.id)
        .collect();

    if ids.len() < DrawKindDto::SecretSanta.min_participants() {
        return Err(AppError::Invalid("Pas assez de participants".into()));
    }

    let txn = db.begin().await?;
    save_assignments(&txn, draw, ids.as_slice()).await?;
    txn.commit().await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}

/// Passe à l'étape d'attente : les emails sont figés à partir d'ici.
/// C'est ici que l'envoi réel des liens par email viendra se brancher.
#[server]
pub async fn send_santa_links(draw_id: i32) -> Result<(), AppError> {
    use crate::entities::{participants, sea_orm_active_enums::DrawKind};
    use crate::server::session::require_user;
    use sea_orm::DatabaseConnection;
    use sea_orm::ModelTrait;

    let user = require_user()?;
    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let draw = draws::Entity::find_by_id(draw_id)
        .one(&db)
        .await?
        .ok_or(AppError::Internal)?;

    if draw.owner_id != user.id || draw.kind != DrawKind::SecretSanta {
        leptos_axum::redirect("/");
        return Ok(());
    }
    if draw.drawn_at.is_none() {
        return Err(AppError::Invalid("Lance d'abord le tirage".into()));
    }
    if draw.links_sent_at.is_some() {
        return Err(AppError::Invalid("Les liens ont déjà été envoyés".into()));
    }

    let missing = draw
        .find_related(participants::Entity)
        .all(&db)
        .await?
        .into_iter()
        .filter(|p| p.email.is_none())
        .count();
    if missing > 0 {
        return Err(AppError::Invalid(format!(
            "Il manque encore {missing} email(s)"
        )));
    }

    let mut draw = draw.into_active_model();
    draw.links_sent_at = Set(Some(chrono::Utc::now().fixed_offset()));
    draw.update(&db).await?;

    leptos_axum::redirect(&format!("/tirage/{draw_id}"));
    Ok(())
}

#[cfg(feature = "ssr")]
async fn save_assignments(
    db: &impl ConnectionTrait,
    draw: draws::Model,
    ids: &[i32],
) -> Result<(), AppError> {
    let exclusions = exclusions::Entity::find()
        .filter(exclusions::Column::ParticipantA.is_in(ids))
        .all(db)
        .await?;

    let idx: HashMap<i32, usize> = ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();

    let mut excl: Vec<Vec<usize>> = vec![Vec::new(); ids.len()];
    for e in exclusions {
        let a = *idx.get(&e.participant_a).ok_or(AppError::Internal)?;
        let b = *idx.get(&e.participant_b).ok_or(AppError::Internal)?;
        excl[a].push(b);
        excl[b].push(a);
    }

    let receiver_of = {
        let mut rng = rand::rng();
        let mut order: Vec<usize> = (0..ids.len()).collect();
        order.shuffle(&mut rng);
        // Les plus contraints d'abord : moins de retours en arrière.
        order.sort_by_key(|&i| std::cmp::Reverse(excl[i].len()));

        let mut receiver_of = vec![None; ids.len()];
        let mut taken = vec![false; ids.len()];
        if !assign(&order, &excl, &mut receiver_of, &mut taken, &mut rng) {
            return Err(AppError::Invalid(
                "Impossible de faire le tirage avec ces exclusions".into(),
            ));
        }

        receiver_of
    };

    let assignments = ids
        .iter()
        .zip(receiver_of)
        .map(|(&giver, receiver)| {
            Ok(santa_assignments::ActiveModel {
                giver_id: Set(giver),
                receiver_id: Set(ids[receiver.ok_or(AppError::Internal)?]),
                ..Default::default()
            })
        })
        .collect::<Result<Vec<_>, AppError>>()?;

    santa_assignments::Entity::delete_many()
        .filter(santa_assignments::Column::GiverId.is_in(ids))
        .exec(db)
        .await?;

    santa_assignments::Entity::insert_many(assignments)
        .exec(db)
        .await?;

    let mut draw = draw.into_active_model();
    draw.drawn_at = Set(Some(chrono::Utc::now().fixed_offset()));
    draw.update(db).await?;

    Ok(())
}

/// Attribue un destinataire à chaque donneur de `order`, par retour arrière :
/// jamais soi-même, jamais un exclu, chaque destinataire une seule fois.
#[cfg(feature = "ssr")]
fn assign(
    order: &[usize],
    excl: &[Vec<usize>],
    receiver_of: &mut [Option<usize>],
    taken: &mut [bool],
    rng: &mut impl rand::Rng,
) -> bool {
    let [giver, rest @ ..] = order else {
        return true;
    };
    let giver = *giver;

    let mut receivers: Vec<usize> = (0..taken.len()).collect();
    receivers.shuffle(rng);

    for r in receivers {
        if r == giver || taken[r] || excl[giver].contains(&r) {
            continue;
        }

        receiver_of[giver] = Some(r);
        taken[r] = true;

        if assign(rest, excl, receiver_of, taken, rng) {
            return true;
        }

        receiver_of[giver] = None;
        taken[r] = false;
    }

    false
}
