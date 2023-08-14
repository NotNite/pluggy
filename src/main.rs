use clap::Parser;
use cmd::{Args, RepoCommand};

mod cmd;
mod config;
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
        _ => todo!(),
    }
}

fn handle_custom(cmd: RepoCommand) -> anyhow::Result<()> {
    todo!()
}
