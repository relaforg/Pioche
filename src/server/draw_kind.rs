use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DrawKindDto {
    SecretSanta,
    Teams,
}

impl DrawKindDto {
    pub fn form_value(self) -> &'static str {
        match self {
            DrawKindDto::SecretSanta => "SecretSanta",
            DrawKindDto::Teams => "Teams",
        }
    }

    pub fn min_participants(self) -> usize {
        match self {
            DrawKindDto::SecretSanta => 3,
            DrawKindDto::Teams => 2,
        }
    }
}

#[cfg(feature = "ssr")]
mod convert {
    use crate::{entities::sea_orm_active_enums, server::draw_kind::DrawKindDto};

    impl From<sea_orm_active_enums::DrawKind> for DrawKindDto {
        fn from(value: sea_orm_active_enums::DrawKind) -> Self {
            match value {
                sea_orm_active_enums::DrawKind::SecretSanta => DrawKindDto::SecretSanta,
                sea_orm_active_enums::DrawKind::Teams => DrawKindDto::Teams,
            }
        }
    }
}
