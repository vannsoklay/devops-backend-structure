use crate::models::user::User;
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, results::InsertOneResult, Collection};

// Create a new user
pub async fn create_user(collection: &Collection<User>, new_user: User) -> Result<InsertOneResult, Error> {
    let result = collection.insert_one(new_user).await?;
    Ok(result)
}

// Find a user by username
pub async fn find_user_by_username(
    collection: &Collection<User>,
    username: &str,
) -> mongodb::error::Result<Option<User>> {
    let filter = doc! { "username": username };
    collection.find_one(filter).await
}

// Find a user by id
pub async fn get_user_by_id_service(
    collection: &Collection<User>,
    id: &str,
) -> mongodb::error::Result<Option<User>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    collection.find_one(filter).await
}