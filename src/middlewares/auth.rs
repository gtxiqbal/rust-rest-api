use crate::utils::context::CTX_APP;
use crate::utils::error::ErrorAuth;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use hyper::StatusCode;

pub async fn auth_check(req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let mut ctx_app = CTX_APP.get();
    let mut error_auth = ErrorAuth::new();
    let header_map = req.headers();
    let auth_opt = header_map.get("Authorization");
    if let Some(auth_header) = auth_opt {
        let auth = auth_header.to_str().unwrap().to_string();
        let (auth_type, token) = auth.split_once(" ").unwrap();

        if !auth_type.trim().eq("Bearer") {
            error_auth.error = "invalid_type_token".to_string();
            return Err(error_auth.into_json());
        }

        if token.trim().eq("") {
            error_auth.error = "missing_token".to_string();
            return Err(error_auth.into_json());
        }

        if !token.trim().eq("s9999") {
            error_auth.error = "invalid_token".to_string();
            return Err(error_auth.into_json());
        }

        ctx_app.user_id = token.to_string();
        return Ok(CTX_APP.scope(ctx_app, next.run(req)).await);
    }

    error_auth.error = "missing_token".to_string();
    Err(error_auth.into_json())
}
