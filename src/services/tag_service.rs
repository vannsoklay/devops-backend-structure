use crate::tag::{Tag, TagRequest};
use crate::tag_repository;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;

pub async fn create_tag_service(
    collection: &Collection<Tag>,
    new_data: Tag,
) -> Result<InsertOneResult, Error> {
    tag_repository::create_tag(collection, new_data).await
}

pub async fn get_tag_by_id_service(
    collection: &Collection<Tag>,
    obj_id: &str,
) -> Result<Option<Tag>, Error> {
    tag_repository::get_tag_by_id(collection, obj_id).await
}

pub async fn get_all_tags_service(collection: &Collection<Tag>) -> Result<Vec<Tag>, Error> {
    tag_repository::get_all_tags(collection).await
}

pub async fn update_tag_service(
    collection: &Collection<Tag>,
    post_id: &str,
    updated_data: TagRequest,
) -> Result<UpdateResult, Error> {
    tag_repository::update_tag(collection, post_id, updated_data).await
}

pub async fn delete_tag_service(
    collection: &Collection<Tag>,
    obj_id: &str,
) -> Result<DeleteResult, Error> {
    tag_repository::delete_tag(collection, obj_id).await
}
