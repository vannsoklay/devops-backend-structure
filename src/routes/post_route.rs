use crate::database::mongodb::get_database;
use crate::post::{Media, Post, PostRequest, PostType};
use crate::{handler, post_service, Authentication};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .wrap(Authentication)
            .route("", web::post().to(create_post))
            .route("", web::get().to(get_posts))
            .route("/{id}", web::get().to(get_post))
            .route("/{id}", web::put().to(update_post))
            .route("/{id}", web::delete().to(delete_post)),
    );
}

fn determine_post_type(media: &Option<Vec<Media>>) -> PostType {
    match media {
        Some(media_list) if media_list.len() > 1 => PostType::Multiple,
        Some(_) | None => PostType::Single,
    }
}

async fn create_post(post: web::Json<PostRequest>, req: HttpRequest) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Post> = db.collection("posts");
    let claims = handler(req).await.expect("User not found");

    let author_id = match ObjectId::parse_str(claims.clone().sub) {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().body("Invalid author_id"),
    };
    
    let post_type = determine_post_type(&Some(post.clone().media));
    let post = Post {
        id: None,
        author_id: author_id.clone(),
        content: post.clone().content,
        media: post.clone().media,
        tag_ids: post.clone().tags,
        post_type: post_type,
        ..Default::default()
    };

    match post_service::create_post_service(&collection, post).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_posts() -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Post> = db.collection("posts");
    match post_service::get_all_posts_service(&collection).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_post(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Post> = db.collection("posts");
    match post_service::get_post_by_id_service(&collection, &id).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().body("Post not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn update_post(id: web::Path<String>, post: web::Json<PostRequest>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Post> = db.collection("posts");
    match post_service::update_post_service(&collection, &id, post.into_inner()).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn delete_post(id: web::Path<String>) -> impl Responder {
    let db = get_database().await;
    let collection: Collection<Post> = db.collection("posts");
    match post_service::delete_post_service(&collection, &id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
