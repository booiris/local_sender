use axum::{extract::Query, Json};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use crate::{
    consts::{ASYNC_THRESHOLD, ASYNC_THRESHOLD_LOW, STREAM_THRESHOLD, STREAM_THRESHOLD_LOW},
    model::http_resp::{BaseResponse, ErrorResponse},
    utils::check_valid_path,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
enum PullMethod {
    Immediate,
    Stream,
    Async,

    #[default]
    Empty,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PullRequest {
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct PullResponse {
    data: Option<String>,
    method: PullMethod,
    size: u64,

    base: BaseResponse,
}

pub async fn pull(Query(req): Query<PullRequest>) -> Result<Json<PullResponse>, ErrorResponse> {
    let path = std::fs::canonicalize(req.path)
        .map_err(|e| "[pull] path invalid. err: ".to_owned() + &e.to_string())?;
    match check_valid_path(&path) {
        Ok(res) => {
            if !res {
                return Err("[pull] path invalid. err: path not in base dir"
                    .to_owned()
                    .into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    if !path.is_file() {
        return Err("[ls] path invalid. err: path is not a file"
            .to_owned()
            .into());
    }

    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| "[pull] can not open file. err: ".to_owned() + &e.to_string())?;
    let size = file
        .metadata()
        .await
        .map_err(|e| "[pull] can not read file. err: ".to_owned() + &e.to_string())?
        .len();

    match size {
        0 => Ok(Json(PullResponse::default())),
        1..=STREAM_THRESHOLD_LOW => {
            let mut data = Vec::with_capacity(size as usize);
            file.read_to_end(&mut data).await.map_err(|e| {
                "[pull] can not read file to data. err: ".to_owned() + &e.to_string()
            })?;

            let data = Base64::encode_string(&data);

            Ok(Json(PullResponse {
                data: Some(data),
                method: PullMethod::Immediate,
                size,
                ..Default::default()
            }))
        }
        STREAM_THRESHOLD..=ASYNC_THRESHOLD_LOW => Ok(Json(PullResponse {
            method: PullMethod::Stream,
            size,
            ..Default::default()
        })),
        ASYNC_THRESHOLD..=u64::MAX => Ok(Json(PullResponse {
            method: PullMethod::Async,
            size,
            ..Default::default()
        })),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_pull() {}
}
