use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::time::Duration;

use crate::config::CONFIG;

pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("数据库连接池创建失败")
}
