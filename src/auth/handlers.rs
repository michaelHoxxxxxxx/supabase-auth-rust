use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::auth::models::{LoginRequest, RegisterRequest};
use crate::auth::services::{get_user_by_id, login_user, register_user};
use crate::db::DbPool;
use crate::errors::ServiceError;

pub async fn register_handler(
    pool: web::Data<DbPool>,
    register_data: web::Json<RegisterRequest>,
) -> impl Responder {
    let register_data = register_data.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 注册用户
    match register_user(&mut conn, register_data).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => {
            eprintln!("Error registering user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to register user")
        }
    }
}

pub async fn login_handler(
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginRequest>,
) -> impl Responder {
    let login_data = login_data.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 登录用户
    match login_user(&mut conn, login_data).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            eprintln!("Error logging in: {:?}", e);
            if let ServiceError::InvalidCredentials = e {
                HttpResponse::Unauthorized().json("Invalid credentials")
            } else {
                HttpResponse::InternalServerError().json("Failed to login")
            }
        }
    }
}

pub async fn get_user_handler(
    pool: web::Data<DbPool>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let user_id = user_id.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 获取用户信息
    match get_user_by_id(&mut conn, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Error getting user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to get user")
        }
    }
}
