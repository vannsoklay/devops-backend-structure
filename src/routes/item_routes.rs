use actix_web::{web, HttpResponse, Responder};
use mongodb::Collection;
use crate::database::mongodb::get_database;
use crate::models::item::Item;
use crate::services::item_service;

// Route configuration
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .route("", web::post().to(create_item))
            .route("", web::get().to(get_items))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::put().to(update_item))
            .route("/{id}", web::delete().to(delete_item))
    );
}

// Handler to create an item
async fn create_item(item: web::Json<Item>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Item> = db.collection("items");
    match item_service::create_item_service(&collection, item.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to get all items
async fn get_items() -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Item> = db.collection("items");
    match item_service::get_all_items_service(&collection).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to get an item by ID
async fn get_item(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Item> = db.collection("items");
    match item_service::get_item_by_id_service(&collection, &id).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to update an item
async fn update_item(id: web::Path<String>, item: web::Json<Item>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Item> = db.collection("items");
    match item_service::update_item_service(&collection, &id, item.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// Handler to delete an item
async fn delete_item(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Item> = db.collection("items");
    match item_service::delete_item_service(&collection, &id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
