use actix_web::{get, web, Responder, HttpResponse};
use serde_json::json;
use super::helpers;
use super::model;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(login);
    cfg.service(logout);
    cfg.service(checkauth);
}

// create user - only by admins
#[get("/user/create")]
async fn create(json_payload: web::Json<model::CreateUserPayload>) -> impl Responder {
    let logged_in = match helpers::is_logged_in(&json_payload.login_key) {
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().json(json!(
                {"status": "failed", "message": "Internal server error"}))
    };

    if !logged_in {
        return HttpResponse::Unauthorized().json(json!(
                {"status": "failed", "message": "Invalid login key"}));
    }

    match helpers::validate_create_user_payload(&json_payload) {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(json!(
                {"status": "failed", "message": helpers::get_error_string(e)}))
    }

    // TODO: chech if user is allowed to create users

    match helpers::create_user(&json_payload).await {
        Ok(_) => HttpResponse::Ok().json(json!(
                {"status": "success", "message": "User created"})),
        Err(_) => HttpResponse::InternalServerError().json(json!(
                {"status": "failed", "message": "Internal server error"}))
    }
}

#[get("/user/login")]
async fn login(json_payload: web::Json<model::LoginPayload>) -> impl Responder {


    let result = match helpers::check_credentials(&json_payload).await {
        Ok(value) => value,
        Err(_) => return HttpResponse::InternalServerError().json(json!(
                {"status": "failed", "message": "Internal server error"}))
    };

    let dbuser = match helpers::check_credentials(&json_payload).await {
        Ok(value) => value,
        Err(_) => return HttpResponse::Unauthorized().json(json!(
                {"status": "failed", "message": "Incorrect username or password"}))
    };

    let user = model::User {
        user_id: dbuser.user_id,
        firstname: dbuser.firstname,
        lastname: dbuser.lastname,
        role: dbuser.role,
        email: dbuser.email,
        password: dbuser.password,
        login_key: helpers::generate_login_key()
    }; 

    let mut users = match model::LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json(json!(
                {"status": "failed", "message": "Internal server error"}))
    };

    users.push(user);

    HttpResponse::Ok().json(json!(
        {"status": "success", 
         "message": "User is logged in", 
         "login_key": users.last().unwrap().login_key}))
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
        return HttpResponse::Ok().json(json!(
                {"status": "success", "message": "User is logged out"}));
    }
    
    HttpResponse::NotFound().json(json!({"status": "failed", "message": "User not found"}))
}



#[get("/user/checkauth")]
async fn checkauth(json_payload: web::Json<model::IsLoggedInPayload>) -> impl Responder {
    let mut users = match model::LOGGED_IN_USERS.lock() {
        Ok(guard) => guard,
        Err(_) => return HttpResponse::InternalServerError().json(json!(
                    {"status": "failed", "message": "Internal server error"}))
    };

    for user in users.iter() {
        if user.login_key == json_payload.login_key {
            return HttpResponse::Ok().json(json!(
                    {"status": "success", "message": "You are succesfuly authenticated"}));
        }
    }
    
    HttpResponse::NotFound().json(json!({"status": "failed", "message": "You are not authenticated"}))
}
