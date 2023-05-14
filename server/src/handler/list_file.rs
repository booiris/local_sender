use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::{
    model::http_resp::{BaseResponse, ErrorResponse},
    utils::check_valid_path,
};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LsRequest {
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct LsResponse {
    files: Vec<String>,

    base: BaseResponse,
}

pub async fn ls(Query(req): Query<LsRequest>) -> Result<Json<LsResponse>, ErrorResponse> {
    let path = std::fs::canonicalize(req.path)
        .map_err(|e| "[ls] path invalid. err: ".to_owned() + &e.to_string())?;
    match check_valid_path(&path) {
        Ok(res) => {
            if !res {
                return Err("[ls] path invalid. err: path not in base dir"
                    .to_owned()
                    .into());
            }
        }
        Err(e) => return Err(e.into()),
    }

    let mut files = vec![];
    for entry in (std::fs::read_dir(path)
        .map_err(|e| "[ls] can not read dir. err: ".to_owned() + &e.to_string())?)
    .flatten()
    {
        let path = entry.path();
        if let Some(path) = path.file_name() {
            if let Some(path) = path.to_str() {
                files.push(path.to_owned());
            }
        }
    }

    Ok(Json(LsResponse {
        files,
        ..Default::default()
    }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_ls() {
        {
            let expect = LsResponse {
                files: ["test.txt".to_owned()].to_vec(),
                ..Default::default()
            };

            let req = LsRequest {
                path: "./test/test_file/test_ls/".into(),
            };
            let resp = ls(Query(req)).await.expect("expect ok, return err").0;

            assert_eq!(expect, resp);
        }

        {
            let expect = ErrorResponse {
                base: BaseResponse {
                    code: -1,
                    msg: "[ls] path invalid. err: path not in base dir".to_owned(),
                },
            };

            let req = LsRequest { path: "../".into() };
            let resp = ls(Query(req)).await.expect_err("expect err, return ok");

            assert_eq!(expect, resp);
        }

        {
            let expect = ErrorResponse {
                base: BaseResponse {
                    code: -1,
                    msg: "[ls] path invalid. err: No such file or directory (os error 2)".into(),
                },
            };

            let req = LsRequest { path: "..?".into() };
            let resp = ls(Query(req)).await.expect_err("expect err, return ok");

            assert_eq!(expect, resp);
        }
    }
}
