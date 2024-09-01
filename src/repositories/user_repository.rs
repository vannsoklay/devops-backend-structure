use crate::models::user::User;
use mongodb::{bson::doc, results::InsertOneResult, Collection, error::Error};

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
