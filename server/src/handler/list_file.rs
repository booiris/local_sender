use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

use crate::model::http_resp::{BaseResponse, ErrorResponse};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LsRequest {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct LsResponse {
    pub files: Vec<String>,

    pub base: BaseResponse,
}

pub async fn ls(Query(req): Query<LsRequest>) -> Result<Json<LsResponse>, ErrorResponse> {
    let base = std::env::current_dir()
        .map_err(|e| "[ls] can not get base dir. err: ".to_owned() + &e.to_string())?;

    let path = std::fs::canonicalize(req.path)
        .map_err(|e| "[ls] path invalid. err: ".to_owned() + &e.to_string())?;

    if !path.starts_with(base.clone()) {
        return Err("[ls] path invalid. err: path not in base dir"
            .to_owned()
            .into());
    }

    let mut files = vec![];
    for entry in std::fs::read_dir(path)
        .map_err(|e| "[ls] can not read dir. err: ".to_owned() + &e.to_string())?
    {
        let entry =
            entry.map_err(|e| "[ls] can not read dir. err: ".to_owned() + &e.to_string())?;
        let path = entry.path();
        let path = path.strip_prefix(&base).map_err(|e| e.to_string())?;
        if let Some(path) = path.to_str() {
            files.push(path.to_owned());
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
                files: ["Cargo.toml".to_owned(), "src".to_owned()].to_vec(),
                ..Default::default()
            };

            let req = LsRequest { path: "./".into() };
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
