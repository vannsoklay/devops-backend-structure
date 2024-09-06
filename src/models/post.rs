use mongodb::bson::{
    oid::ObjectId,
    serde_helpers::{
        deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string,
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
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author_id: ObjectId,
    pub content: String,
    pub media: Vec<Media>,
    pub tags: Vec<String>,
    pub likes_count: i32,
    pub comments_count: i32,
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

impl Default for Post {
    fn default() -> Self {
        Post {
            id: None,
            author_id: ObjectId::new(),
            content: String::new(),
            media: Vec::new(),
            tags: Vec::new(),
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
    pub tags: Vec<String>,
}
