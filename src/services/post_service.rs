use crate::post::{Post, PostRequest, PostResponse};
use crate::repositories::post_repository;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;

pub async fn create_post_service(
    collection: &Collection<Post>,
    new_post: Post,
) -> Result<InsertOneResult, Error> {
    post_repository::create_post(collection, new_post).await
}

pub async fn get_post_by_id_service(
    collection: &Collection<Post>,
    post_id: &str,
) -> Result<Option<Post>, Error> {
    post_repository::get_post_by_id(collection, post_id).await
}

pub async fn get_all_posts_service(collection: &Collection<Post>) -> Result<Vec<PostResponse>, Error> {
    post_repository::get_all_posts(collection).await
}

pub async fn update_post_service(
    collection: &Collection<Post>,
    post_id: &str,
    updated_post: PostRequest,
) -> Result<UpdateResult, Error> {
    post_repository::update_post(collection, post_id, updated_post).await
}

pub async fn delete_post_service(
    collection: &Collection<Post>,
    post_id: &str,
) -> Result<DeleteResult, Error> {
    post_repository::delete_post(collection, post_id).await
}
