use human_bytes::human_bytes;

use axum::{extract::Query, Json};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{
    model::{
        file::FileInfo,
        http_resp::{BaseResponse, ErrorResponse},
    },
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
    files: Vec<FileInfo>,
    dirs: Vec<FileInfo>,

    base: BaseResponse,
}

// TODO: optimize read limit
const READ_LIMIT: usize = 100;

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
    let mut dirs = vec![];
    for (cnt, entry) in (std::fs::read_dir(path)
        .map_err(|e| "[ls] can not read dir. err: ".to_owned() + &e.to_string())?)
    .flatten()
    .enumerate()
    {
        if cnt >= READ_LIMIT {
            break;
        }
        let path = entry.path();
        if let (Ok(meta), Some(file_name)) = (path.metadata(), path.file_name()) {
            if let (Some(file_name), Ok(modified)) = (file_name.to_str(), meta.modified()) {
                let modified: DateTime<Local> = modified.into();
                let modified = modified.format("%Y/%m/%d %H:%M").to_string();
                if meta.is_dir() {
                    dirs.push(FileInfo {
                        name: file_name.to_owned(),
                        size: "0 B".into(),
                        modified_time: modified,
                    });
                } else {
                    files.push(FileInfo {
                        name: file_name.to_owned(),
                        size: human_bytes(meta.len() as f64).to_string(),
                        modified_time: modified,
                    });
                }
            }
        }
    }

    Ok(Json(LsResponse {
        files,
        dirs,
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
                files: vec![FileInfo {
                    name: "test.txt".into(),
                    size: "9 B".into(),
                    modified_time: "".into(),
                }],
                ..Default::default()
            };

            let req = LsRequest {
                path: "./test/test_file/test_ls/".into(),
            };
            let mut resp = ls(Query(req)).await.expect("expect ok, return err").0;
            resp.files[0].modified_time = expect.files[0].modified_time.clone();

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
