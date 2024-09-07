use crate::database::mongodb::get_database;
use crate::tag::{Tag, TagRequest};
use crate::{handler, tag_repository, tag_service, Authentication};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tags")
            .wrap(Authentication)
            .route("", web::post().to(create_tag))
            .route("", web::get().to(get_tags))
            .route("/{id}", web::get().to(get_tag))
            .route("/{id}", web::put().to(update_tag))
            .route("/{id}", web::delete().to(delete_tag)),
    );
}

async fn create_tag(tag: web::Json<TagRequest>, req: HttpRequest) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Tag> = db.collection("tags");
    let claims = handler(req).await.expect("Tag not found");

    let owner_id = match ObjectId::parse_str(claims.clone().sub) {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().body("Invalid author_id"),
    };

    let post = Tag {
        id: None,
        owner_id: owner_id.clone(),
        name: tag.name.to_owned(),
        ..Default::default()
    };

    match tag_repository::create_tag(&collection, post).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_tags() -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Tag> = db.collection("tags");
    match tag_service::get_all_tags_service(&collection).await {
        Ok(tags) => HttpResponse::Ok().json(tags),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_tag(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Tag> = db.collection("tags");
    match tag_service::get_tag_by_id_service(&collection, &id).await {
        Ok(Some(tag)) => HttpResponse::Ok().json(tag),
        Ok(None) => HttpResponse::NotFound().body("TAG not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn update_tag(id: web::Path<String>, tag: web::Json<TagRequest>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Tag> = db.collection("tags");
    match tag_service::update_tag_service(&collection, &id, tag.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn delete_tag(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Tag> = db.collection("tags");
    match tag_repository::delete_tag(&collection, &id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
