use leptos::prelude::*;

use crate::server::error::AppError;

#[server]
pub async fn register(
    email: String,
    name: String,
    password: String,
    validation_password: String,
) -> Result<(), AppError> {
    use crate::entities::users;
    use crate::server::ssr::hash::hash;
    use leptos_axum::redirect;
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

    let email = email.trim();
    if email.is_empty() {
        return Err(AppError::Invalid("L'email est obligatoire".into()));
    }
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::Invalid("Le prénom est obligatoire".into()));
    }
    if password.len() < 12 {
        return Err(AppError::Invalid(
            "Le mot de passe doit contenir 12 characteres minimum".into(),
        ));
    }
    if password != validation_password {
        return Err(AppError::Invalid(
            "Les mots de passe ne sont pas identiques".into(),
        ));
    }

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    if users::Entity::find_by_email(email)
        .one(&db)
        .await?
        .is_some()
    {
        return Err(AppError::Invalid("Email already in use".into()));
    }

    let password_hash = tokio::task::spawn_blocking(move || hash(&password)).await??;
    let user = users::ActiveModel {
        password_hash: Set(password_hash),
        email: Set(email.into()),
        name: Set(name.into()),
        ..Default::default()
    };

    user.insert(&db).await?;
    redirect("/");
    Ok(())
}

#[server]
pub async fn connect(email: String, password: String) -> Result<(), ServerFnError> {
    use crate::entities::users;
    use crate::server::ssr::hash::{verify, DUMMY_HASH};
    use sea_orm::DatabaseConnection;

    let db = use_context::<DatabaseConnection>()
        .ok_or_else(|| ServerFnError::new("No DB connection"))?;

    let user = users::Entity::find_by_email(&email).one(&db).await?;
    let Some(user) = user else {
        let _ = tokio::task::spawn_blocking(move || verify("honeypot", DUMMY_HASH.as_str())).await;
        return Err(ServerFnError::new("Invalid email or password"));
    };

    match tokio::task::spawn_blocking(move || verify(&password, &user.password_hash)).await? {
        Ok(true) => Ok(()),
        Ok(false) => Err(ServerFnError::new("Invalid email or password")),
        Err(err) => Err(err.into()),
    }
}
