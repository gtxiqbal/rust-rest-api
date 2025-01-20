use std::env;

pub mod app;
pub mod logging;
pub mod db;
pub mod rsa_crypt;
pub mod setting;

pub fn get_resources() -> String {
    env::var("RESOURCES_PATH").unwrap_or("resources".to_string())
}
