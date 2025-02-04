use async_once_cell::Lazy;
use mongo::{options::ClientOptions, Client, Database};
use std::sync::Arc;

static MONGO_CLIENT: Lazy<Arc<Client>> = Lazy::new(async {
    let uri = std::env::var("MONGO_URI").expect("MONGO_URI mus be set");
    let options = ClientOptions::parse(&uri)
        .await
        .expect("Failder to parse MongoDB opotions");
    Arc::new(Client::with_options(options).expect("Failed to create MongoDB client"));
});

pub async fn get_db() -> Database {
    let client = MONGO_CLIENT.await.clone();
    client.database("habit_tracker")
}
