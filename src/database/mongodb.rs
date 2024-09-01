use mongodb::{options::ClientOptions, Client, Database};

use crate::get_config;

pub async fn get_database() -> Database {
    // Fetch the MongoDB URI from environment variables
    let mongo_uri = get_config().db_url;
    let db_name = get_config().db_name;

    // Parse the connection string into an options struct
    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Failed to parse MongoDB URI");

    // Get a handle to the MongoDB cluster
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");

    // Return a handle to the specified database
    client.database(&db_name)
}
