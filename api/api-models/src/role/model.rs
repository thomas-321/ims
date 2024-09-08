// use serde::{Serialize, Deserialize};

pub struct Role {
    pub role_id: i32,
    pub role: String,
    pub create_quotations: bool,
    pub delete_quotations: bool,
    pub create_articles: bool,
    pub delete_articles: bool,
    pub can_change_users: bool,
    pub change_role_permissions: bool,
}


