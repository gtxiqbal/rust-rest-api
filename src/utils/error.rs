use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ErrorApp {
    #[error("no rows returned by a query that expected to return at least one row")]
    RowNotFound,

    #[error("error duplicate data")]
    DuplicateKey,

    #[error("error: {0}")]
    OtherErr(String),
}

#[derive(Clone, Serialize)]
pub struct ErrorAuth {
    pub error: String,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
}

impl ErrorAuth {
    pub fn new() -> Self {
        Self {
            error: "".to_string(),
            error_description: None,
            error_uri: None,
        }
    }

    pub fn into_json(self) -> (StatusCode, String) {
        let result = serde_json::to_string(&self).unwrap();
        if self.error.eq("invalid_token") {
            return (StatusCode::UNAUTHORIZED, result);
        }
        (StatusCode::BAD_REQUEST, result)
    }
}
