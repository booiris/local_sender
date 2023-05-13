use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct BaseResponse {
    pub code: i32,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct ErrorResponse {
    pub base: BaseResponse,
}

impl From<String> for ErrorResponse {
    fn from(msg: String) -> Self {
        ErrorResponse {
            base: BaseResponse { code: -1, msg },
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(self).into_response()
    }
}
