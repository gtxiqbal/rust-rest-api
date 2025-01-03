use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::config::setting::Setting;

pub async fn conn(setting: &Setting) -> Result<Pool<Postgres>, Error> {
    let url = format!("{}://{}:{}@{}:{}/{}?search_path={}&sslmode={}",
                      setting.datasource.db.driver,
                      setting.datasource.db.username,
                      setting.datasource.db.password,
                      setting.datasource.db.host,
                      setting.datasource.db.port,
                      setting.datasource.db.name,
                      setting.datasource.db.schema,
                      setting.datasource.db.sslmode,
    );
    PgPoolOptions::new()
        .min_connections(setting.datasource.db.min_conn as u32)
        .max_connections(setting.datasource.db.max_conn as u32)
        .acquire_timeout(std::time::Duration::from_secs(setting.datasource.db.acquire_timeout as u64))
        .idle_timeout(std::time::Duration::from_secs(setting.datasource.db.idle_timeout as u64))
        .connect(&url)
        .await
}