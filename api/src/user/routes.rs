use actix_web::{get, web, Responder, HttpResponse};
use serde_json::json;
use super::helpers;
use super::model;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(login);
    cfg.service(logout);
}

// create user - only by admins
#[get("/user/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/logout")]
async fn logout(json_payload: web::Json<model::LogoutPayload>) -> impl Responder {
    let mut users = match model::LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json(json!(
                    {"status": "failed", "message": "Internal server error"}))
    };

    if let Some(index) = users.iter().position(|x| x.login_key == json_payload.login_key) {
        users.remove(index);
        return HttpResponse::Ok().json(json!({"status": "success", "message": "User is logged out"}));
    }
    
    HttpResponse::NotFound().json(json!({"status": "failed", "message": "User not found"}))
}



