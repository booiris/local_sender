use axum::{
    body::StreamBody,
    extract::Query,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;

use crate::{model::http_resp::ErrorResponse, utils::check_valid_path};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct StreamPullRequest {
    path: String,
}

pub async fn pull_stream(Query(req): Query<StreamPullRequest>) -> Result<Response, ErrorResponse> {
    let path = std::fs::canonicalize(req.path)
        .map_err(|e| "[stream_pull] path invalid. err: ".to_owned() + &e.to_string())?;
    match check_valid_path(&path) {
        Ok(res) => {
            if !res {
                return Err("[stream_pull] path invalid. err: path not in base dir"
                    .to_owned()
                    .into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    if !path.is_file() {
        return Err("[stream_pull] path invalid. err: path is not a file"
            .to_owned()
            .into());
    }

    let file = tokio::fs::File::open(path)
        .await
        .map_err(|e| "[stream_pull] can not open file. err: ".to_owned() + &e.to_string())?;

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok(body.into_response())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_temp() {}
}
