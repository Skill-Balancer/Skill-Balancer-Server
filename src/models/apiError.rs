use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": self.message
        }));
        (self.status, body).into_response()
    }
}
