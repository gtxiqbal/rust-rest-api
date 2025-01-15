use crate::configs::rsa_crypt;
use crate::configs::setting::Setting;
use crate::utils::context::TX_MANAGER;
use crate::utils::error::ErrorApp;
use sqlx::postgres::{PgArguments, PgPoolOptions, PgQueryResult};
use sqlx::query::Query;
use sqlx::{Error, Executor, Pool, Postgres};

pub async fn conn(setting: &mut Setting) -> Result<Pool<Postgres>, ErrorApp> {
    if setting.datasource.db.encrypt {
        setting.datasource.db.password =
            rsa_crypt::decrypt(setting.datasource.db.password.as_str())?;
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
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => Err(ErrorApp::OtherErr(err.to_string())),
    }
}

pub async fn execute<'q>(query: Query<'q, Postgres, PgArguments>) -> Result<PgQueryResult, Error> {
    let tx_manager = TX_MANAGER.get();
    let tx_opt = tx_manager.tx.lock().await.take();
    if let Some(mut tx) = tx_opt {
        let result = query.execute(&mut *tx).await;
        tx_manager.tx.lock().await.replace(tx);
        return result;
    }

    if tx_manager.is_tx {
        return Err(Error::Protocol("tx not found".to_string()));
    }

    let db = tx_manager.db;
    query.execute(&db).await
}