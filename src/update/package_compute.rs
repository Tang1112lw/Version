use sha2::{Digest, Sha256};
use std::io;
use std::path::Path;
use tokio::io::AsyncReadExt;

const BYTES_PER_MB: u64 = 1024 * 1024;

#[derive(Debug, Clone, PartialEq)]
pub struct PackageInfo {
    /// 精确字节数，用于安装包完整性校验。
    pub size_bytes: u64,
    /// 以 1024 * 1024 字节为 1 MB 换算后的展示大小。
    pub size_mb: f64,
    /// 小写十六进制 SHA-256。
    pub sha256: String,
}

/// 返回安装包的精确字节数。
pub async fn calculate_size_bytes(path: &Path) -> io::Result<u64> {
    let metadata = tokio::fs::metadata(path).await?;
    Ok(metadata.len())
}

/// 返回安装包大小，单位为 MB（1 MB = 1024 * 1024 字节）。
pub async fn calculate_size_mb(path: &Path) -> io::Result<f64> {
    let size_bytes = calculate_size_bytes(path).await?;
    Ok(size_bytes as f64 / BYTES_PER_MB as f64)
}

/// 流式计算安装包 SHA-256，避免一次性把整个文件读入内存。
pub async fn calculate_sha256(path: &Path) -> io::Result<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// 一次性计算安装包大小和 SHA-256。
pub async fn compute_package_info(path: &Path) -> io::Result<PackageInfo> {
    let size_bytes = calculate_size_bytes(path).await?;
    let sha256 = calculate_sha256(path).await?;

    Ok(PackageInfo {
        size_bytes,
        size_mb: size_bytes as f64 / BYTES_PER_MB as f64,
        sha256,
    })
}

/// 计算 JSON 中 `package_path` 指向的安装包，并写回 `size` 和 `hash`。
///
/// `package_path` 相对于 storage 根目录。
pub async fn sync_package_metadata(config_path: &Path) -> io::Result<PackageInfo> {
    let content = tokio::fs::read(config_path).await?;
    let mut config: serde_json::Value = serde_json::from_slice(&content)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    let package_path = config
        .get("package_path")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "JSON 缺少 package_path 字段"))?;

    let package_path = crate::paths::package_path(package_path);
    let info = compute_package_info(&package_path).await?;
    config["size"] = serde_json::json!(info.size_bytes);
    config["hash"] = serde_json::json!(info.sha256);

    let updated = serde_json::to_vec_pretty(&config)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    tokio::fs::write(config_path, updated).await?;

    Ok(info)
}
