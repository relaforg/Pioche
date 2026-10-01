use std::fmt;

use leptos::server_fn::{
    codec::JsonEncoding,
    error::{FromServerFnError, ServerFnErrorErr},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    Invalid(String),
    Internal,
    ServerFn(ServerFnErrorErr),
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::ServerFn(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Invalid(msg) => write!(f, "{msg}"),
            AppError::Internal => write!(f, "Oups, une erreur est survenue"),
            AppError::ServerFn(_) => write!(f, "Impossible de joindre le serveur"),
        }
    }
}

#[cfg(feature = "ssr")]
impl<E: std::error::Error> From<E> for AppError {
    fn from(value: E) -> Self {
        leptos::logging::error!("{value}");
        AppError::Internal
    }
}
