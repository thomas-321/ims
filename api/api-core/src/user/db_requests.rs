use crate::database;
use sqlx::mysql::MySqlPool;

pub async fn check_credentials (email: &str, password: &str) -> bool {
    let db = database::get_pool().await;

    let result = sqlx::query!(
        r#"SELECT users.user_id, users.email, users.password FROM users 
           WHERE email = ? AND password = ?"#,
        email,
        password)
    .fetch_one(&*db)
    .await;

    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}
