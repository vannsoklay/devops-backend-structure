use crate::models::user::User;
use crate::user_repository;
use anyhow::Result;
use mongodb::{error::Error, Collection};

// Service to get an item by username
pub async fn get_user_by_username_service(
    collection: &Collection<User>,
    username: &str,
) -> Result<Option<User>, Error> {
    user_repository::find_user_by_username(collection, username).await
}

// Service to get an item by id
pub async fn get_user_by_id_service(
    collection: &Collection<User>,
    id: &str,
) -> Result<Option<User>, Error> {
    user_repository::get_user_by_id_service(collection, id).await
}
