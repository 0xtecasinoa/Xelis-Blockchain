pub mod transaction_builder;
pub mod storage;
pub mod wallet;
pub mod config;
pub mod account;
pub mod cipher;

use std::{sync::Arc, time::Duration, path::Path};

use anyhow::Result;
use config::DIR_PATH;
use fern::colors::Color;
use log::{error, info};
use clap::Parser;
use xelis_common::{config::{
    DEFAULT_DAEMON_ADDRESS,
    VERSION
}, prompt::{Prompt, command::CommandManager}};
use wallet::Wallet;


#[derive(Parser)]
#[clap(version = VERSION, about = "XELIS Wallet")]
pub struct Config {
    /// Daemon address to use
    #[clap(short = 'a', long, default_value_t = String::from(DEFAULT_DAEMON_ADDRESS))]
    daemon_address: String,
    /// Enable the debug mode
    #[clap(short, long)]
    debug: bool,
    /// Disable the log file
    #[clap(short = 'f', long)]
    disable_file_logging: bool,
    /// Log filename
    #[clap(short = 'l', long, default_value_t = String::from("xelis-wallet.log"))]
    filename_log: String,
    /// Set name path for wallet storage
    #[clap(short, long)]
    name: String,
    /// Password used to open wallet
    #[clap(short, long)]
    password: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::parse();
    let prompt = Prompt::new(config.debug, config.filename_log, config.disable_file_logging)?;
    let dir = format!("{}{}", DIR_PATH, config.name);

    let wallet = if Path::new(&dir).is_dir() {
        info!("Opening wallet {}", dir);
        Wallet::open(dir, config.password, config.daemon_address)?
    } else {
        info!("Creating a new wallet at {}", dir);
        Wallet::new(dir, config.password, config.daemon_address)?
    };

    if let Err(e) = run_prompt(prompt).await {
        error!("Error while running prompt: {}", e);
    }

    Ok(())
}

async fn run_prompt(prompt: Arc<Prompt>) -> Result<()> {
    let command_manager: CommandManager<()> = CommandManager::default();
    let closure = || async {
        let height_str = format!("{}/{}", 0, 0); // TODO
        let status = Prompt::colorize_str(Color::Red, "Offline");
        format!(
            "{} | {} | {} | {} ",
            Prompt::colorize_str(Color::Blue, "XELIS Wallet"),
            height_str,
            status,
            Prompt::colorize_str(Color::BrightBlack, ">>")
        )
    };
    prompt.start(Duration::from_millis(100), &closure, command_manager).await?;
    Ok(())
}