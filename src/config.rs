use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{
    git::{git_clone, run_git},
    util::get_pluggy_dir,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub git_command: String,
    pub dotnet_command: String,
    pub dp17_upstream: Option<String>,
    pub dp17_fork: Option<String>,
    pub sign_commits: bool,
    pub default_author: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git_command: "git".to_string(),
            dotnet_command: "dotnet".to_string(),
            dp17_upstream: None,
            dp17_fork: None,
            sign_commits: false,
            default_author: None,
        }
    }
}

pub fn get_pluggy_config() -> anyhow::Result<Config> {
    let path = get_pluggy_dir()?.join("config.toml");

    if !path.exists() {
        let default_config = toml::to_string_pretty(&Config::default())
            .context("Failed to serialize default config")?;
        std::fs::write(&path, default_config).context("Failed to write default config")?;
    }

    let str = std::fs::read_to_string(&path).context("Failed to read config file")?;
    let config = toml::from_str(&str).context("Failed to parse config file")?;
    Ok(config)
}

pub fn write_pluggy_config(config: &Config) -> anyhow::Result<()> {
    let path = get_pluggy_dir()?.join("config.toml");
    let str = toml::to_string_pretty(config).context("Failed to serialize config")?;
    std::fs::write(path, str).context("Failed to write config file")?;
    Ok(())
}

pub fn oobe() -> anyhow::Result<()> {
    let dir = get_pluggy_dir()?;
    let mut config = get_pluggy_config()?;

    let run_config_oobe = if config.dp17_upstream.is_some() {
        inquire::Confirm::new("You already set up your config file - redo it?")
            .with_default(false)
            .prompt()?
    } else {
        true
    };

    if run_config_oobe {
        oobe_config(&mut config)?;
    }

    let upstream_dir = dir.join("dp17_upstream");
    let already_exists = upstream_dir.exists();

    let run_repo_oobe = if already_exists {
        inquire::Confirm::new("You already cloned the repositories - redo it? This will delete the existing repository!")
            .with_default(false)
            .prompt()?
    } else {
        true
    };

    if run_repo_oobe {
        if already_exists {
            println!("Removing old repository...");
            std::fs::remove_dir_all(&upstream_dir)
                .context("Failed to remove upstream directory")?;
        }

        oobe_repo(
            &upstream_dir,
            config.dp17_upstream.as_ref().unwrap(),
            config.dp17_fork.as_ref().unwrap(),
        )?;
    }

    Ok(())
}

fn oobe_config(config: &mut Config) -> anyhow::Result<()> {
    let username = inquire::Text::new("GitHub username:").prompt()?;
    let use_ssh = inquire::Confirm::new("Use Git over SSH?")
        .with_default(true)
        .prompt()?;
    let sign_commits = inquire::Confirm::new("Sign Git commits?")
        .with_default(false)
        .prompt()?;

    let upstream = if use_ssh {
        "git@github.com:goatcorp/DalamudPluginsD17.git"
    } else {
        "https://github.com/goatcorp/DalamudPluginsD17.git"
    };

    let fork_default = if use_ssh {
        format!("git@github.com:{}/DalamudPluginsD17.git", username)
    } else {
        format!("https://github.com/{}/DalamudPluginsD17.git", username)
    };

    let fork = inquire::Text::new("Fork remote:")
        .with_default(&fork_default)
        .prompt()?;

    println!("Writing config file...");
    config.default_author = Some(username);
    config.dp17_upstream = Some(upstream.to_string());
    config.dp17_fork = Some(fork);
    config.sign_commits = sign_commits;
    write_pluggy_config(config)?;

    Ok(())
}

fn oobe_repo(upstream_dir: &Path, upstream: &str, fork: &str) -> anyhow::Result<()> {
    let pluggy_dir = get_pluggy_dir()?;
    println!("Cloning upstream repository...");
    git_clone(upstream, upstream_dir, &pluggy_dir)?;
    run_git(upstream_dir, &["remote", "add", "fork", fork])?;

    Ok(())
}
