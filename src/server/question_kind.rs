use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::server::error::AppError;

pub const TSHIRT_SIZES: [&str; 7] = ["XS", "S", "M", "L", "XL", "XXL", "XXXL"];
const TEXT_MAX_CHARS: usize = 500;

// `Ord` suit l'ordre de déclaration : c'est l'ordre d'affichage des questions.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

    pub fn placeholder(self) -> &'static str {
        match self {
            QuestionKindDto::TshirtSize => "Choisir…",
            QuestionKindDto::ShoeSize => "42",
            QuestionKindDto::Allergies => "Fruits à coque, lactose… ou « aucune »",
            QuestionKindDto::Wishes => "Les livres de SF, le thé, les jeux de société…",
            QuestionKindDto::Dislikes => "Les bougies parfumées, les mugs…",
        }
    }

    /// Valide une réponse brute. `None` : pas de réponse (champ laissé vide).
    pub fn parse_answer(self, raw: &str) -> Result<Option<String>, AppError> {
        let value = raw.trim();
        if value.is_empty() {
            return Ok(None);
        }
        let invalid = |msg: &str| AppError::Invalid(format!("{} : {msg}", self.label()));

        match self {
            QuestionKindDto::TshirtSize => TSHIRT_SIZES
                .contains(&value)
                .then(|| value.to_string())
                .ok_or_else(|| invalid("taille inconnue")),
            QuestionKindDto::ShoeSize => {
                let size: f32 = value
                    .replace(',', ".")
                    .parse()
                    .map_err(|_| invalid("ce n'est pas un nombre"))?;
                if !(15.0..=55.0).contains(&size) {
                    return Err(invalid("entre 15 et 55"));
                }
                Ok(value.to_string())
            }
            QuestionKindDto::Allergies | QuestionKindDto::Wishes | QuestionKindDto::Dislikes => {
                if value.chars().count() > TEXT_MAX_CHARS {
                    return Err(invalid(&format!("{TEXT_MAX_CHARS} caractères maximum")));
                }
                Ok(value.to_string())
            }
        }
        .map(Some)
    }

    pub fn form_value(self) -> &'static str {
        match self {
            QuestionKindDto::ShoeSize => "ShoeSize",
            QuestionKindDto::Dislikes => "Dislikes",
            QuestionKindDto::TshirtSize => "TshirtSize",
            QuestionKindDto::Allergies => "Allergies",
            QuestionKindDto::Wishes => "Wishes",
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

    impl Into<QuestionKind> for QuestionKindDto {
        fn into(self) -> QuestionKind {
            match self {
                QuestionKindDto::ShoeSize => QuestionKind::ShoeSize,
                QuestionKindDto::TshirtSize => QuestionKind::TshirtSize,
                QuestionKindDto::Dislikes => QuestionKind::Dislikes,
                QuestionKindDto::Wishes => QuestionKind::Wishes,
                QuestionKindDto::Allergies => QuestionKind::Allergies,
            }
        }
    }
}
