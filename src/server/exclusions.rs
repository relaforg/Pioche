use std::{cmp::Ordering, collections::BTreeSet};

use serde::{Deserialize, Serialize};

use crate::server::{error::AppError, participants::Participants};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Exclusion {
    a: usize,
    b: usize,
}

impl Exclusion {
    pub fn new(i: usize, j: usize) -> Option<Self> {
        match i.cmp(&j) {
            Ordering::Less => Some(Self { a: i, b: j }),
            Ordering::Greater => Some(Self { a: j, b: i }),
            Ordering::Equal => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExclusionInput {
    pub a: String,
    pub b: String,
}

pub struct Exclusions(BTreeSet<Exclusion>);

impl Exclusions {
    pub fn parse(raw: Vec<ExclusionInput>, participants: &Participants) -> Result<Self, AppError> {
        let mut set = BTreeSet::new();

        for ExclusionInput { a, b } in raw {
            let i = find(participants, &a)?;
            let j = find(participants, &b)?;

            let exclusion = Exclusion::new(i, j).ok_or_else(|| {
                AppError::Invalid(
                    "Une exclusion doit concerner deux participants différents".into(),
                )
            })?;

            if !set.insert(exclusion) {
                return Err(AppError::Invalid(format!(
                    "'{a}' et '{b}' sont déjà exclus l'un de l'autre"
                )));
            }
        }
        Ok(Exclusions(set))
    }
}

fn find(participants: &Participants, name: &str) -> Result<usize, AppError> {
    participants.position(name).ok_or_else(|| {
        AppError::Invalid(format!("'{name}' n'est pas dans la liste des participants"))
    })
}

#[cfg(feature = "ssr")]
mod convert {
    use sea_orm::ActiveValue::Set;

    use crate::{
        entities::exclusions,
        server::exclusions::{Exclusion, Exclusions},
    };

    impl Exclusions {
        pub fn into_active_models(self, ids: &[i32]) -> Vec<exclusions::ActiveModel> {
            self.0
                .into_iter()
                .map(|Exclusion { a, b }| {
                    let (x, y) = (ids[a], ids[b]);
                    exclusions::ActiveModel {
                        participant_a: Set(x.min(y)),
                        participant_b: Set(x.max(y)),
                        ..Default::default()
                    }
                })
                .collect()
        }
    }
}
