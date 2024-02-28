use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

lazy_static! {
    pub static ref MONGO_URI: String = env::var("MONGO_URI").unwrap_or_default();
    pub static ref DATABASE_NAME: String = env::var("DATABASE_NAME").unwrap_or_default();
}
