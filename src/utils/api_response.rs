use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use crate::utils::error::ErrorApp;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    (StatusCode, Json<ApiResponse<T>>): IntoResponse,
    T: Serialize,
{
    pub fn failed_from<V>(resp: ApiResponse<V>) -> ApiResponse<T>
    where
        V: Serialize,
    {
        Self {
            code: resp.code,
            status: resp.status,
            message: resp.message,
            data: None,
        }
    }

    pub fn failed_not_found(message: String) -> ApiResponse<T> {
        Self::failed_with_code("99".to_string(), message)
    }

    pub fn failed_with_code(code: String, message: String) -> ApiResponse<T> {
        Self {
            code,
            status: "FAILED".to_string(),
            message,
            data: None,
        }
    }

    pub fn success(message: String, data: T) -> ApiResponse<T> {
        Self {
            code: "00".to_string(),
            status: "SUCCESS".to_string(),
            message,
            data: Some(data),
        }
    }

    pub fn failed_internal(message: String) -> ApiResponse<T> {
        Self::failed_with_code("93".to_string(), message)
    }

    pub fn into_response(self) -> impl IntoResponse {
        if self.code.eq("00") {
            (StatusCode::OK, Json(self))
        } else if self.code.eq("93") {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(self))
        } else if self.status.eq("NODATA") {
            (StatusCode::NOT_FOUND, Json(self))
        } else {
            (StatusCode::BAD_REQUEST, Json(self))
        }
    }

    pub fn into_json(self) -> (StatusCode, String) {
        let result = serde_json::to_string(&self).unwrap();
        if self.code.eq("00") {
            (StatusCode::OK, result)
        } else if self.code.eq("93") {
            (StatusCode::INTERNAL_SERVER_ERROR, result)
        } else if self.status.eq("NODATA") {
            (StatusCode::NOT_FOUND, result)
        } else {
            (StatusCode::BAD_REQUEST, result)
        }
    }
    
    pub fn response_from(result_response: Result<ApiResponse<T>, ErrorApp>) -> ApiResponse<T> {
        result_response.unwrap_or_else(|err| {
            match err {
                ErrorApp::WithCode(code, message) => Self::failed_with_code(code, message),
                _ => Self::failed_internal(err.to_string()),
            }
        })
    }
}
