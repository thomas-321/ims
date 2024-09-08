// use serde::{Serialize, Deserialize};
use sqlx;

#[derive(Clone, sqlx::FromRow)]
pub struct Role {
    pub role_id: i32,
    pub role: String,
    pub create_articles: bool,
    pub delete_articles: bool,
    pub read_articles: bool,
    pub create_quotations: bool,
    pub delete_quotations: bool,
    pub read_quotations: bool,
    pub can_edit_roles: bool,
    pub can_edit_users: bool,
}


