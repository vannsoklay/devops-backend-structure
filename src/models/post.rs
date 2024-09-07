use super::{tag::TagResponse, user::UserResponse};
use crate::utils::helps::{
    deserialize_string_vec_as_object_id_vec, serialize_object_id_vec_as_string_vec,
};
use mongodb::bson::{
    oid::ObjectId,
    serde_helpers::{
        deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string,
        serialize_object_id_as_hex_string,
    },
    DateTime,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Image,
    Video,
}
// Struct to represent a media item with its URL and type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    pub url: String,           // URL or path to the media file
    pub media_type: MediaType, // Type of the media: Image or Video
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PostType {
    Single,
    Multiple,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author_id: ObjectId,
    pub content: String,
    pub media: Vec<Media>,
    #[serde(rename = "tags")]
    pub tag_ids: Vec<ObjectId>,
    pub likes_count: i32,
    pub comments_count: i32,
    #[serde(rename = "type")]
    pub post_type: PostType,
    #[serde(
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub created_at: DateTime,
    #[serde(
        deserialize_with = "deserialize_bson_datetime_from_rfc3339_string",
        serialize_with = "serialize_bson_datetime_as_rfc3339_string"
    )]
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    #[serde(rename(serialize = "id"))]
    #[serde(rename(deserialize = "_id"))]
    pub id: ObjectId,
    pub content: String,
    pub media: Option<Vec<Media>>,
    pub author: Option<UserResponse>,
    pub tags: Option<Vec<TagResponse>>,
    pub likes_count: i32,
    pub comments_count: i32,
    #[serde(rename = "type")]
    pub post_type: PostType,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            id: None,
            author_id: ObjectId::new(),
            content: String::new(),
            media: Vec::new(),
            tag_ids: Vec::new(),
            post_type: PostType::Single,
            likes_count: 0,
            comments_count: 0,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostRequest {
    pub content: String,
    pub media: Vec<Media>,
    #[serde(
        serialize_with = "serialize_object_id_vec_as_string_vec",
        deserialize_with = "deserialize_string_vec_as_object_id_vec"
    )]
    pub tags: Vec<ObjectId>,
}
