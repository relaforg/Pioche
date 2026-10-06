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
    use crate::server::ssr::session::{append_cookie, create_session};
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

    let users::Model { id, .. } = user.insert(&db).await?;
    let cookie = create_session(&db, id).await?;
    append_cookie(cookie)?;
    redirect("/mes-tirages");
    Ok(())
}

#[server]
pub async fn connect(email: String, password: String) -> Result<(), AppError> {
    use crate::entities::users;
    use crate::server::ssr::hash::{verify, DUMMY_HASH};
    use crate::server::ssr::session::{append_cookie, create_session};
    use leptos_axum::redirect;
    use sea_orm::DatabaseConnection;

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;

    let user = users::Entity::find_by_email(email.trim()).one(&db).await?;
    let Some(user) = user else {
        let _ = tokio::task::spawn_blocking(move || verify("honeypot", DUMMY_HASH.as_str())).await;
        return Err(AppError::Invalid("Invalid email or password".into()));
    };
    let users::Model {
        id, password_hash, ..
    } = user;

    match tokio::task::spawn_blocking(move || verify(&password, &password_hash)).await? {
        Ok(true) => {
            let cookie = create_session(&db, id).await?;
            append_cookie(cookie)?;
            redirect("/mes-tirages");
            Ok(())
        }
        Ok(false) => Err(AppError::Invalid("Invalid email or password".into())),
        Err(err) => Err(err.into()),
    }
}

#[server]
pub async fn logout() -> Result<(), AppError> {
    use crate::entities::sessions;
    use crate::server::ssr::session::{append_cookie, hash_token, read_token, removal_cookie};
    use axum::http::request::Parts;
    use leptos_axum::redirect;
    use sea_orm::DatabaseConnection;

    let db = use_context::<DatabaseConnection>().ok_or(AppError::Internal)?;
    let hash = use_context::<Parts>()
        .and_then(|parts| read_token(&parts.headers))
        .and_then(|token| hash_token(&token));

    if let Some(hash) = hash {
        sessions::Entity::delete_by_token_hash(hash)
            .exec(&db)
            .await?;
    }

    let cookie = removal_cookie();
    append_cookie(cookie)?;
    redirect("/");
    Ok(())
}
