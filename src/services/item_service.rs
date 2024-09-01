use crate::models::item::Item;
use crate::repositories::item_repository;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::Collection;

// Service to create an item
pub async fn create_item_service(
    collection: &Collection<Item>,
    new_item: Item,
) -> Result<InsertOneResult, Error> {
    item_repository::create_item(collection, new_item).await
}

// Service to get an item by id
pub async fn get_item_by_id_service(
    collection: &Collection<Item>,
    item_id: &str,
) -> Result<Option<Item>, Error> {
    item_repository::get_item_by_id(collection, item_id).await
}

// Service to get all items
pub async fn get_all_items_service(collection: &Collection<Item>) -> Result<Vec<Item>, Error> {
    item_repository::get_all_items(collection).await
}

// Service to update an item
pub async fn update_item_service(
    collection: &Collection<Item>,
    item_id: &str,
    updated_item: Item,
) -> Result<UpdateResult, Error> {
    item_repository::update_item(collection, item_id, updated_item).await
}

// Service to delete an item
pub async fn delete_item_service(
    collection: &Collection<Item>,
    item_id: &str,
) -> Result<DeleteResult, Error> {
    item_repository::delete_item(collection, item_id).await
}
