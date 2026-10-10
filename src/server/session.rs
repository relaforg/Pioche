use serde::{Deserialize, Serialize};

use crate::server::error::AppError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[cfg(feature = "ssr")]
pub fn current_user() -> Option<UserDto> {
    use axum::http::request::Parts;
    use leptos::context::use_context;

    use_context::<Parts>()?
        .extensions
        .get::<UserDto>()
        .cloned()
}

#[cfg(not(feature = "ssr"))]
pub fn current_user() -> Option<UserDto> {
    None
}

pub fn require_user() -> Result<UserDto, AppError> {
    current_user().ok_or(AppError::Unauthorized)
}
