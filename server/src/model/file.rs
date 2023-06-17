use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct FileInfo {
    pub name: String,
    pub size: String,
    pub modified_time: String,
}
