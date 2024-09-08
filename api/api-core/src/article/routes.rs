use actix_web::{post, web, Responder, HttpResponse};


#[post("/article/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[post("/article/delete")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(delete);
}
