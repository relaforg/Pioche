use std::sync::LazyLock;

use argon2::{password_hash, Argon2, PasswordHasher, PasswordVerifier};
use leptos::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

use crate::entities::users;

static DUMMY_HASH: LazyLock<String> = LazyLock::new(|| hash("dummy".into()).unwrap());

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

#[server]
pub async fn connect(email: String, password: String) -> Result<(), ServerFnError> {
    let db = use_context::<DatabaseConnection>()
        .ok_or_else(|| ServerFnError::new("No DB connection"))?;

    let user = users::Entity::find_by_email(&email).one(&db).await?;
    let Some(user) = user else {
        let _ = tokio::task::spawn_blocking(move || verify("honeypot".to_string(), DUMMY_HASH.clone())).await;
        return Err(ServerFnError::new("Invalid email or password"))
    };
    
    match tokio::task::spawn_blocking(move || verify(password, user.password_hash)).await? {
        Ok(true) => Ok(()),
        Ok(false) => Err(ServerFnError::new("Invalid email or password")),
        Err(err) => Err(err.into())
    }
}
