// src/models/user.rs

use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename(serialize = "id", deserialize = "_id"), serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    #[serde(skip_serializing)]
    pub hashed_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
