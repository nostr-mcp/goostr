use clap::{Parser, Subcommand};

pub const DEFAULT_EXTENSION_ID: &str = "goostr";
pub const DEFAULT_EXTENSION_NAME: &str = "Goostr";
pub const DEFAULT_DISPLAY_NAME: &str = "Goostr";
pub const DEFAULT_DESCRIPTION: &str = "Connect Goose to the Nostr network";
pub const DEFAULT_TIMEOUT_SECS: u64 = 300;

#[derive(Parser, Debug)]
#[command(
    name = "goostr",
    version,
    about = "Codename Goose Nostr MCP extension",
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(aliases = ["stdio", "serve"])]
    Start,
    Install {
        #[arg(long, default_value_t = DEFAULT_EXTENSION_ID.to_string())]
        id: String,
        #[arg(long, default_value_t = DEFAULT_EXTENSION_NAME.to_string())]
        name: String,
        #[arg(long, default_value_t = DEFAULT_DISPLAY_NAME.to_string())]
        display_name: String,
        #[arg(long, default_value_t = DEFAULT_DESCRIPTION.to_string())]
        description: String,
        #[arg(long)]
        cmd: Option<String>,
        #[arg(long, default_value_t = true)]
        enabled: bool,
        #[arg(long, default_value_t = DEFAULT_TIMEOUT_SECS)]
        timeout: u64,
        #[arg(long, default_value_t = false)]
        bundled: bool,
        #[arg(long, value_delimiter = ',', default_values_t = Vec::<String>::new())]
        available_tools: Vec<String>,
        #[arg(long = "arg", value_delimiter = ',', default_values_t = Vec::<String>::new())]
        args: Vec<String>,
        #[arg(long = "env", value_delimiter = ',', default_values_t = Vec::<String>::new())]
        env: Vec<String>,
        #[arg(long = "env-key", value_delimiter = ',', default_values_t = Vec::<String>::new())]
        env_keys: Vec<String>,
    },
    Uninstall {
        #[arg(long, default_value_t = DEFAULT_EXTENSION_ID.to_string())]
        id: String,
    },
}
