use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct VersionConfig{
    pub app_id: String,
    pub min_support_code: u32,
    pub release:Vec<Release>,
}



#[derive(Serialize,Deserialize)]
pub struct Release{
    pub version:String,
    pub support_code:u32,
    pub download_url:String,
    pub size:u32,
    pub hash:String,
    pub release_notes:String,
}

