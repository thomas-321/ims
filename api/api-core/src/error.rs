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

    #[error("Failed to conect to the database")]
    LoggedInUserLockFailed,

    #[error("Firstname is too short")]
    FirstnameTooShort,

    #[error("Lastname is too short")]
    LastnameTooShort,

    #[error("Failed to conect to the database")]
    DatabaseConnectionFailed,

    #[error("Unknown error, catched all reached")]
    UnknownError
}
