use std::env;
use std::path::{Path, PathBuf};

const STORAGE_DIR: &str = "storage";
const CONFIG_DIR: &str = "config";
const AICRM_CONFIG_FILE: &str = "aicrm.json";

/// Returns the storage root. An explicit environment override wins; otherwise
/// use the storage directory beside the executable, falling back to the
/// project directory while developing with `cargo run`.
pub fn storage_root() -> PathBuf {
    if let Ok(root) = env::var("AICRM_STORAGE_ROOT")
        && !root.trim().is_empty()
    {
        return PathBuf::from(root);
    }

    if let Ok(executable) = env::current_exe()
        && let Some(directory) = executable.parent()
    {
        let candidate = directory.join(STORAGE_DIR);
        if candidate.join(CONFIG_DIR).is_dir() {
            return candidate;
        }
    }

    if let Ok(directory) = env::current_dir() {
        let candidate = directory.join(STORAGE_DIR);
        if candidate.join(CONFIG_DIR).is_dir() {
            return candidate;
        }
    }

    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(STORAGE_DIR)
}

pub fn config_path() -> PathBuf {
    storage_root().join(CONFIG_DIR).join(AICRM_CONFIG_FILE)
}

/// Resolves a package path from the storage root. Absolute paths are kept
/// as-is so the storage location can still be overridden explicitly.
pub fn package_path(configured_path: &str) -> PathBuf {
    let path = Path::new(configured_path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        storage_root().join(path)
    }
}
