use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use tracing::info;

use goostr::{
    cli::{Cli, Command},
    config, logging, server, util,
};

#[tokio::main]
async fn main() -> Result<()> {
    logging::init();
    let _ = util::ensure_keystore_secret();
    info!("goostr runtime starting");

    let cli = Cli::parse();

    match cli.command {
        None | Some(Command::Start) => {
            server::start_stdio_server().await?;
        }
        Some(Command::Install {
            id,
            name,
            display_name,
            description,
            cmd,
            enabled,
            timeout,
            bundled,
            available_tools,
            args,
            env,
            env_keys,
        }) => {
            let exe = resolve_exe(cmd, "goostr");
            let envs = parse_env_pairs(&env);
            config::upsert_stdio_extension(
                &id,
                &name,
                &display_name,
                &description,
                &exe,
                enabled,
                timeout,
                bundled,
                &available_tools,
                &args,
                &envs,
                &env_keys,
            )?;
            println!("Installed or updated Goose extension '{}'", id);
            println!("Config path: {}", config::path().display());
        }
        Some(Command::Uninstall { id }) => {
            let removed = config::remove_extension(&id)?;
            if removed {
                println!("Removed Goose extension '{}'", id);
            } else {
                println!("No Goose extension '{}' found", id);
            }
            println!("Config path: {}", config::path().display());
        }
    }

    info!("goostr runtime stopped");
    Ok(())
}

fn resolve_exe(cmd: Option<String>, fallback_name: &str) -> String {
    if let Some(c) = cmd {
        return c;
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| fallback_name.to_string())
}

fn parse_env_pairs(pairs: &[String]) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for p in pairs {
        if let Some((k, v)) = p.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    map
}
