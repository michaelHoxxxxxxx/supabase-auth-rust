use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::errors::ServiceError;

/// 使用Argon2哈希密码
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    // 哈希密码
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ServiceError::PasswordHashError(e.to_string()))?
        .to_string();

    Ok(password_hash)
}

/// 验证密码
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, ServiceError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| ServiceError::PasswordHashError(e.to_string()))?;

    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    
    Ok(result.is_ok())
}
