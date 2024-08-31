
use std::sync::{Arc, Mutex, LazyLock};
use serde::Deserialize;
use crate::database;

// holds all loged-in users
pub static LOGGED_IN_USERS: LazyLock<Arc<Mutex<Vec<User>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

pub struct User {
    pub user_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub role: String,
    pub email: String,
    pub password: String,
    pub login_key: String    // a key given to a user after logging in to identify the user
    // TODO: last interaction time to remove login_keys for inactive users
}

// required json payload to log a user out
#[derive(Deserialize)]
pub struct LogoutPayload {
    pub login_key: String,
}
