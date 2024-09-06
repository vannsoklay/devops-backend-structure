use crate::models::post::Post;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};
use mongodb::{bson::oid::ObjectId, error::Error};

pub async fn create_post(
    collection: &Collection<Post>,
    new_post: Post,
) -> Result<InsertOneResult, Error> {
    let result = collection.insert_one(new_post).await?;
    Ok(result)
}

pub async fn get_post_by_id(
    collection: &Collection<Post>,
    post_id: &str,
) -> Result<Option<Post>, Error> {
    let obj_id = match ObjectId::parse_str(post_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid Post ID")),
    };

    let filter = doc! { "_id": obj_id };
    let post = collection.find_one(filter).await?;
    Ok(post)
}

pub async fn get_all_posts(collection: &Collection<Post>) -> Result<Vec<Post>, Error> {
    let mut cursor = collection.find(doc! {}).await?;
    let mut posts: Vec<Post> = Vec::new();
    while let Some(post) = cursor.try_next().await? {
        posts.push(post);
    }
    Ok(posts)
}

pub async fn update_post(
    collection: &Collection<Post>,
    post_id: &str,
    updated_post: Post,
) -> Result<UpdateResult, Error> {
    let obj_id = match ObjectId::parse_str(post_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid Post ID")),
    };

    let filter = doc! { "_id": obj_id };
    let update = doc! {
        "$set": {
            "content": updated_post.content,
            "images": updated_post.images,
            "videos": updated_post.videos,
            "tags": updated_post.tags,
        }
    };
    let result = collection.update_one(filter, update).await?;
    Ok(result)
}

pub async fn delete_post(
    collection: &Collection<Post>,
    post_id: &str,
) -> Result<DeleteResult, Error> {
    let obj_id = match ObjectId::parse_str(post_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid Post ID")),
    };
    let filter = doc! { "_id": obj_id };
    let result = collection.delete_one(filter).await?;
    Ok(result)
}
