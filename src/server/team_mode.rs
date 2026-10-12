use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TeamModeDto {
    Size,
    Count,
}

impl TeamModeDto {
    pub fn form_value(self) -> &'static str {
        match self {
            TeamModeDto::Size => "Size",
            TeamModeDto::Count => "Count",
        }
    }
}

#[cfg(feature = "ssr")]
mod convert {
    use crate::{entities::sea_orm_active_enums::TeamMode, server::team_mode::TeamModeDto};

    impl From<TeamMode> for TeamModeDto {
        fn from(value: TeamMode) -> Self {
            match value {
                TeamMode::Size => Self::Size,
                TeamMode::Count => Self::Count,
            }
        }
    }
}
