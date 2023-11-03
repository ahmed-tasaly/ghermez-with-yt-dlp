use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use aria2_ws::response::{File, TaskStatus};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CustomStatus {
    /// GID of the download.
    pub gid: String,

    pub status: TaskStatus,

    #[serde_as(as = "DisplayFromStr")]
    pub connections: u64,

    pub error_code: Option<String>,

    pub error_message: Option<String>,

    #[serde_as(as = "DisplayFromStr")]
    pub download_speed: u64,

    pub dir: String,

    #[serde_as(as = "DisplayFromStr")]
    pub total_length: u64,

    #[serde_as(as = "DisplayFromStr")]
    pub completed_length: u64,

    pub files: Vec<File>,
}

pub trait ValuesToString {
    fn to_string(&self) -> String;
}

impl ValuesToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Active => "active".to_string(),
            TaskStatus::Complete => "complete".to_string(),
            TaskStatus::Error => "error".to_string(),
            TaskStatus::Paused => "paused".to_string(),
            TaskStatus::Removed => "removed".to_string(),
            TaskStatus::Waiting => "waiting".to_string(),
        }
    }
}
