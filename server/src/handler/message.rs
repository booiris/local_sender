use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::http_resp::{BaseResponse, ErrorResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct MessageRequest {
    msg: String,
    time: i64,
    from_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct MessageResponse {
    base: BaseResponse,
}

pub async fn message(
    Json(req): Json<MessageRequest>,
) -> Result<Json<MessageResponse>, ErrorResponse> {
    log::info!("message: {:?}", req);
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_message() {}
}
