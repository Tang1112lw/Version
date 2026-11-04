use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VersionRequest {
    pub app_name: String,
    pub support_code: u32,
}

#[derive(Serialize, Deserialize)]
pub struct VersionRelease {
    pub app_name: String,
    pub support_code: u32,
    pub version: String,
    pub download_url: String,
    #[serde(default, skip_serializing)]
    pub package_path: String,
    pub size: u64,
    pub hash: String,
    pub release_note: String,
}

#[derive(Serialize, Deserialize)]
pub struct VersionResponse {
    pub release_notes: String,
    pub update: bool,
}
