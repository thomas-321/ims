use std::sync::{Arc, Mutex, LazyLock};
use sqlx::Error;

use api_models::role::model::Role;
use crate::user::helpers as user_helpers;
// use api_models::user::model::User;
use crate::error::ApiError;
use crate::database;

static ROLES: LazyLock<Arc<Mutex<Vec<Role>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

pub async fn init_roles() -> Result<(), ApiError> {
    let db = database::get_pool().await;

    let result = sqlx::query_as!(
        Role,
        r#"SELECT role_id, role,
                    create_articles as `create_articles: bool`,
                    delete_articles as `delete_articles: bool`,
                    read_articles as `read_articles: bool`,
                    create_quotations as `create_quotations: bool`,
                    delete_quotations as `delete_quotations: bool`,
                    read_quotations as `read_quotations: bool`,
                    can_edit_roles as `can_edit_roles: bool`,
                    can_edit_users as `can_edit_users: bool`
                    FROM roles"#)
    .fetch_all(&*db)
    .await;

    // return Err(ApiError::DatabaseConnectionFailed)
    let result = match result {
        Ok(roles) => roles,
        Err(Error::RowNotFound) => {
            println!("Error: {:?}", ApiError::InvalidPassword.to_string());
            return Err(ApiError::InvalidPassword);
        },
        Err(Error::PoolTimedOut) => {
            println!("Error: {:?}", ApiError::DatabaseConnectionFailed.to_string());
            return Err(ApiError::DatabaseConnectionFailed);
        }
        Err(_) => {
            println!("Error: {:?}", ApiError::UnknownError.to_string());
            return Err(ApiError::UnknownError);
        }
    };

    let mut roles = match ROLES.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return Err(ApiError::RolesLockFailed);
        }
    };


    for role in result {
        roles.push(role);
    }

    Ok(())
}

pub fn get_role_copy(rolename: &str) -> Result<Role, ApiError> {
    let roles = match ROLES.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ApiError::RolesLockFailed)
    };  

    for role in roles.iter() {
        if rolename == role.role {
            return Ok(role.clone());
        }
    }

    return Err(ApiError::InvalidRole);
}

pub fn get_user_role_from_login_key(login_key: &String) -> Result<Role, ApiError> {
    let user_role = match user_helpers::get_user_role(&login_key) {
        Ok(value) => value,
        Err(ApiError::LoggedInUserLockFailed) => { return Err(ApiError::LoggedInUserLockFailed); }
        Err(ApiError::InvalidLoginKey) =>        { return Err(ApiError::InvalidLoginKey); }
        Err(_) =>                                { return Err(ApiError::UnknownError); }
    };

    match get_role_copy(&user_role) {
        Ok(value) => Ok(value),
        Err(ApiError::RolesLockFailed) => return Err(ApiError::RolesLockFailed),
        Err(ApiError::InvalidRole) =>     return Err(ApiError::InvalidRole),
        Err(_) =>                         return Err(ApiError::UnknownError),
    }
}

