use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[allow(unused)]
pub struct App {
    pub name: String,
    pub desc: String,
    pub port: i16,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[allow(unused)]
pub struct Database {
    pub driver: String,
    pub host: String,
    pub port: i16,
    pub name: String,
    pub schema: String,
    pub username: String,
    pub password: String,
    pub sslmode: String,
    pub min_conn: i16,
    pub max_conn: i16,
    pub idle_timeout: i16,
    pub acquire_timeout: i16,
    pub encrypt: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[allow(unused)]
pub struct Datasource {
    pub db: Database,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[allow(unused)]
pub struct Setting {
    pub app: App,
    pub datasource: Datasource,
}

impl Setting {
    pub fn new() -> Result<Self, ConfigError> {
        let resources_path = env::var("RESOURCES_PATH").unwrap_or("resources".to_string());
        let path_config = format!("{resources_path}/application");

        let mut file_name = format!("{path_config}.yaml");
        if let Ok(run_mode) = env::var("RUN_MODE") {
            if !run_mode.trim().eq("") && !run_mode.trim().eq("local") {
                let run_mode = run_mode.split(",").collect::<Vec<&str>>();
                file_name = format!("{path_config}-{}.yaml", run_mode.join("-"))
            }
        }
        let config = Config::builder()
            .add_source(File::with_name(&file_name))
            .build()?;

        config.try_deserialize()
    }
}
