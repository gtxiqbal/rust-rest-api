use std::env;

pub mod app;
pub mod pg_conn;
pub mod setting;
pub mod rsa_crypt;
mod log;

pub fn get_resources() -> String {
    env::var("RESOURCES_PATH").unwrap_or("resources".to_string())
}