use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;

use crate::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

/// 生成JWT令牌
pub fn generate_token(user_id: Uuid) -> Result<String, ServiceError> {
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|e| ServiceError::JwtError(format!("JWT_SECRET not set: {}", e)))?;
    
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| ServiceError::JwtError(e.to_string()))
}

/// 验证JWT令牌
pub fn verify_token(token: &str) -> Result<Uuid, ServiceError> {
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|e| ServiceError::JwtError(format!("JWT_SECRET not set: {}", e)))?;
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| ServiceError::JwtError(e.to_string()))?;
    
    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| ServiceError::JwtError(e.to_string()))?;
    
    Ok(user_id)
}
