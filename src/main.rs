use anyhow::Context;
use clap::Parser;
use cmd::{Args, RepoCommand};
use sha2::Digest;
use util::get_pluggy_dir;

mod cmd;
mod config;
mod git;
mod types;
mod util;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args {
        Args::Official(cmd) => handle_official(cmd)?,
        Args::Custom(cmd) => handle_custom(cmd)?,
    }

    Ok(())
}

fn handle_official(cmd: RepoCommand) -> anyhow::Result<()> {
    match cmd {
        RepoCommand::Init => {
            config::oobe()?;
            println!("Done!");
            Ok(())
        }

        RepoCommand::Add {
            name,
            remote,
            path,
            branch,
            commit,
            testing,
            track,
            owners,
        } => {
            let config = util::check_config_fulfilled()?;

            let path = path.unwrap_or_else(|| name.clone());

            let hash = format!("{:x}", sha2::Sha256::digest(remote.as_bytes()));
            let dir = git::ensure_clone(&hash, &remote)?;

            let branch = git::calc_branch(&dir, branch)?;
            git::switch_branch(&dir, &branch, true)?;

            let commit = if let Some(commit) = commit {
                commit
            } else {
                git::get_latest_commit(&dir, &branch)?
            };
            git::checkout_commit(&dir, &commit)?;

            let build = util::build(&dir, &path)?;
            let plugin_manifest_path = build.join(&name).join(format!("{}.json", &name));
            let plugin_manifest_str = std::fs::read_to_string(plugin_manifest_path)?;
            let plugin_manifest: types::PluginManifest =
                serde_json::from_str(&plugin_manifest_str)?;

            let plugin_name = plugin_manifest.name;
            let version = plugin_manifest.assembly_version;
            let icon_url = plugin_manifest.icon_url;

            let default_author = config.default_author.context("No default author set")?;
            let owners = owners.unwrap_or_else(|| vec![default_author]);
            let manifest = types::D17Manifest {
                plugin: types::D17ManifestPlugin {
                    repository: remote,
                    project_path: path,
                    commit,
                    owners,
                },
            };

            let track = util::calc_track(testing, track);
            let branch_name = format!("{}-{}-{}", track, name, version);
            let commit_name = if track != *"stable" {
                format!("[{}] {} {}", track, plugin_name, version)
            } else {
                format!("{} {}", plugin_name, version)
            };

            let dp17 = get_pluggy_dir()?.join("dp17_upstream");
            git::switch_branch(&dp17, "main", false)?;

            git::create_branch(&dp17, &branch_name)?;
            let track_dir = dp17.join(track).join(name);
            util::write_manifest(&track_dir, &manifest, icon_url)?;

            git::commit(&dp17, &commit_name)?;

            Ok(())
        }

        RepoCommand::Push { name } => {
            let name = name.context("Name is required on official repository")?;
            let dp17 = get_pluggy_dir()?.join("dp17_upstream");
            let branches = git::list_branches(&dp17)?;

            for branch in branches {
                if branch.contains(&name) {
                    git::push(&dp17, "fork", &branch)?;
                }
            }

            Ok(())
        }
    }
}

fn handle_custom(_cmd: RepoCommand) -> anyhow::Result<()> {
    todo!()
}
