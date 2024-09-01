// src/repositories/item_repository.rs

use crate::models::item::Item;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
use mongodb::{bson::oid::ObjectId, error::Error};

// Create a new item
pub async fn create_item(
    collection: &Collection<Item>,
    new_item: Item,
) -> Result<InsertOneResult, Error> {
    let result = collection.insert_one(new_item).await?;
    Ok(result)
}

// Get an item by ID
pub async fn get_item_by_id(
    collection: &Collection<Item>,
    item_id: &str,
) -> Result<Option<Item>, Error> {
    let obj_id = ObjectId::parse_str(item_id).unwrap();
    let filter = doc! { "_id": obj_id };
    let item = collection.find_one(filter).await?;
    Ok(item)
}

// Get all items
pub async fn get_all_items(collection: &Collection<Item>) -> Result<Vec<Item>, Error> {
    let mut cursor = collection.find(doc! {}).await?;
    let mut items: Vec<Item> = Vec::new();
    while let Some(item) = cursor.try_next().await? {
        items.push(item);
    }
    Ok(items)
}

// Update an item
pub async fn update_item(
    collection: &Collection<Item>,
    item_id: &str,
    updated_item: Item,
) -> Result<UpdateResult, Error> {
    let obj_id = ObjectId::parse_str(item_id).unwrap();
    let filter = doc! { "_id": obj_id };
    let update = doc! {
        "$set": {
            "name": updated_item.name,
            "description": updated_item.description,
            "price": updated_item.price,
            "stock": updated_item.stock,
        }
    };
    let result = collection.update_one(filter, update).await?;
    Ok(result)
}

// Delete an item
pub async fn delete_item(
    collection: &Collection<Item>,
    item_id: &str,
) -> Result<DeleteResult, Error> {
    let obj_id = ObjectId::parse_str(item_id).unwrap();
    let filter = doc! { "_id": obj_id };
    let result = collection.delete_one(filter).await?;
    Ok(result)
}
