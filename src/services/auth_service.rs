use crate::helps::set_session;
use crate::jwt::create_jwt;
use crate::models::user::{LoginRequest, RegisterRequest, User};
use crate::repositories::user_repository;
use actix_session::Session;
use bcrypt::{hash, verify};
use mongodb::results::InsertOneResult;
use mongodb::{error::Error, Collection};
use serde_json::Value;

// Register a new user
pub async fn register_user_service(
    collection: &Collection<User>,
    req: RegisterRequest,
) -> Result<InsertOneResult, Error> {
    let hashed_password = match hash(&req.password, 4) {
        Ok(pwd) => pwd,
        Err(_) => return Err(Error::custom("Invalid to register user")),
    };

    let new_user = User {
        id: None,
        username: req.username,
        hashed_password,
    };
    user_repository::create_user(collection, new_user).await
}

// Login user and generate JWT
pub async fn login_user_service(
    collection: &Collection<User>,
    req: LoginRequest,
    session: Session
) -> Result<Value, Box<dyn std::error::Error>> {
    let user_opt = user_repository::find_user_by_username(collection, &req.username).await?;
    if let Some(user) = user_opt {
        if verify(&req.password, &user.hashed_password)? {
            let token = create_jwt(&user.id.unwrap().to_hex(), "USER")?;
            let _ = set_session(session,"message".to_string() ,"hello".to_string()).await;
            Ok(serde_json::json!({
                "access_token": token
            }))
        } else {
            Err("Invalid credentials".into())
        }
    } else {
        Err("User not found".into())
    }
}
