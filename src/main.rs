use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;

mod auth;
mod config;
mod db;
mod errors;
mod permissions;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化环境变量
    dotenv().ok();
    
    // 初始化日志
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // 创建数据库连接池
    let pool = db::init_pool();
    
    log::info!("启动服务器在 http://127.0.0.1:8080");
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
