use mongodb::bson::{
    oid::ObjectId,
    serde_helpers::{
        deserialize_bson_datetime_from_rfc3339_string, serialize_bson_datetime_as_rfc3339_string, serialize_object_id_as_hex_string
    },
    DateTime,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "id"))]
    #[serde(rename(deserialize = "_id"))]
    pub id: Option<ObjectId>,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub owner_id: ObjectId,
    pub name: String,
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

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: None,
            owner_id: ObjectId::new(),
            name: String::new(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Tag {
    pub fn to_tag(tag: Tag) -> TagResponse {
        TagResponse {
            _id: tag.clone().id.unwrap(),
            name: tag.clone().name,
        }
    }
}

#[derive(Debug , Deserialize, Serialize)]
pub struct TagRequest {
    pub name: String,
}

#[derive(Debug , Deserialize, Serialize, Clone)]
pub struct TagResponse {
    #[serde(rename(serialize = "id"))]
    #[serde(rename(deserialize = "_id"))]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
}