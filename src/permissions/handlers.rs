use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::db::DbPool;
use crate::errors::ServiceError;
use crate::permissions::models::{CreatePermissionRequest, CreateRoleRequest};
use crate::permissions::services::{assign_permission_to_role, assign_role_to_user, create_permission, create_role};

pub async fn create_role_handler(
    pool: web::Data<DbPool>,
    role_data: web::Json<CreateRoleRequest>,
) -> impl Responder {
    let role_data = role_data.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 创建角色
    match create_role(&mut conn, role_data) {
        Ok(role) => HttpResponse::Created().json(role),
        Err(e) => {
            eprintln!("Error creating role: {:?}", e);
            match e {
                ServiceError::RoleAlreadyExists => {
                    HttpResponse::Conflict().json("Role already exists")
                }
                _ => HttpResponse::InternalServerError().json("Failed to create role")
            }
        }
    }
}

pub async fn create_permission_handler(
    pool: web::Data<DbPool>,
    permission_data: web::Json<CreatePermissionRequest>,
) -> impl Responder {
    let permission_data = permission_data.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 创建权限
    match create_permission(&mut conn, permission_data) {
        Ok(permission) => HttpResponse::Created().json(permission),
        Err(e) => {
            eprintln!("Error creating permission: {:?}", e);
            match e {
                ServiceError::PermissionAlreadyExists => {
                    HttpResponse::Conflict().json("Permission already exists")
                }
                _ => HttpResponse::InternalServerError().json("Failed to create permission")
            }
        }
    }
}

pub async fn assign_role_handler(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> impl Responder {
    let (user_id, role_id) = path.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 分配角色
    match assign_role_to_user(&mut conn, user_id, role_id) {
        Ok(user_role) => HttpResponse::Created().json(user_role),
        Err(e) => {
            eprintln!("Error assigning role: {:?}", e);
            match e {
                ServiceError::UserRoleAlreadyExists => {
                    HttpResponse::Conflict().json("User already has this role")
                }
                ServiceError::UserNotFound => {
                    HttpResponse::NotFound().json("User not found")
                }
                ServiceError::RoleNotFound => {
                    HttpResponse::NotFound().json("Role not found")
                }
                _ => HttpResponse::InternalServerError().json("Failed to assign role")
            }
        }
    }
}

pub async fn assign_permission_handler(
    pool: web::Data<DbPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> impl Responder {
    let (role_id, permission_id) = path.into_inner();
    
    // 获取数据库连接
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return HttpResponse::InternalServerError().json("Database connection error");
        }
    };
    
    // 分配权限
    match assign_permission_to_role(&mut conn, role_id, permission_id) {
        Ok(role_permission) => HttpResponse::Created().json(role_permission),
        Err(e) => {
            eprintln!("Error assigning permission: {:?}", e);
            match e {
                ServiceError::RolePermissionAlreadyExists => {
                    HttpResponse::Conflict().json("Role already has this permission")
                }
                ServiceError::RoleNotFound => {
                    HttpResponse::NotFound().json("Role not found")
                }
                ServiceError::PermissionNotFound => {
                    HttpResponse::NotFound().json("Permission not found")
                }
                _ => HttpResponse::InternalServerError().json("Failed to assign permission")
            }
        }
    }
}
