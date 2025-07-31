use actix_web::HttpResponse;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("unexpected null; try decoding as an `Option`")]
pub enum ApiError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid password")]
    InvalidPassword,

    #[error("Invalid login key")]
    InvalidLoginKey,

    #[error("Invalid role")]
    InvalidRole,

    #[error("Failed to lock the 'logged in users' vector")]
    LoggedInUserLockFailed,

    #[error("Firstname is too short")]
    FirstnameTooShort,

    #[error("Lastname is too short")]
    LastnameTooShort,

    #[error("Failed to conect to the database")]
    DatabaseConnectionFailed,

    #[error("Failed to lock the roles vector")]
    RolesLockFailed,

    #[error("Unknown error, catched all reached")]
    UnknownError,

    #[error("JWT token is invalid")]
    JwtInvalid,

    #[error("JWT token is expired")]
    JwtExpired,

}

/// Value or Error to Responder:
/// 
/// Gets a Result containg T or an ApiError
/// if err returns an HttpResponse accroding to the error
/// if ok returns the value
pub fn value_or_err_to_responder<T>(result: Result<T, ApiError>) -> Result<T, HttpResponse> {
    match result {
        Ok(value) => Ok(value),
        Err(ApiError::InvalidLoginKey) => Err( HttpResponse::InternalServerError().json(json!(
            {"status": "failed", "message": "You are not authenticated"})) ),
        Err(ApiError::InvalidRole) =>     Err( HttpResponse::InternalServerError().json(json!(
            {"status": "failed", "message": "Role does not exist"})) ),
        Err(_) =>                         Err( HttpResponse::InternalServerError().json(json!(
            {"status": "failed", "message": "Internal server error"})) )
    }
}


// impl Responder for ApiError {
