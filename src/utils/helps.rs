use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

// Serialize Vec<ObjectId> to Vec<String>
pub fn serialize_object_id_vec_as_string_vec<S>(
    tags: &Vec<ObjectId>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_tags: Vec<String> = tags.iter().map(|id| id.to_hex()).collect();
    string_tags.serialize(serializer)
}

// Deserialize Vec<String> to Vec<ObjectId>
pub fn deserialize_string_vec_as_object_id_vec<'de, D>(
    deserializer: D,
) -> Result<Vec<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_tags: Vec<String> = Vec::deserialize(deserializer)?;
    string_tags
        .into_iter()
        .map(|s| ObjectId::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}
