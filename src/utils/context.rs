use crate::configs::setting::Setting;
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct CtxApp {
    pub accept_languages: HashSet<String>,
    pub user_id: String,
    pub setting: Setting,
}
tokio::task_local! {
    pub static CTX_APP: CtxApp;
}
