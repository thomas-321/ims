use std::sync::{Arc, Mutex, LazyLock};
// use serde::Deserialize;
use crate::database;
use crate::role;

static ROLES: LazyLock<Arc<Mutex<Vec<Role>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

#[derive(Clone)] 
#[allow(non_snake_case)]
pub struct Role {
    pub role_id: i32,
    pub role: String,
    pub createQuotations: bool,
    pub deleteQuotations: bool,
    pub createArticles: bool,
    pub deleteArticles: bool,
    pub canChangeUsers: bool,
    pub changeRolePermissions: bool,
}


