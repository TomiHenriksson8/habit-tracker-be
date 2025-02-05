use crate::models::user::User;
use mongodb::{bson::doc, error::Result, Database};

pub async fn save_user(db: &Database, user: User) -> Result<()> {
    let collection = db.collection::<User>("users");
    collection.insert_one(user, None).await.map(|_| ())
}

pub async fn get_user_by_email(db: &Database, email: &str) -> Result<Option<User>> {
    let collection = db.collection::<User>("users");
    collection.find_one(doc! { "email": email }, None).await
}
