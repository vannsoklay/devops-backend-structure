use crate::post::{PostRequest, PostResponse};
use crate::{models::post::Post, post::Media};
use futures::stream::TryStreamExt;
use mongodb::bson::{from_bson, Bson};
use mongodb::{bson::oid::ObjectId, error::Error};
use mongodb::{
    bson::{doc, to_document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

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

pub async fn get_all_posts(collection: &Collection<Post>) -> Result<Vec<PostResponse>, Error> {
    let pipeline = vec![
        doc! {
            "$lookup": {
                "from": "users",           // Collection to join
                "localField": "author_id", // Field from the posts collection
                "foreignField": "_id",     // Field from the users collection
                "as": "user",              // Output array field
            }
        },
        doc! {
            "$unwind": "$user" // Unwind the user array to merge it into a single object
        },
        doc! {
            "$lookup": {
                "from": "tags",
                "localField": "tags",
                "foreignField": "_id",
                "as": "tag_details",
            }
        },
        doc! {
            "$project": {
                "_id": 1,
                "title": 1,
                "content": 1,
                "media": 1,
                "author": {
                    "_id": "$user._id",
                    "username": "$user.username",
                    "email": "$user.email",
                    "avatar": "$user.avatar",
                    "bio": "$user.bio",
                    "follower_count": "$user.follower_count",
                    "following_count": "$user.following_count",
                    "is_verified": "$user.is_verified",
                    "last_login": "$user.last_login",
                    "status": "$user.status",
                    "created_at": "$user.created_at",
                    "updated_at": "$user.created_at"
                },
                "tags": {
                    "$map": {
                        "input": "$tag_details",
                        "as": "tag",
                        "in": {
                            "_id": "$$tag._id",
                            "name": "$$tag.name"
                        }
                    }
                },
                "likes_count": 1,
                "comments_count": 1,
                "created_at": 1,
                "updated_at": 1,
            }
        },
        // Pagination: Sort by creation date and limit results
        doc! {
            "$sort": { "created_at": -1 } // Sort by creation date descending
        },
        doc! {
            "$limit": 10 // Limit to 10 documents per page
        },
    ];

    let mut cursor = collection.aggregate(pipeline).await?;
    let mut posts: Vec<PostResponse> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        let post: PostResponse = from_bson(Bson::Document(doc))?;
        posts.push(post);
    }
    Ok(posts)
}

pub async fn update_post(
    collection: &Collection<Post>,
    post_id: &str,
    updated_post: PostRequest,
) -> Result<UpdateResult, Error> {
    let obj_id = match ObjectId::parse_str(post_id) {
        Ok(id) => id,
        Err(_) => return Err(Error::custom("Invalid Post ID")),
    };

    let mut updated_media = vec![];

    for m in updated_post.media {
        updated_media.push(to_document(&Media {
            url: m.url,
            media_type: m.media_type,
        })?)
    }

    let filter = doc! { "_id": obj_id };
    let update = doc! {
        "$set": {
            "content": updated_post.content,
            "media": updated_media,
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
