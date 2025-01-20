use crate::configs::setting::Setting;
use sqlx::{Pool, Postgres};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Default)]
pub struct CtxApp {
    pub accept_languages: HashSet<String>,
    pub user_id: String,
}

#[derive(Clone)]
pub struct TxManager {
    pub db: Pool<Postgres>,
    pub tx: Arc<Mutex<Option<sqlx::Transaction<'static, Postgres>>>>,
    pub is_tx: bool,
}


tokio::task_local! {
    pub static CTX_APP: CtxApp;
    pub static TX_MANAGER: TxManager;
    pub static SETTING: Setting;
}
