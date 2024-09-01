use actix_web::{web, HttpResponse, Responder};
use mongodb::Collection;

use crate::{get_database, user::User, user_service};

// Function to configure user routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("", web::get().to(get_user)));
}

// Handler to get an user
async fn get_user(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<User> = db.collection("users");
    match user_service::get_item_by_id_service(&collection, &id).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
