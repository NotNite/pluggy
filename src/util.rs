use anyhow::Context;
use std::path::{Path, PathBuf};

fn get_default_pluggy_dir() -> anyhow::Result<PathBuf> {
    let home = home::home_dir().context("Failed to find home directory")?;
    let pluggy = home.join(".pluggy");
    Ok(pluggy)
}

pub fn get_pluggy_dir() -> anyhow::Result<PathBuf> {
    let pluggy = std::env::var("PLUGGY_HOME")
        .map(PathBuf::from)
        .unwrap_or(get_default_pluggy_dir().context("Failed to find pluggy directory")?);

    if !pluggy.exists() {
        std::fs::create_dir(&pluggy).context("Failed to create pluggy directory")?;
    }

    Ok(pluggy)
}

pub fn run_git(dir: &Path, args: &[&str]) -> anyhow::Result<()> {
    let git_cmd = std::env::var("PLUGGY_GIT_CMD").unwrap_or_else(|_| "git".to_string());
    let status = std::process::Command::new(git_cmd)
        .current_dir(dir)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .args(args)
        .status()
        .context("Failed to run git")?;

    if !status.success() {
        anyhow::bail!("git failed with status {}", status);
    }

    Ok(())
}

pub fn git_clone(url: &str, path: &Path, dir: &Path) -> anyhow::Result<()> {
    for i in 0..5 {
        let result = run_git(dir, &["clone", url, path.to_str().unwrap()]);
        if result.is_ok() {
            break;
        } else {
            if i == 4 {
                result?;
            }

            println!("Failed to clone, retrying...");
        }
    }

    Ok(())
}
