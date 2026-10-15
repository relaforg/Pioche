use crate::views::colors::Color;

pub mod header;
pub mod secret_santa;
pub mod teams;

/// Couleurs appliquées à tour de rôle aux équipes et aux avatars.
pub const ACCENT_COLORS: [Color; 3] = [Color::Butter, Color::Blue, Color::Raspberry];
