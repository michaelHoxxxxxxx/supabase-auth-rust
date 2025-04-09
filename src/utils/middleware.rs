use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpRequest, web,
    FromRequest, dev::Payload,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use uuid::Uuid;

use crate::utils::jwt::verify_token;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        AuthMiddleware
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // 从请求头中获取令牌
            let auth_header = req.headers().get("Authorization");
            
            let token = match auth_header {
                Some(header) => {
                    let header_str = header.to_str().map_err(|_| {
                        actix_web::error::ErrorUnauthorized("Invalid authorization header")
                    })?;
                    
                    if !header_str.starts_with("Bearer ") {
                        return Err(actix_web::error::ErrorUnauthorized("Invalid token format"));
                    }
                    
                    header_str[7..].to_string()
                }
                None => {
                    return Err(actix_web::error::ErrorUnauthorized("Missing authorization token"));
                }
            };
            
            // 验证令牌
            let user_id = verify_token(&token).map_err(|_| {
                actix_web::error::ErrorUnauthorized("Invalid token")
            })?;
            
            // 将用户ID添加到请求扩展中
            req.extensions_mut().insert(user_id);
            
            // 继续处理请求
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

// 用于从请求中提取用户ID的提取器
pub struct AuthenticatedUser {
    pub id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user_id = req.extensions().get::<Uuid>().cloned();
        
        match user_id {
            Some(id) => ready(Ok(AuthenticatedUser { id })),
            None => ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized").into())),
        }
    }
}

// 权限检查中间件
pub struct PermissionCheckMiddleware {
    resource: String,
    action: String,
}

impl PermissionCheckMiddleware {
    pub fn new(resource: &str, action: &str) -> Self {
        PermissionCheckMiddleware {
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PermissionCheckMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = PermissionCheckMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionCheckMiddlewareService {
            service: Rc::new(service),
            resource: self.resource.clone(),
            action: self.action.clone(),
        }))
    }
}

pub struct PermissionCheckMiddlewareService<S> {
    service: Rc<S>,
    resource: String,
    action: String,
}

impl<S, B> Service<ServiceRequest> for PermissionCheckMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let resource = self.resource.clone();
        let action = self.action.clone();

        Box::pin(async move {
            // 从请求中获取用户ID
            let user_id = match req.extensions().get::<Uuid>() {
                Some(id) => *id,
                None => return Err(actix_web::error::ErrorForbidden("Access denied")),
            };
            
            // 获取数据库连接
            let pool = req.app_data::<web::Data<crate::db::DbPool>>()
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("Database pool not found"))?;
            
            let mut conn = pool.get()
                .map_err(|_| actix_web::error::ErrorInternalServerError("Database connection error"))?;
            
            // 检查用户是否有权限
            let has_permission = crate::permissions::services::check_user_permission(
                &mut conn, 
                user_id, 
                &resource, 
                &action
            )?;
            
            if !has_permission {
                return Err(actix_web::error::ErrorForbidden("Insufficient permissions"));
            }
            
            // 继续处理请求
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
