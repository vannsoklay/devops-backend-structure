use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::Collection;

use crate::{get_database, handler, user::User, user_service, Authentication};

// Function to configure user routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").wrap(Authentication).route("/me", web::get().to(get_user)));
}

// Handler to get an user
async fn get_user(req: HttpRequest) -> impl Responder {
    let db = get_database().await;
    let claims = handler(req).await.expect("User not found");

    let collection: Collection<User> = db.collection("users");
    match user_service::get_user_by_id_service(&collection, &claims.sub).await {
        Ok(Some(user)) => HttpResponse::Ok().json(User::to_user(user)),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
