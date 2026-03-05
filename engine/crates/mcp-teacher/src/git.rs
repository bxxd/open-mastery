use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn status(work_dir: &Path) -> Result<String> {
    let output = Command::new("git")
        .args(["status", "--short"])
        .current_dir(work_dir)
        .output()
        .context("Failed to run git status")?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn commit(work_dir: &Path, message: &str) -> Result<String> {
    Command::new("git")
        .args(["add", "."])
        .current_dir(work_dir)
        .output()
        .context("Failed to run git add")?;

    let output = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(work_dir)
        .output()
        .context("Failed to run git commit")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if output.status.success() {
        Ok(stdout)
    } else {
        Ok(format!("{}{}", stdout, stderr))
    }
}

pub fn log(work_dir: &Path, count: usize) -> Result<String> {
    let output = Command::new("git")
        .args(["log", "--oneline", &format!("-{}", count)])
        .current_dir(work_dir)
        .output()
        .context("Failed to run git log")?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
