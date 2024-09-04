use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use actix_session::{Session, SessionMiddleware};
use actix_web::cookie::{Key, SameSite};

#[derive(serde::Deserialize)]
pub struct CookieModel {
    pub token: String
}

pub fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("d_stc"))
        .cookie_secure(true)
        .session_lifecycle(BrowserSession::default())
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private)
        .cookie_http_only(true)
        .build()
}

pub async fn set_session(
    session: Session,
    key: String,
    data: String,
) -> Result<(), Box<dyn std::error::Error>> {
    match session.insert(key, data.clone()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid to set session".into()),
    }
}
