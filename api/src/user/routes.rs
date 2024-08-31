use actix_web::{get, web, Responder, HttpResponse};

// create user - only by admins
#[get("/user/create")]
async fn create(json_payload: web::Json<model::CreateUserPayload>) -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(login);
    cfg.service(logout);
}


