pub enum ApiError {
    UserAlreadyExists,
    InvalidPassword,
    InvalidLoginKey,
    InvalidRole,
    LoggedInUserLockFailed,
    FirstnameTooShort,
    LastnameTooShort,
}

pub fn get_error_string(error: ApiError) -> String {
    match error {
        ApiError::UserAlreadyExists => "User already exists".to_string(),
        ApiError::InvalidPassword => "Invalid password".to_string(),
        ApiError::InvalidLoginKey => "Invalid login key".to_string(),
        ApiError::InvalidRole => "Invalid role".to_string(),
        ApiError::FirstnameTooShort => "Firstname is too short".to_string(),
        ApiError::LastnameTooShort => "Lastname is too short".to_string(),
        ApiError::LoggedInUserLockFailed => "Failed to lock logged in users".to_string(),
    }
}