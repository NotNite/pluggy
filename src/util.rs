use anyhow::Context;
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

use crate::config::{get_pluggy_config, Config};

fn get_default_pluggy_dir() -> anyhow::Result<PathBuf> {
    Ok(ProjectDirs::from("com", "notnite", "pluggy")
        .context("Failed to setup project directory")?
        .config_dir()
        .to_path_buf())
}

pub fn get_pluggy_dir() -> anyhow::Result<PathBuf> {
    let pluggy = std::env::var("PLUGGY_HOME")
        .map(PathBuf::from)
        .unwrap_or(get_default_pluggy_dir().context("Failed to find pluggy directory")?);

    if !pluggy.exists() {
        std::fs::create_dir_all(&pluggy).context("Failed to create pluggy directory")?;
    }

    Ok(pluggy)
}

pub fn calc_track(testing: bool, track: Option<String>) -> String {
    if let Some(track) = track {
        track
    } else if testing {
        "testing/live".to_string()
    } else {
        "stable".to_string()
    }
}

pub fn guarantee_temp() -> anyhow::Result<PathBuf> {
    let temp = get_pluggy_dir()?.join("temp");
    if temp.exists() {
        std::fs::remove_dir_all(&temp).context("Failed to remove temp directory")?;
    }
    std::fs::create_dir(&temp).context("Failed to create temp directory")?;
    Ok(temp)
}

pub fn build(dir: &Path, path: &str) -> anyhow::Result<PathBuf> {
    let config = get_pluggy_config()?;
    let temp = guarantee_temp()?;
    let temp_str = temp.to_str().unwrap();

    let arg = std::process::Command::new(config.dotnet_command)
        .current_dir(dir)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .args(["build", path, "-c", "Release", "-o", temp_str])
        .status()?;

    if !arg.success() {
        anyhow::bail!("Failed to build project");
    }

    Ok(temp)
}

pub fn check_config_fulfilled() -> anyhow::Result<Config> {
    let config = get_pluggy_config()?;
    if config.default_author.is_none() {
        anyhow::bail!("Configuration not generated")
    } else {
        Ok(config)
    }
}

pub fn write_manifest(
    dir: &Path,
    manifest: &crate::types::D17Manifest,
    _icon_url: Option<String>,
) -> anyhow::Result<()> {
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }

    let manifest_str = toml::to_string(manifest)?;
    std::fs::write(dir.join("manifest.toml"), manifest_str)?;
    // TODO icon url
    Ok(())
}
