use axum::{routing,Json,Router};

use super::version::VersionConfig;

pub fn version_router() -> Router{
    Router::new()
    .route("/update/version",get(get_version))
}

pub fn download_router() ->Router{
    Router::new()
    .route("/update/{donwload}",post(get_download))
}


