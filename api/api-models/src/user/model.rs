use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
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


pub struct DBUser {
    pub user_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub role: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKeyPayload {
    pub login_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub iat: i64,
    pub exp: i64,
}
