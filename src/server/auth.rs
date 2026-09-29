use leptos::prelude::*;

#[server]
pub async fn register(email: String, password: String) -> Result<(), ServerFnError> {
    use crate::entities::users;
    use crate::server::ssr::hash::hash;
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

    let db = use_context::<DatabaseConnection>()
        .ok_or_else(|| ServerFnError::new("No DB connection"))?;

    if users::Entity::find_by_email(&email)
        .one(&db)
        .await?
        .is_some()
    {
        return Err(ServerFnError::new("Email already in use"));
    }

    let password_hash = tokio::task::spawn_blocking(move || hash(&password)).await??;
    let user = users::ActiveModel {
        password_hash: Set(password_hash),
        email: Set(email),
        ..Default::default()
    };

    user.insert(&db).await?;
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
