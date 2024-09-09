use futures::stream;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    tag::{Tag, TagResponse},
    user::{User, UserResponse},
};
use crate::{
    get_database,
    utils::helps::{
        deserialize_string_vec_as_object_id_vec, serialize_object_id_vec_as_string_vec,
    },
};
use mongodb::{
    bson::{
        doc,
        oid::ObjectId,
        serde_helpers::{
            deserialize_bson_datetime_from_rfc3339_string,
            serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string,
        },
        DateTime,
    },
    Collection,
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

impl Post {
    pub async fn to_post(data: Option<Post>) -> Option<PostResponse> {
        match data {
            Some(d) => Some(PostResponse {
                id: d.id.unwrap(),
                content: d.content,
                media: Some(d.media),
                author: Self::author(d.author_id).await,
                tags: Self::tags(d.tag_ids).await,
                likes_count: d.likes_count,
                comments_count: d.comments_count,
                post_type: d.post_type,
                created_at: d.created_at.to_string(),
                updated_at: d.updated_at.to_string(),
            }),
            None => None,
        }
    }

    async fn author(author_id: ObjectId) -> Option<UserResponse> {
        let db = get_database().await;
        let collection: Collection<User> = db.collection("users");

        let user = collection.find_one(doc! { "_id": author_id }).await;
        if user.is_err() {
            return None;
        }
        match user.unwrap() {
            Some(u) => Some(User::to_user(u)),
            None => None,
        }
    }

    async fn tags(tag_ids: Vec<ObjectId>) -> Option<Vec<TagResponse>> {
        let db = get_database().await; // Ensure `get_database` returns the correct connection
        let collection: Collection<Tag> = db.collection("tags"); // Correct the collection name to "tags"

        // Shared, thread-safe vector to accumulate TagResponses
        let tags = Arc::new(Mutex::new(vec![]));

        stream::iter(tag_ids)
            .then(|tag_id| {
                let tags = Arc::clone(&tags);
                {
                    let value = collection.clone();
                    async move {
                        match value.find_one(doc! { "_id": tag_id }).await {
                            Ok(Some(tag)) => {
                                let mut tags = tags.lock().await;
                                tags.push(Tag::to_tag(tag));
                            }
                            Ok(None) | Err(_) => {} // Handle not found and error cases
                        }
                    }
                }
            })
            .collect::<Vec<()>>() // Collect into a Vec of () just to drive the stream
            .await;

        let tags = Arc::try_unwrap(tags).ok().unwrap().into_inner();
        Some(tags)
    }
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