use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::http_resp::{BaseResponse, ErrorResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PullRequest {}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct PullResponse {
    pub base: BaseResponse,
}

pub async fn pull(Json(_req): Json<PullRequest>) -> Result<Json<PullResponse>, ErrorResponse> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_pull() {}
}
