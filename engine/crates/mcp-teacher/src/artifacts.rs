use anyhow::{Context, Result};
use base64::Engine as _;
use std::path::{Path, PathBuf};

fn artifact_dir(base_dir: &Path, scope: &str) -> PathBuf {
    base_dir.join("artifacts").join(scope)
}

pub fn save_artifact(base_dir: &Path, scope: &str, filename: &str, data_base64: &str) -> Result<PathBuf> {
    let dir = artifact_dir(base_dir, scope);
    std::fs::create_dir_all(&dir).context("Failed to create artifact directory")?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_base64)
        .context("Invalid base64 data")?;

    let path = dir.join(filename);
    std::fs::write(&path, &data)?;
    Ok(path)
}

pub fn get_artifact(base_dir: &Path, scope: &str, filename: &str) -> Result<String> {
    let path = artifact_dir(base_dir, scope).join(filename);
    let data = std::fs::read(&path).context(format!("Artifact not found: {}", path.display()))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&data))
}

pub fn list_artifacts(base_dir: &Path, scope: &str) -> Result<Vec<String>> {
    let dir = artifact_dir(base_dir, scope);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        if entry.path().is_file() {
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }
    }
    files.sort();
    Ok(files)
}
