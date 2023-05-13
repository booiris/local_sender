use std::io::Read;

use axum::{extract::Query, Json};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::{
    consts::SIZE_THRESHOLD,
    model::http_resp::{BaseResponse, ErrorResponse},
    utils::check_valid_path,
};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct PullRequest {
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct PullResponse {
    data: Option<Vec<u8>>,
    hash: Option<String>,

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

    let mut file = std::fs::File::open(path)
        .map_err(|e| "[pull] can not open file. err: ".to_owned() + &e.to_string())?;
    let size = file
        .metadata()
        .map_err(|e| "[pull] can not read file. err: ".to_owned() + &e.to_string())?
        .len();

    if size < SIZE_THRESHOLD {
        let mut data = Vec::with_capacity(size as usize);
        let mut buffer = [0u8; 1024];
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => data.extend(&buffer[..n]),
                Err(e) => {
                    return Err(
                        ("[pull] can not read file. err: ".to_owned() + &e.to_string()).into(),
                    )
                }
            }
        }

        let hasher = Sha1::new().chain_update(&data).finalize();
        let hash = Base64::encode_string(&hasher);

        Ok(Json(PullResponse {
            data: Some(data),
            hash: Some(hash),
            ..Default::default()
        }))
    } else {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_pull() {}
}
