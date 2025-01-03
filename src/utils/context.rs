use std::collections::HashSet;
use crate::config::setting::{App, Database, Datasource, Setting};

#[derive(Clone, Debug)]
pub struct CtxApp {
    pub accept_languages: HashSet<String>,
    pub user_id: String,
    pub setting: Setting,
}

impl CtxApp {
    pub fn new() -> Self {
        Self{
            accept_languages: HashSet::new(),
            user_id: "".to_string(),
            setting: Setting {
                app: App {
                    name: "".to_string(),
                    desc: "".to_string(),
                    port: 0,
                },
                datasource: Datasource {
                    db: Database {
                        driver: "".to_string(),
                        host: "".to_string(),
                        port: 0,
                        name: "".to_string(),
                        schema: "".to_string(),
                        username: "".to_string(),
                        password: "".to_string(),
                        sslmode: "".to_string(),
                        min_conn: 0,
                        max_conn: 0,
                        idle_timeout: 0,
                        acquire_timeout: 0,
                        encrypt: false,
                    }
                }
            },
        }
    }
}

tokio::task_local! {
    pub static CTX_APP: CtxApp;
}