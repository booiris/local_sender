use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::http_resp::{BaseResponse, ErrorResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct TempRequest {}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct TempResponse {
    base: BaseResponse,
}

pub async fn temp(Json(_req): Json<TempRequest>) -> Result<Json<TempResponse>, ErrorResponse> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_temp() {}
}
