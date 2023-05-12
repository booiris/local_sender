use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LsRequest {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct LsResponse {
    pub files: Vec<String>,
}

pub async fn ls(Json(req): Json<LsRequest>) -> Result<Json<LsResponse>, String> {
    let base = std::env::current_dir()
        .map_err(|e| "can not get base dir. err: ".to_owned() + &e.to_string())?;

    let path = std::fs::canonicalize(req.path)
        .map_err(|e| "path invalid. err: ".to_owned() + &e.to_string())?;

    if !path.starts_with(base.clone()) {
        return Err("path invalid. err: path not in base dir".to_owned());
    }

    let mut files = vec![];
    for entry in std::fs::read_dir(path)
        .map_err(|e| "can not read dir. err: ".to_owned() + &e.to_string())?
    {
        let entry = entry.map_err(|e| "can not read dir. err: ".to_owned() + &e.to_string())?;
        let path = entry.path();
        let path = path.strip_prefix(&base).map_err(|e| e.to_string())?;
        if let Some(path) = path.to_str() {
            files.push(path.to_owned());
        }
    }

    Ok(Json(LsResponse { files }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_ls() {
        {
            let expect = LsResponse {
                files: ["Cargo.toml".to_owned(), "src".to_owned()].to_vec(),
            };

            let req = LsRequest { path: "./".into() };
            let resp = ls(Json(req)).await.expect("expect ok, return err").0;

            assert_eq!(expect, resp);
        }

        {
            let expect = "path invalid. err: path not in base dir";

            let req = LsRequest { path: "../".into() };
            let resp = ls(Json(req)).await.expect_err("expect err, return ok");

            assert_eq!(expect, resp);
        }

        {
            let expect = "path invalid. err: No such file or directory (os error 2)";

            let req = LsRequest { path: "..?".into() };
            let resp = ls(Json(req)).await.expect_err("expect err, return ok");

            assert_eq!(expect, resp);
        }
    }
}
