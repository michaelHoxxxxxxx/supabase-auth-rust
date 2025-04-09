use actix_web::{web, Scope};

use crate::auth::handlers::{register_handler, login_handler, get_user_handler};
use crate::permissions::handlers::{create_role_handler, create_permission_handler, assign_role_handler, assign_permission_handler};
use crate::utils::middleware::AuthMiddleware;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // 认证路由
    cfg.service(auth_routes());

    // 权限管理路由
    cfg.service(
        web::scope("/permissions")
            .wrap(AuthMiddleware::new())
            .route("/roles", web::post().to(create_role_handler))
            .route("/permissions", web::post().to(create_permission_handler))
            .route("/users/{user_id}/roles/{role_id}", web::post().to(assign_role_handler))
            .route("/roles/{role_id}/permissions/{permission_id}", web::post().to(assign_permission_handler))
    );

    // 示例：使用权限中间件保护的路由
    cfg.service(
        web::scope("/admin")
            .wrap(AuthMiddleware::new())
            .route("/dashboard", web::get().to(|| async { "管理员仪表盘" }))
    );
}

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
        .service(
            web::scope("/users")
                .wrap(AuthMiddleware::new())
                .route("/{user_id}", web::get().to(get_user_handler))
        )
}
