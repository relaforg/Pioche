use serde::{Deserialize, Serialize};

use crate::server::{error::AppError, team_mode::TeamModeDto};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamOptions {
    pub team_mode: TeamModeDto,
    pub team_value: i16,
}

impl TeamOptions {
    pub fn new(team_mode: Option<TeamModeDto>, team_value: Option<i16>) -> Result<Self, AppError> {
        if team_mode.is_none() {
            return Err(AppError::Invalid(
                "Le mode d'équipe n'est pas rempli".into(),
            ));
        }
        Ok(Self {
            team_mode: team_mode.unwrap(),
            team_value: team_value
                .ok_or_else(|| {
                    AppError::Invalid("La taille ou le nombre d'équipe n'est pas rempli".into())
                })
                .and_then(parse_team_value)?,
        })
    }
}

fn parse_team_value(team_value: i16) -> Result<i16, AppError> {
    if team_value <= 0 {
        return Err(AppError::Invalid(
            "La taille ou nombre d'équipe doit être strictement supérieur a 0".into(),
        ));
    }

    Ok(team_value)
}

#[cfg(feature = "ssr")]
mod convert {
    use crate::{entities::team_settings, server::teams_options::TeamOptions};

    impl From<team_settings::Model> for TeamOptions {
        fn from(value: team_settings::Model) -> Self {
            Self {
                team_mode: value.mode.into(),
                team_value: value.value,
            }
        }
    }
}
