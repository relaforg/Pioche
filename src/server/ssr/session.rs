use axum::{
    extract::{Request, State},
    http::{
        header::{COOKIE, SET_COOKIE},
        HeaderMap, HeaderValue,
    },
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, TimeDelta, Utc};
use cookie::{time, Cookie, SameSite};
use hex::FromHex;
use leptos::context::use_context;
use leptos_axum::ResponseOptions;
use rand::Rng;
use sea_orm::{
    entity::prelude::DateTimeWithTimeZone, ActiveModelTrait, ActiveValue::Set, ColumnTrait,
    DatabaseConnection, DbErr, IntoActiveModel, QueryFilter,
};
use sha2::{Digest, Sha256};

use crate::{
    entities::{sessions, users},
    server::{error::AppError, session::CurrentUser},
};

pub const COOKIE_NAME: &str = "pioche_session";
pub const SESSION_TTL: TimeDelta = TimeDelta::days(30);
pub const REFRESH_AFTER: TimeDelta = TimeDelta::days(1);

enum CookieAction {
    Keep,
    Refresh(String),
    Clear,
}

fn digest(bytes: &[u8]) -> Vec<u8> {
    Sha256::digest(bytes).to_vec()
}

pub fn generate_token() -> (String, Vec<u8>) {
    let mut buf = [0; 32];
    rand::rng().fill_bytes(&mut buf);
    let enc = hex::encode(buf);
    (enc, digest(&buf))
}

pub fn hash_token(token: &str) -> Option<Vec<u8>> {
    if token.len() != 64 {
        return None;
    }
    let Some(dec) = <[u8; 32]>::from_hex(token).ok() else {
        return None;
    };
    Some(digest(&dec))
}

pub fn session_cookie(token: String) -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, token))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .secure(!cfg!(debug_assertions))
        .max_age(time::Duration::seconds(SESSION_TTL.num_seconds()))
        .into()
}

pub fn removal_cookie() -> Cookie<'static> {
    Cookie::build((COOKIE_NAME, "")).path("/").removal().into()
}

pub fn read_token(headers: &HeaderMap) -> Option<String> {
    for value in headers.get_all(COOKIE) {
        let Ok(s) = value.to_str() else {
            continue;
        };
        for result in Cookie::split_parse(s) {
            if let Ok(cookie) = result {
                if cookie.name() == COOKIE_NAME {
                    return Some(cookie.value().to_string());
                }
            }
        }
    }
    None
}

async fn find_session(
    db: &DatabaseConnection,
    hash: Vec<u8>,
) -> Result<Option<(sessions::Model, Option<users::Model>)>, DbErr> {
    sessions::Entity::find_by_token_hash(hash)
        .filter(sessions::Column::ExpiresAt.gt(Utc::now()))
        .find_also_related(users::Entity)
        .one(db)
        .await
}

pub async fn session_middleware(
    State(db): State<DatabaseConnection>,
    mut req: Request,
    next: Next,
) -> Response {
    let Some(token) = read_token(req.headers()) else {
        return next.run(req).await;
    };

    let (user, action) = resolve(&db, token).await;
    if let Some(user) = user {
        req.extensions_mut().insert(user);
    }
    let mut res = next.run(req).await;
    apply_cookie_action(&mut res, action);
    res
}

fn needs_refresh(expires_at: DateTimeWithTimeZone, now: DateTime<Utc>) -> bool {
    expires_at < now + SESSION_TTL - REFRESH_AFTER
}

async fn resolve(db: &DatabaseConnection, token: String) -> (Option<CurrentUser>, CookieAction) {
    let Some(hash) = hash_token(&token) else {
        return (None, CookieAction::Clear);
    };

    match find_session(db, hash).await {
        Ok(Some((session, Some(user)))) => {
            let current_user = CurrentUser {
                id: user.id,
                name: user.name,
                email: user.email,
            };
            let now = Utc::now();

            let action = if needs_refresh(session.expires_at, now) {
                let mut active = session.into_active_model();
                active.expires_at = Set((now + SESSION_TTL).into());

                match active.update(db).await {
                    Ok(_) => CookieAction::Refresh(token),
                    Err(e) => {
                        leptos::logging::error!("{e}");
                        CookieAction::Keep
                    }
                }
            } else {
                CookieAction::Keep
            };

            (Some(current_user), action)
        }
        Ok(None) | Ok(Some((_, None))) => (None, CookieAction::Clear),
        Err(e) => {
            leptos::logging::error!("{e}");
            (None, CookieAction::Keep)
        }
    }
}

fn sets_session_cookie(res: &Response) -> bool {
    for value in res.headers().get_all(SET_COOKIE) {
        let Ok(s) = value.to_str() else {
            continue;
        };
        let Ok(cookie) = Cookie::parse(s) else {
            continue;
        };
        if cookie.name() == COOKIE_NAME {
            return true;
        }
    }
    false
}

fn apply_cookie_action(res: &mut Response, action: CookieAction) {
    if sets_session_cookie(res) {
        return;
    }

    let cookie = match action {
        CookieAction::Keep => return,
        CookieAction::Refresh(token) => session_cookie(token),
        CookieAction::Clear => removal_cookie(),
    };

    let Ok(value) = HeaderValue::from_str(&cookie.to_string()) else {
        leptos::logging::error!("session cookie is not a valis header value");
        return;
    };
    res.headers_mut().append(SET_COOKIE, value);
}

pub async fn create_session(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Cookie<'static>, DbErr> {
    let (token, hash) = generate_token();

    let session = sessions::ActiveModel {
        expires_at: Set((Utc::now() + SESSION_TTL).into()),
        user_id: Set(user_id),
        token_hash: Set(hash),
        ..Default::default()
    };
    session.insert(db).await?;
    Ok(session_cookie(token))
}

pub fn append_cookie(cookie: Cookie<'_>) -> Result<(), AppError> {
    let response = use_context::<ResponseOptions>().ok_or(AppError::Internal)?;
    let value = HeaderValue::from_str(&cookie.to_string())?;
    response.append_header(SET_COOKIE, value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_token_matches_generated_hash() {
        let (token, hash) = generate_token();
        assert_eq!(hash_token(&token), Some(hash));
    }

    #[test]
    fn generated_tokens_are_unique() {
        let (a, _) = generate_token();
        let (b, _) = generate_token();
        assert_ne!(a, b);
    }

    #[test]
    fn hash_token_rejects_invalid_hex() {
        // Bonne longueur (64 caractères) : seul le décodage hex peut échouer.
        assert_eq!(hash_token(&"z".repeat(64)), None);
    }

    #[test]
    fn hash_token_rejects_wrong_length() {
        assert_eq!(hash_token("abcd"), None);
        assert_eq!(hash_token(&"ab".repeat(33)), None);
    }

    fn cookie_headers(lines: &[&str]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for line in lines {
            headers.append(COOKIE, HeaderValue::from_str(line).unwrap());
        }
        headers
    }

    #[test]
    fn read_token_finds_session_cookie_alone() {
        let headers = cookie_headers(&["pioche_session=abc"]);
        assert_eq!(read_token(&headers), Some("abc".into()));
    }

    #[test]
    fn read_token_finds_session_cookie_among_others() {
        let headers = cookie_headers(&["theme=dark; pioche_session=abc; lang=fr"]);
        assert_eq!(read_token(&headers), Some("abc".into()));
    }

    #[test]
    fn read_token_searches_every_cookie_header() {
        let headers = cookie_headers(&["theme=dark", "pioche_session=abc"]);
        assert_eq!(read_token(&headers), Some("abc".into()));
    }

    #[test]
    fn read_token_skips_non_ascii_header() {
        let mut headers = HeaderMap::new();
        headers.append(COOKIE, HeaderValue::from_bytes(&[0xff]).unwrap());
        headers.append(COOKIE, HeaderValue::from_static("pioche_session=abc"));
        assert_eq!(read_token(&headers), Some("abc".into()));
    }

    #[test]
    fn read_token_returns_none_without_session_cookie() {
        assert_eq!(read_token(&HeaderMap::new()), None);
        assert_eq!(read_token(&cookie_headers(&["theme=dark; lang=fr"])), None);
    }

    use axum::body::Body;
    use chrono::TimeZone;

    fn now() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 10, 1, 12, 0, 0).unwrap()
    }

    // Le seuil : une session rafraîchie il y a exactement REFRESH_AFTER.
    fn threshold() -> DateTimeWithTimeZone {
        (now() + SESSION_TTL - REFRESH_AFTER).fixed_offset()
    }

    #[test]
    fn needs_refresh_false_for_fresh_session() {
        let expires_at = (now() + SESSION_TTL).fixed_offset();
        assert!(!needs_refresh(expires_at, now()));
    }

    #[test]
    fn needs_refresh_false_just_before_threshold() {
        let expires_at = threshold() + TimeDelta::seconds(1);
        assert!(!needs_refresh(expires_at, now()));
    }

    #[test]
    fn needs_refresh_true_just_after_threshold() {
        let expires_at = threshold() - TimeDelta::seconds(1);
        assert!(needs_refresh(expires_at, now()));
    }

    fn response_with_set_cookies(lines: &[&str]) -> Response {
        let mut res = Response::new(Body::empty());
        for line in lines {
            res.headers_mut()
                .append(SET_COOKIE, HeaderValue::from_str(line).unwrap());
        }
        res
    }

    fn set_cookies(res: &Response) -> Vec<Cookie<'static>> {
        res.headers()
            .get_all(SET_COOKIE)
            .iter()
            .map(|v| Cookie::parse(v.to_str().unwrap().to_owned()).unwrap())
            .collect()
    }

    #[test]
    fn sets_session_cookie_detects_our_cookie_with_attributes() {
        let res = response_with_set_cookies(&["pioche_session=abc; Path=/; HttpOnly"]);
        assert!(sets_session_cookie(&res));
    }

    #[test]
    fn sets_session_cookie_ignores_other_cookies() {
        let res = response_with_set_cookies(&["theme=dark; Path=/"]);
        assert!(!sets_session_cookie(&res));
        assert!(!sets_session_cookie(&Response::new(Body::empty())));
    }

    #[test]
    fn apply_keep_adds_nothing() {
        let mut res = Response::new(Body::empty());
        apply_cookie_action(&mut res, CookieAction::Keep);
        assert!(set_cookies(&res).is_empty());
    }

    #[test]
    fn apply_refresh_sets_session_cookie() {
        let mut res = Response::new(Body::empty());
        apply_cookie_action(&mut res, CookieAction::Refresh("abc".into()));

        let cookies = set_cookies(&res);
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].name(), COOKIE_NAME);
        assert_eq!(cookies[0].value(), "abc");
        assert_eq!(cookies[0].http_only(), Some(true));
        assert_eq!(cookies[0].path(), Some("/"));
        assert_eq!(
            cookies[0].max_age(),
            Some(time::Duration::seconds(SESSION_TTL.num_seconds()))
        );
    }

    #[test]
    fn apply_clear_sets_removal_cookie() {
        let mut res = Response::new(Body::empty());
        apply_cookie_action(&mut res, CookieAction::Clear);

        let cookies = set_cookies(&res);
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].value(), "");
        assert_eq!(cookies[0].path(), Some("/"));
        assert_eq!(cookies[0].max_age(), Some(time::Duration::ZERO));
    }

    #[test]
    fn apply_does_not_override_handler_cookie() {
        // Ex. : logout a déjà posé un cookie de suppression.
        let mut res = response_with_set_cookies(&["pioche_session=; Path=/; Max-Age=0"]);
        apply_cookie_action(&mut res, CookieAction::Refresh("abc".into()));

        let cookies = set_cookies(&res);
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].value(), "");
    }
}
