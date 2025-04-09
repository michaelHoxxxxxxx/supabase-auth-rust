use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::env;

use crate::errors::ServiceError;

// 定义Supabase配置
struct SupabaseConfig {
    supabase_url: String,
    supabase_key: String,
}

// 获取Supabase配置
fn get_config() -> Result<SupabaseConfig, ServiceError> {
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|e| ServiceError::SupabaseError(format!("SUPABASE_URL not set: {}", e)))?;
    let supabase_key = env::var("SUPABASE_KEY")
        .map_err(|e| ServiceError::SupabaseError(format!("SUPABASE_KEY not set: {}", e)))?;
    
    Ok(SupabaseConfig {
        supabase_url,
        supabase_key,
    })
}

// 创建Supabase HTTP客户端
pub async fn create_client() -> Result<Client, ServiceError> {
    let config = get_config()?;
    
    let mut headers = header::HeaderMap::new();
    
    // 添加Supabase API密钥
    let auth_value = header::HeaderValue::from_str(&format!("Bearer {}", config.supabase_key))
        .map_err(|e| ServiceError::SupabaseError(format!("Invalid header value: {}", e)))?;
    
    headers.insert(header::AUTHORIZATION, auth_value);
    
    // 添加Content-Type
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    
    // 创建客户端
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| ServiceError::SupabaseError(format!("Failed to create HTTP client: {}", e)))?;
    
    Ok(client)
}

// 用户注册请求体
#[derive(Serialize)]
pub struct SupabaseSignUpRequest {
    pub email: String,
    pub password: String,
    pub data: Option<serde_json::Value>,
}

// 用户注册响应体
#[derive(Deserialize, Debug)]
pub struct SupabaseSignUpResponse {
    pub id: String,
    pub email: String,
}

// 在Supabase中注册用户
pub async fn sign_up_user(email: &str, password: &str) -> Result<SupabaseSignUpResponse, ServiceError> {
    let config = get_config()?;
    let client = create_client().await?;
    
    let request_body = SupabaseSignUpRequest {
        email: email.to_string(),
        password: password.to_string(),
        data: None,
    };
    
    let response = client
        .post(&format!("{}/auth/v1/signup", config.supabase_url))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| ServiceError::SupabaseError(format!("Failed to send request: {}", e)))?;
    
    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        
        return Err(ServiceError::SupabaseError(format!(
            "Supabase signup failed: {}",
            error_text
        )));
    }
    
    let user_data = response
        .json::<SupabaseSignUpResponse>()
        .await
        .map_err(|e| ServiceError::SupabaseError(format!("Failed to parse response: {}", e)))?;
    
    Ok(user_data)
}
