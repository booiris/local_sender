use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::http_resp::{BaseResponse, ErrorResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct MessageRequest {
    pub msg: String,
    pub time: i64,
    pub from_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct MessageResponse {
    pub base: BaseResponse,
}

pub async fn message(
    Json(req): Json<MessageRequest>,
) -> Result<Json<MessageResponse>, ErrorResponse> {
    log::info!("message: {:?}", req);
    Ok(Json(MessageResponse::default()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_message() {}
}
