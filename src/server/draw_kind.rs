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
    use crate::{entities::sea_orm_active_enums::DrawKind, server::draw_kind::DrawKindDto};

    impl From<DrawKind> for DrawKindDto {
        fn from(value: DrawKind) -> Self {
            match value {
                DrawKind::SecretSanta => DrawKindDto::SecretSanta,
                DrawKind::Teams => DrawKindDto::Teams,
            }
        }
    }

    impl Into<DrawKind> for DrawKindDto {
        fn into(self) -> DrawKind {
            match self {
                DrawKindDto::Teams => DrawKind::Teams,
                DrawKindDto::SecretSanta => DrawKind::SecretSanta,
            }
        }
    }
}
