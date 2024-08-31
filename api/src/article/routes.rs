
use actix_web::{get, web, Responder, HttpResponse};


#[get("/article/create")]
async fn create(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/article/delete")]
async fn delete(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(delete);
}
