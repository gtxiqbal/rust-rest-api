use crate::utils::context::CTX_APP;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::body::{to_bytes};
use bytes::Bytes;
use http_body::Body;
use hyper::StatusCode;
use log::info;
use crate::utils::api_response::ApiResponse;

pub async fn log_std_middleware(req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let method = req.method().as_str().to_string();
    let path = req.uri().path_and_query().unwrap().as_str().to_string();

    let (parts, body) = req.into_parts();
    let req_bytes = convert_to_bytes(body).await?;
    let body = axum::body::Body::from(req_bytes.clone());
    let req = Request::from_parts(parts, body);

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let res_bytes = convert_to_bytes(body).await?;
    let body = axum::body::Body::from(res_bytes.clone());
    let res = Response::from_parts(parts, body);

    let user_id = CTX_APP.get().user_id;
    let status = res.status().as_u16();
    info!("{user_id} - {status} - {method} {path}");
    write_log("request", &req_bytes);
    write_log("response", &res_bytes);
    Ok(res)
}

async fn convert_to_bytes(body: axum::body::Body) -> Result<Bytes, (StatusCode, String)> {
    let size_body = body.size_hint().lower() as usize;
    match to_bytes(body, size_body).await {
        Ok(result) => Ok(result),
        Err(err) => Err(ApiResponse::<()>::failed_internal(err.to_string()).into_json()),
    }
}

fn write_log(type_log: &str, bytes: &Bytes) {
    let result= serde_json::from_slice::<serde_json::Value>(&bytes.to_vec());
    if let Ok(json) = result {
        info!("{type_log}: {}", serde_json::to_string(&json).unwrap())
    }
}