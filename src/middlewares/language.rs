use crate::utils::context::{CtxApp, CTX_APP};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use hyper::StatusCode;
use std::collections::HashSet;

pub async fn accept_language(req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let mut ctx_app = CtxApp::default();
    let mut key_languages: HashSet<String> = HashSet::new();
    ctx_app.user_id = "Anonymous".to_string();

    let header_map = req.headers();
    let header_value_opt = header_map.get("Accept-Language");
    if let Some(header_value) = header_value_opt {
        if let Ok(result) = header_value.to_str() {
            let accept_languages = result.trim().split(",").collect::<Vec<&str>>();
            for language in accept_languages {
                let language = language.trim();
                let lang_vec = language.split(";").collect::<Vec<&str>>();
                let lang = lang_vec[0].trim().to_string();
                key_languages.insert(lang.clone());

                let lang_vec = lang.split("-").collect::<Vec<&str>>();
                let lang = lang_vec[0].trim().to_string();
                key_languages.insert(lang);
            }
        }
    }

    if key_languages.len() == 0 {
        key_languages.insert("id".to_string());
    }
    ctx_app.accept_languages = key_languages;

    let res = CTX_APP.scope(ctx_app, next.run(req)).await;

    Ok(res)
}
