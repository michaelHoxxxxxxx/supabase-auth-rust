use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();
        
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL必须设置"),
            supabase_url: env::var("SUPABASE_URL").expect("SUPABASE_URL必须设置"),
            supabase_key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY必须设置"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET必须设置"),
        }
    };
}

pub struct Config {
    pub database_url: String,
    pub supabase_url: String,
    pub supabase_key: String,
    pub jwt_secret: String,
}
