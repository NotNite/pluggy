use crate::{config::get_pluggy_config, util::get_pluggy_dir};
use anyhow::Context;
use std::path::{Path, PathBuf};

pub fn run_git(dir: &Path, args: &[&str]) -> anyhow::Result<()> {
    let config = get_pluggy_config()?;
    let status = std::process::Command::new(config.git_command)
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

pub fn run_git_with_output(dir: &Path, args: &[&str]) -> anyhow::Result<String> {
    let config = get_pluggy_config()?;
    let output = std::process::Command::new(config.git_command)
        .current_dir(dir)
        .args(args)
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
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

pub fn ensure_clone(hash: &str, remote: &str) -> anyhow::Result<PathBuf> {
    // sha256 folder name
    let dir = get_pluggy_dir()?.join("repos").join(hash);
    if dir.exists() {
        return Ok(dir);
    }

    std::fs::create_dir_all(&dir).context("Failed to create repo directory")?;
    git_clone(remote, &dir, &dir).context("Failed to clone repo")?;
    Ok(dir)
}

pub fn calc_branch(dir: &Path, branch: Option<String>) -> anyhow::Result<String> {
    let branch = if let Some(branch) = branch {
        branch
    } else {
        run_git_with_output(dir, &["rev-parse", "--abbrev-ref", "HEAD"])?
            .trim()
            .to_string()
    };
    Ok(branch)
}

pub fn switch_branch(dir: &Path, branch: &str, reset: bool) -> anyhow::Result<()> {
    println!("Switching to branch {}...", branch);
    run_git(dir, &["stash"])?;
    run_git(dir, &["fetch", "--all"])?;
    run_git(dir, &["checkout", branch])?;

    if reset {
        run_git(dir, &["reset", "--hard", &format!("origin/{}", branch)])?;
    }

    Ok(())
}

pub fn get_latest_commit(dir: &Path, branch: &str) -> anyhow::Result<String> {
    let output = run_git_with_output(dir, &["rev-parse", branch])?;
    Ok(output.trim().to_string())
}

pub fn checkout_commit(dir: &Path, commit: &str) -> anyhow::Result<()> {
    println!("Checking out commit {}...", commit);
    run_git(dir, &["checkout", commit])?;
    Ok(())
}

pub fn create_branch(dir: &Path, branch: &str) -> anyhow::Result<()> {
    let does_not_exist = run_git_with_output(
        dir,
        &["branch", "--list", "--format", "%(refname:short)", branch],
    )?
    .trim()
    .is_empty();

    if !does_not_exist {
        println!("Branch {} already exists, remaking it...", branch);
        run_git(dir, &["branch", "-D", branch])?;
    }

    println!("Creating branch {}...", branch);
    run_git(dir, &["checkout", "-b", branch])?;
    Ok(())
}

pub fn commit(dir: &Path, desc: &str) -> anyhow::Result<()> {
    let mut args = vec!["commit", "-m", desc];

    let config = get_pluggy_config()?;
    if config.sign_commits {
        args.push("-S");
    }

    run_git(dir, &["add", "-A"])?;
    run_git(dir, &args)?;
    Ok(())
}

pub fn list_branches(dir: &Path) -> anyhow::Result<Vec<String>> {
    let output = run_git_with_output(dir, &["branch", "--list", "--format", "%(refname:short)"])?;
    let branches = output
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(branches)
}

pub fn push(dir: &Path, remote: &str, branch: &str) -> anyhow::Result<()> {
    println!("Pushing to {}...", remote);
    switch_branch(dir, branch, false)?;
    run_git(dir, &["push", remote, branch])?;
    Ok(())
}

pub fn pull(dir: &Path) -> anyhow::Result<()> {
    run_git(dir, &["pull"])?;
    Ok(())
}
