use std::sync::{Arc, Mutex, LazyLock};
use rand::{Rng, thread_rng, distributions::Alphanumeric};
use sqlx::Error;

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

    return Err(ApiError::InvalidLoginKey);
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
           VALUES (?, ?, (SELECT role_id from roles where role = "New_User"), ?, ?)"#,
        &user_payload.firstname,
        &user_payload.lastname,
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
        Err(Error::RowNotFound) => {
            println!("Error: {:?}", ApiError::InvalidPassword.to_string());
            Err(ApiError::InvalidPassword)
        },
        Err(Error::PoolTimedOut) => {
            println!("Error: {:?}", ApiError::DatabaseConnectionFailed.to_string());
            Err(ApiError::DatabaseConnectionFailed)
        }
        Err(_) => {
            println!("Error: {:?}", ApiError::UnknownError.to_string());
            Err(ApiError::UnknownError)
        }
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

pub fn get_user_copy(login_key: &String) -> Result<User, ApiError> {
    let users = match LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ApiError::LoggedInUserLockFailed)
    };

    match users.iter().find(|x| x.login_key == *login_key) {
        Some(value) =>  Ok(value.clone()),
        None => Err(ApiError::InvalidLoginKey)
    }
}

pub fn get_user_role(login_key: &String) -> Result<String, ApiError> {
    let users = match LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ApiError::LoggedInUserLockFailed)
    };

    match users.iter().find(|x| x.login_key == *login_key) {
        Some(value) =>  Ok(value.role.clone()),
        None => Err(ApiError::InvalidLoginKey)
    }
}
