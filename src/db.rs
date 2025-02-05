use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Database,
};
use std::sync::Arc;
use tokio::sync::OnceCell;

static MONGO_CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

/// Async function to initialize the MongoDB client
async fn init_mongo_client() -> Arc<Client> {
    let uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");

    // Set up MongoDB client options
    let mut client_options = ClientOptions::parse(&uri)
        .await
        .expect("Failed to parse MongoDB options");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create and return the MongoDB client
    Arc::new(Client::with_options(client_options).expect("Failed to create MongoDB client"))
}

/// Helper function to get or initialize the MongoDB client
async fn get_mongo_client() -> Arc<Client> {
    MONGO_CLIENT
        .get_or_init(|| async { init_mongo_client().await })
        .await
        .clone()
}

/// Function to get a database reference
pub async fn get_db() -> Database {
    let client = get_mongo_client().await;
    client.database("habit_tracker")
}
