use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use jsonwebtoken::errors::ErrorKind;
use once_cell::sync::Lazy;
use rand::RngCore;

use api_models::user::model::{Claims};

use crate::error::ApiError;

const CLAIM_DAYS_VALID:i64 = 1;

static HMAC_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut key = vec![0u8; 64]; // 512-bit key
    rand::thread_rng().fill_bytes(&mut key);
    key
});

/// Creates a JWT token for the given user ID and role.
/// JWT also contains the issue and expiration timestamps.
fn create_token (user_id: String, roles: Vec<String>) -> String {
    let jwt_payload = Claims {
        sub: user_id,
        roles,
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::days(CLAIM_DAYS_VALID)).timestamp(),
    };

    encode(
        &Header::default(),
        &jwt_payload,
        &EncodingKey::from_secret(&HMAC_SECRET),
    ).expect("JWT encoding failed")
} 

/// Verifies the JWT token and returns the claims if valid.
/// If the token is expired or otherwise incorrect, it returns an error.
fn check_token(token: &str) -> Result<Claims, ApiError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "sub", "roles"]);
    
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(&HMAC_SECRET), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => return Err(ApiError::JwtExpired),
            _ => return Err(ApiError::JwtInvalid),
        },
    };

    Ok(token_data.claims)
}

