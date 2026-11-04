use axum::{
    Json, Router,
    body::Body,
    http::{StatusCode, header},
    response::Response,
    routing::post,
};

use std::sync::OnceLock;

use tokio::fs::File;
use tokio_util::io::ReaderStream;

use super::version::{VersionRelease, VersionRequest};
use crate::paths;

//版本校验配置文件初始化

static AICRM_CONFIG: OnceLock<VersionRelease> = OnceLock::new();

fn aicrm_config() -> &'static VersionRelease {
    AICRM_CONFIG.get_or_init(|| {
        let config_path = paths::config_path();
        let content = std::fs::read_to_string(&config_path)
            .unwrap_or_else(|error| panic!("无法读取配置 {}: {error}", config_path.display()));

        serde_json::from_str(&content).expect("配置文件格式错误")
    })
}

pub fn version_router() -> Router {
    Router::new().route("/aicrm/update/version", post(get_version))
}
pub fn version_check_router() -> Router {
    Router::new().route("/aicrm/update/version_check", post(version_check))
}
pub fn download_router() -> Router {
    Router::new().route("/aicrm/update/download", post(version_download))
}

async fn get_version(Json(request): Json<VersionRequest>) -> Json<bool> {
    let versionrelease: &VersionRelease = aicrm_config();

    if request.support_code < versionrelease.support_code {
        return Json(true);
    } else {
        return Json(false);
    }
}

async fn version_check(Json(request): Json<VersionRelease>) -> Json<bool> {
    let aicrm_config = aicrm_config();
    if request.hash.eq_ignore_ascii_case(&aicrm_config.hash) && request.size == aicrm_config.size {
        return Json(true);
    } else {
        Json(false)
    }
}

async fn version_download() -> Result<Response, StatusCode> {
    let release = aicrm_config();
    let package_path = paths::package_path(&release.package_path);

    let metadata = tokio::fs::metadata(&package_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let file = File::open(&package_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = Body::from_stream(ReaderStream::new(file));
    let filename = package_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("package.apk");

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "application/vnd.android.package-archive",
        )
        .header(header::CONTENT_LENGTH, metadata.len())
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        )
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
