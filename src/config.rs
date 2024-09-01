use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub db_url: String,
    pub db_name: String
}

pub fn get_config() -> Config {
    dotenv().ok();
    Config {
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        jwt_expiration: env::var("JWT_EXPIRATION")
            .expect("JWT_EXPIRATION must be set")
            .parse()
            .expect("JWT_EXPIRATION must be an integer"),
        db_url: env::var("DB_URL")
            .expect("JWT_EXPIRATION must be set")
            .parse()
            .expect("JWT_EXPIRATION must be an integer"),
        db_name: env::var("DB_NAME").expect("JWT_SECRET must be set")
    }
}
