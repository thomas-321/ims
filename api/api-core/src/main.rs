
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

mod database;
mod error;
mod user;
mod role;
mod article;
mod auth;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", &name))
}


#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    database::init_pool(&database_url).await.expect("Database must be initialized");
    role::init_roles().await.expect("Roles must be initialized");

    HttpServer::new(move || {
        App::new()
            .configure(user::init_routes)
            .configure(role::init_routes)
            .configure(article::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

