use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    InvalidCredentials,
    UserAlreadyExists,
    RoleAlreadyExists,
    PermissionAlreadyExists,
    UserRoleAlreadyExists,
    RolePermissionAlreadyExists,
    UserNotFound,
    RoleNotFound,
    PermissionNotFound,
    DatabaseError(String),
    JwtError(String),
    PasswordHashError(String),
    SupabaseError(String),
    MissingToken,
    InvalidToken,
    InsufficientPermissions,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::InternalServerError => write!(f, "内部服务器错误"),
            ServiceError::BadRequest(msg) => write!(f, "错误的请求: {}", msg),
            ServiceError::Unauthorized(msg) => write!(f, "未授权: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "未找到: {}", msg),
            ServiceError::Conflict(msg) => write!(f, "冲突: {}", msg),
            ServiceError::InvalidCredentials => write!(f, "无效的凭证"),
            ServiceError::UserAlreadyExists => write!(f, "用户已存在"),
            ServiceError::RoleAlreadyExists => write!(f, "角色已存在"),
            ServiceError::PermissionAlreadyExists => write!(f, "权限已存在"),
            ServiceError::UserRoleAlreadyExists => write!(f, "用户已拥有该角色"),
            ServiceError::RolePermissionAlreadyExists => write!(f, "角色已拥有该权限"),
            ServiceError::UserNotFound => write!(f, "用户不存在"),
            ServiceError::RoleNotFound => write!(f, "角色不存在"),
            ServiceError::PermissionNotFound => write!(f, "权限不存在"),
            ServiceError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
            ServiceError::JwtError(msg) => write!(f, "JWT错误: {}", msg),
            ServiceError::PasswordHashError(msg) => write!(f, "密码哈希错误: {}", msg),
            ServiceError::SupabaseError(msg) => write!(f, "Supabase错误: {}", msg),
            ServiceError::MissingToken => write!(f, "缺少令牌"),
            ServiceError::InvalidToken => write!(f, "无效的令牌"),
            ServiceError::InsufficientPermissions => write!(f, "权限不足"),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json("内部服务器错误"),
            ServiceError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ServiceError::Unauthorized(msg) => HttpResponse::Unauthorized().json(msg),
            ServiceError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            ServiceError::Conflict(msg) => HttpResponse::Conflict().json(msg),
            ServiceError::InvalidCredentials => HttpResponse::Unauthorized().json("无效的凭证"),
            ServiceError::UserAlreadyExists => HttpResponse::Conflict().json("用户已存在"),
            ServiceError::RoleAlreadyExists => HttpResponse::Conflict().json("角色已存在"),
            ServiceError::PermissionAlreadyExists => HttpResponse::Conflict().json("权限已存在"),
            ServiceError::UserRoleAlreadyExists => HttpResponse::Conflict().json("用户已拥有该角色"),
            ServiceError::RolePermissionAlreadyExists => HttpResponse::Conflict().json("角色已拥有该权限"),
            ServiceError::UserNotFound => HttpResponse::NotFound().json("用户不存在"),
            ServiceError::RoleNotFound => HttpResponse::NotFound().json("角色不存在"),
            ServiceError::PermissionNotFound => HttpResponse::NotFound().json("权限不存在"),
            ServiceError::DatabaseError(msg) => HttpResponse::InternalServerError().json(msg),
            ServiceError::JwtError(msg) => HttpResponse::Unauthorized().json(msg),
            ServiceError::PasswordHashError(msg) => HttpResponse::InternalServerError().json(msg),
            ServiceError::SupabaseError(msg) => HttpResponse::InternalServerError().json(msg),
            ServiceError::MissingToken => HttpResponse::Unauthorized().json("缺少令牌"),
            ServiceError::InvalidToken => HttpResponse::Unauthorized().json("无效的令牌"),
            ServiceError::InsufficientPermissions => HttpResponse::Forbidden().json("权限不足"),
        }
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        match error {
            DieselError::DatabaseError(_, info) => {
                ServiceError::DatabaseError(info.message().to_string())
            }
            DieselError::NotFound => ServiceError::NotFound("记录不存在".to_string()),
            err => ServiceError::DatabaseError(format!("{}", err)),
        }
    }
}

impl From<R2D2Error> for ServiceError {
    fn from(error: R2D2Error) -> ServiceError {
        ServiceError::DatabaseError(format!("连接池错误: {}", error))
    }
}
