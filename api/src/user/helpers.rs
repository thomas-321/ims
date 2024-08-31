use rand::{Rng, thread_rng, distributions::Alphanumeric};
use sqlx;

use super::model::{self, DBUser, LoginPayload};
use crate::database;

pub fn is_logged_in(login_key: &String) -> Result<bool, model::Error> {
    let users = match model::LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(model::Error::LoggedInUserLockFailed)
    };

    for user in users.iter() {
        if user.login_key == *login_key {
            return Ok(true);
        }
    }

    return Ok(false);
}

pub fn validate_create_user_payload(payload: &model::CreateUserPayload) -> Result<(), model::Error> {
    if payload.firstname.len() < 2 {
        return Err(model::Error::FirstnameTooShort);
    }

    if payload.lastname.len() < 2 {
        return Err(model::Error::LastnameTooShort);
    }



    return Ok(());
}

pub fn get_error_string(error: model::Error) -> String {
    match error {
        model::Error::UserAlreadyExists => "User already exists".to_string(),
        model::Error::InvalidPassword => "Invalid password".to_string(),
        model::Error::InvalidLoginKey => "Invalid login key".to_string(),
        model::Error::InvalidRole => "Invalid role".to_string(),
        model::Error::FirstnameTooShort => "Firstname is too short".to_string(),
        model::Error::LastnameTooShort => "Lastname is too short".to_string(),
        model::Error::LoggedInUserLockFailed => "Failed to lock logged in users".to_string(),
    }
}

pub async fn create_user(user_payload: &model::CreateUserPayload) -> Result<(), model::Error> {
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
                return Err(model::Error::InvalidRole);
            }
            return Err(model::Error::UserAlreadyExists);
        }
    }
}

pub async fn check_credentials (json_payload: &LoginPayload) -> Result<DBUser, model::Error> {
    let db = database::get_pool().await;

    let result = sqlx::query_as!(
        model::DBUser,
        r#"SELECT users.user_id, users.firstname, users.lastname, roles.role, users.email, users.password FROM users JOIN roles ON users.role_id = roles.role_id 
           WHERE email = ? AND password = ?"#,
        &json_payload.email,
        &json_payload.password)
    .fetch_one(&*db)
    .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(model::Error::InvalidPassword),
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

