
use actix_web::{get, web, Responder, HttpResponse};

#[get("/role/change")]
async fn change() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

// create user - only by admins
#[get("/role/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/role/delete")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(change);
    cfg.service(create);
    cfg.service(delete);
}
