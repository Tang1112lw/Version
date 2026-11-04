mod paths;
mod update;
use axum::Router;
use tokio::net::TcpListener;

use update::api::{download_router, version_check_router, version_router};
use update::package_compute::sync_package_metadata;

#[tokio::main]
async fn main() {
    sync_package_metadata(&paths::config_path())
        .await
        .expect("json配置信息 同步失败");

    let aicrm = Router::new()
        .merge(version_check_router())
        .merge(version_router())
        .merge(download_router());

    let listener = TcpListener::bind("0.0.0.0:5200").await.expect("端口被占用");

    println!("服务已启动,端口号5200");

    axum::serve(listener, aicrm).await.expect("服务启动失败");
}
