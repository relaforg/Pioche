use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, EnumIter)]
pub enum QuestionKindDto {
    TshirtSize,
    ShoeSize,
    Allergies,
    Wishes,
    Dislikes,
}

impl QuestionKindDto {
    pub fn label(self) -> &'static str {
        match self {
            QuestionKindDto::ShoeSize => "Pointure",
            QuestionKindDto::Dislikes => "Ce que je n'aime pas",
            QuestionKindDto::TshirtSize => "Taille t-shirt",
            QuestionKindDto::Allergies => "Allergies",
            QuestionKindDto::Wishes => "Ce que j'aime",
        }
    }

    pub fn form_value(self) -> &'static str {
        match self {
            QuestionKindDto::ShoeSize => "shoe_size",
            QuestionKindDto::Dislikes => "dislikes",
            QuestionKindDto::TshirtSize => "tshirt_size",
            QuestionKindDto::Allergies => "allergies",
            QuestionKindDto::Wishes => "likes",
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use crate::{
        entities::sea_orm_active_enums::QuestionKind, server::question_kind::QuestionKindDto,
    };

    impl From<QuestionKind> for QuestionKindDto {
        fn from(value: QuestionKind) -> Self {
            match value {
                QuestionKind::Allergies => QuestionKindDto::Allergies,
                QuestionKind::ShoeSize => QuestionKindDto::ShoeSize,
                QuestionKind::Wishes => QuestionKindDto::Wishes,
                QuestionKind::TshirtSize => QuestionKindDto::TshirtSize,
                QuestionKind::Dislikes => QuestionKindDto::Dislikes,
            }
        }
    }
}
