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
                    edit_roles as `edit_roles: bool`
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

