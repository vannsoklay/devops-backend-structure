
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use mongodb::Collection;
use crate::database::mongodb::get_database;
use crate::models::user::{RegisterRequest, LoginRequest};
use crate::services::auth_service;
use crate::user::User;

// Route configuration
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    );
}

// Register a new user
async fn register(register_req: web::Json<RegisterRequest>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<User> = db.collection("users");
    match auth_service::register_user_service(&collection, register_req.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Login and return JWT token
async fn login(login_req: web::Json<LoginRequest>, session: Session) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<User> = db.collection("users");
    match auth_service::login_user_service(&collection, login_req.into_inner(), session).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}
