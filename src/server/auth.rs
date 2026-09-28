use argon2::{password_hash, Argon2, PasswordHasher, PasswordVerifier};
use leptos::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::entities::users;

fn hash(password: String) -> Result<String, password_hash::Error> {
    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    Ok(hash)
}

fn verify(password: String, hash: String) -> Result<bool, password_hash::Error> {
    match Argon2::default().verify_password(password.as_bytes(), hash.as_str()) {
        Ok(_) => Ok(true),
        Err(password_hash::Error::PasswordInvalid) => Ok(false),
        Err(err) => Err(err),
    }
}

#[server]
pub async fn register(email: String, password: String) -> Result<(), ServerFnError> {
    let db = use_context::<DatabaseConnection>()
        .ok_or_else(|| ServerFnError::new("No DB connection"))?;

    if users::Entity::find_by_email(&email).one(&db).await?.is_some() {
        return Err(ServerFnError::new("Email already in use"));
    }

    let password_hash = tokio::task::spawn_blocking(move || hash(password)).await??;
    let user = users::ActiveModel {
        password_hash: Set(password_hash),
        email: Set(email),
        ..Default::default()};

    user.insert(&db).await?;
    Ok(())
}
