use crate::models::user::User;
use crate::user_repository;
use anyhow::Result;
use mongodb::{error::Error, Collection};

// Service to get an item by id
pub async fn get_item_by_id_service(
    collection: &Collection<User>,
    username: &str,
) -> Result<Option<User>, Error> {
    user_repository::find_user_by_username(collection, username).await
}
