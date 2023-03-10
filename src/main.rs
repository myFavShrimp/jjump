use std::fs::canonicalize;

use clap::Parser;
use eyre::Result;
use thiserror::Error;

mod cli;
mod config;

fn main() -> Result<()> {
    let cli_app = cli::Args::parse();

    let app_config = match config::create_and_read() {
        Ok(config) => config,
        Err(e) => panic!("{}", e.to_string()),
    };

    if let Some(command) = cli_app.subcommand.or(cli_app.goto.map(cli::Commands::Goto)) {
        match command {
            cli::Commands::Add(add_args) => add_entry(app_config, add_args),
            cli::Commands::Goto(goto_args) => goto_entry(app_config, goto_args),
            cli::Commands::Rem(rem_args) => remove_entry(app_config, rem_args),
        }?;
    };

    Ok(())
}

#[derive(Debug, Error)]
enum JJumpError {
    #[error("error with file path")]
    PathError(#[from] std::io::Error),
    #[error("error with config file")]
    ConfigError(#[from] config::ConfigError),
    #[error("could not find the portal in config")]
    PortalNotFoundError,
}

fn add_entry(mut config: config::Config, add_args: cli::CommandAdd) -> Result<(), JJumpError> {
    let path = canonicalize(&add_args.destination)?;

    let mut portal_names = config.portals.remove(&path)
        .map(|mut entries| {
            entries.extend(add_args.names.clone());
            entries
        })
        .or(Some(add_args.names))
        .unwrap();

    portal_names.sort();
    portal_names.dedup();

    config.portals.insert(path, portal_names);
    config::write(config)?;
    Ok(())
}

fn goto_entry(config: config::Config, goto_args: cli::CommandGoto) -> Result<(), JJumpError> {
    let portal_name = goto_args.name;

    for (path, portal_names) in config.portals {
        if !portal_names.contains(&portal_name) {
            continue;
        }

        println!("{:?}", path);
        break;
    }

    Ok(())
}

fn remove_entry(mut config: config::Config, rem_args: cli::CommandRemove) -> Result<(), JJumpError> {
    let portal_name = rem_args.name;

    return match config.portals.clone().iter().find(|(.., portal_names)| portal_names.contains(&portal_name)) {
        Some((path, ..)) => {
            if config.portals.remove(path).is_some() {
                config::write(dbg!(config))?;
                Ok(())
            } else {
                Err(JJumpError::PortalNotFoundError)
            }},
        None => Err(JJumpError::PortalNotFoundError),
    }
}
