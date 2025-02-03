use crate::models::user::User;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref USERS: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
}

pub fn save_user(user: User) {
    let mut users = USERS.lock().unwrap();
    users.insert(user.id.clone(), user);
}

pub fn get_user_by_email(email: &str) -> Option<User> {
    let users = USERS.lock().unwrap();
    users.values().find(|user| user.email == email).cloned()
}
