use serde::{Serialize, Deserialize};


#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}


