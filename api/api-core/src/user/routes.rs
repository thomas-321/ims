use actix_web::{post, web, Responder, HttpResponse};
use serde_json::json;
use api_models::user::model::{LoginPayload};
use crate::user::db_requests;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(login);
    cfg.service(logout);
}

#[post("/user/create")]
async fn create() -> impl Responder {
    HttpResponse::NotImplemented().json(json!({"status": "failed", "message": "Api not implemented"}))
}

#[post("/user/login")]
async fn login(payload: web::Json<LoginPayload>) -> impl Responder {
    let auth_correct = db_requests::check_credentials(&payload.email, &payload.password).await; 
    if !auth_correct {
        return HttpResponse::Unauthorized().json(json!({"status": "failed", "message": "Invalid credentials"}));
    }
    else {
        return HttpResponse::Ok().json(json!({"status": "success"}));
    }
}

#[post("/user/logout")]
async fn logout() -> impl Responder {
    HttpResponse::NotImplemented().json(json!({"status": "failed", "message": "Api not implemented"}))
}

#[post("/user/checkauth")]
async fn checkauth() -> impl Responder {
    HttpResponse::NotImplemented().json(json!({"status": "failed", "message": "Api not implemented"}))
}
