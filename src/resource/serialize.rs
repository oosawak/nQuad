use super::data::ResourcePackage;
use bincode;
use std::path::Path;

/// ResourcePackageをバイナリ形式で保存
pub fn save_package(pkg: &ResourcePackage, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let encoded = bincode::serialize(pkg)?;
    std::fs::write(path, encoded)?;
    Ok(())
}

/// バイナリ形式のResourcePackageを読み込む
pub fn load_package(path: &Path) -> Result<ResourcePackage, Box<dyn std::error::Error>> {
    let bytes = std::fs::read(path)?;
    let pkg = bincode::deserialize(&bytes)?;
    Ok(pkg)
}

/// 正常性チェック付きでロード
pub fn load_package_safe(path: &Path) -> Result<ResourcePackage, String> {
    load_package(path).map_err(|e| format!("Failed to load package from {:?}: {}", path, e))
}
