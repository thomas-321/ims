use std::sync::{Arc, Mutex, LazyLock};
use rand::{Rng, thread_rng, distributions::Alphanumeric};
use sqlx;

use api_models::user::model::{User, DBUser, LoginPayload, CreateUserPayload};
use crate::database;
use crate::error::ApiError;

pub static LOGGED_IN_USERS: LazyLock<Arc<Mutex<Vec<User>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

pub fn is_logged_in(login_key: &String) -> Result<bool, ApiError> {
    let users = match LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ApiError::LoggedInUserLockFailed)
    };

    for user in users.iter() {
        if user.login_key == *login_key {
            return Ok(true);
        }
    }

    return Ok(false);
}

pub fn validate_create_user_payload(payload: &CreateUserPayload) -> Result<(), ApiError> {
    if payload.firstname.len() < 2 {
        return Err(ApiError::FirstnameTooShort);
    }

    if payload.lastname.len() < 2 {
        return Err(ApiError::LastnameTooShort);
    }



    return Ok(());
}



pub async fn create_user(user_payload: &CreateUserPayload) -> Result<(), ApiError> {
    let db = database::get_pool().await;
    
    let result = sqlx::query!(
        r#"INSERT INTO users (firstname, lastname, role_id, email, password) 
           VALUES (?, ?, (SELECT role_id from roles where role = ?), ?, ?)"#,
        &user_payload.firstname,
        &user_payload.lastname,
        &user_payload.role,
        &user_payload.email,
        &user_payload.password,
    )
    .execute(&*db)
    .await;

    // check for errors such as Column 'role_id' cannot be null
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.to_string().contains("Column 'role_id' cannot be null") {
                return Err(ApiError::InvalidRole);
            }
            return Err(ApiError::UserAlreadyExists);
        }
    }
}

pub async fn check_credentials (json_payload: &LoginPayload) -> Result<DBUser, ApiError> {
    let db = database::get_pool().await;

    let result = sqlx::query_as!(
        DBUser,
        r#"SELECT users.user_id, users.firstname, users.lastname, roles.role, users.email, users.password FROM users JOIN roles ON users.role_id = roles.role_id 
           WHERE email = ? AND password = ?"#,
        &json_payload.email,
        &json_payload.password)
    .fetch_one(&*db)
    .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(ApiError::InvalidPassword),
    }

}

pub fn generate_login_key() -> String {
    let mut rng = thread_rng();

    // Generate a random alphanumeric string
    let key: String = (0..10)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    key
}

