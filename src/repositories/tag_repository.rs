use crate::tag::{Tag, TagRequest};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
use mongodb::{bson::oid::ObjectId, error::Error};

pub async fn create_tag(
    collection: &Collection<Tag>,
    new_tag: Tag,
) -> Result<InsertOneResult, Error> {
    let result = collection.insert_one(new_tag).await?;
    Ok(result)
}

pub async fn get_tag_by_id(
    collection: &Collection<Tag>,
    tag_id: &str,
) -> Result<Option<Tag>, Error> {
    let obj_id = match ObjectId::parse_str(tag_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid TAG ID")),
    };

    let filter = doc! { "_id": obj_id };
    let tag = collection.find_one(filter).await?;
    Ok(tag)
}

pub async fn get_all_tags(collection: &Collection<Tag>) -> Result<Vec<Tag>, Error> {
    let mut cursor = collection.find(doc! {}).await?;
    let mut tags: Vec<Tag> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        tags.push(doc);
    }
    Ok(tags)
}

pub async fn update_tag(
    collection: &Collection<Tag>,
    tag_id: &str,
    updated_post: TagRequest,
) -> Result<UpdateResult, Error> {
    let obj_id = match ObjectId::parse_str(tag_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid Post ID")),
    };

    let filter = doc! { "_id": obj_id };
    let update = doc! {
        "$set": {
            "name": updated_post.name
        }
    };
    let result = collection.update_one(filter, update).await?;
    Ok(result)
}

pub async fn delete_tag(collection: &Collection<Tag>, tag_id: &str) -> Result<DeleteResult, Error> {
    let obj_id = match ObjectId::parse_str(tag_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid Post ID")),
    };
    let filter = doc! { "_id": obj_id };
    let result = collection.delete_one(filter).await?;
    Ok(result)
}
