use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::http_resp::{BaseResponse, ErrorResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PushRequest {}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct PushResponse {
    pub base: BaseResponse,
}

pub async fn push(Json(_req): Json<PushRequest>) -> Result<Json<PushResponse>, ErrorResponse> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_push() {}
}
