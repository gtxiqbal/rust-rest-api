use crate::configs::rsa_crypt;
use crate::configs::setting::Setting;
use crate::utils::error::ErrorApp;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn conn(setting: &mut Setting) -> Result<Pool<Postgres>, ErrorApp> {
    if setting.datasource.db.encrypt {
        setting.datasource.db.password =  rsa_crypt::decrypt(setting.datasource.db.password.as_str())?;
    }
    let url = format!(
        "{}://{}:{}@{}:{}/{}?search_path={}&sslmode={}",
        setting.datasource.db.driver,
        setting.datasource.db.username,
        setting.datasource.db.password,
        setting.datasource.db.host,
        setting.datasource.db.port,
        setting.datasource.db.name,
        setting.datasource.db.schema,
        setting.datasource.db.sslmode,
    );

    match PgPoolOptions::new()
        .min_connections(setting.datasource.db.min_conn as u32)
        .max_connections(setting.datasource.db.max_conn as u32)
        .acquire_timeout(std::time::Duration::from_secs(
            setting.datasource.db.acquire_timeout as u64,
        ))
        .idle_timeout(std::time::Duration::from_secs(
            setting.datasource.db.idle_timeout as u64,
        ))
        .connect(&url)
        .await {
        Ok(result) => Ok(result),
        Err(err) => Err(ErrorApp::OtherErr(err.to_string()))
    }
}
