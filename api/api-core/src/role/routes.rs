
use actix_web::{post, web, Responder, HttpResponse};

#[post("/role/change")]
async fn change() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

// create role - only by admins
#[post("/role/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[post("/role/delete")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(change);
    cfg.service(create);
    cfg.service(delete);
}
