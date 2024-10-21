use std::env;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

use api_service::ApiService;

use crate::cmd::{CodegenCmd, CompileCmd, CreateCmd, DeployCmd, EventsCommand, ForkCmd, ListCmd};
use crate::utils::install_handler;

mod abi_processor;
mod api_service;
mod cmd;
mod configure;
mod constants;
mod etherscan_client;
mod types;
mod utils;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Ghost CLI - Interact with the GhostGraph API",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Configure the Ghost API key")]
    Configure {
        #[arg(help = "Set the Ghost API key")]
        api_key: String,
    },

    #[command(about = "Create a new graph")]
    Create(CreateCmd),

    #[command(about = "Run codegen for an existing graph")]
    Codegen(CodegenCmd),

    #[command(about = "Compile the graph")]
    Compile(CompileCmd),

    #[command(about = "Deploy the graph")]
    Deploy(DeployCmd),

    #[command(about = "List all my graphs")]
    List(ListCmd),

    #[command(about = "Fork a graph")]
    Fork(ForkCmd),

    #[command(about = "Fetch events from contract ABI")]
    Events(EventsCommand),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let enable = yansi::Condition::os_support();
    yansi::whenever(yansi::Condition::cached(enable));
    unsafe {
        install_handler();
    }

    let cli = Cli::parse();

    if let Some(Commands::Configure { api_key }) = &cli.command {
        configure::set_api_key(api_key)?;
        return Ok(());
    }

    let api_key = match configure::get_api_key() {
        Ok(key) => key,
        Err(e) => {
            eprintln!(
                "Failed to retrieve API key: {}. Please run the 'configure' command first.",
                e
            );
            return Ok(());
        }
    };

    let base_url =
        env::var("GHOST_BASE_URL").unwrap_or_else(|_| "https://api.ghostlogs.xyz".to_string());
    let web_base_url =
        env::var("GHOST_WEB_BASE_URL").unwrap_or_else(|_| "https://app.ghostlogs.xyz".to_string());
    let api_service = ApiService::new(base_url, api_key, web_base_url);

    match cli.command {
        Some(Commands::Create(cmd)) => {
            cmd.run(&api_service).await?;
        }
        Some(Commands::Codegen(cmd)) => {
            cmd.run(&api_service).await?;
        }
        Some(Commands::Compile(cmd)) => {
            cmd.run(&api_service).await?;
        }
        Some(Commands::Deploy(cmd)) => {
            cmd.run(&api_service).await?;
        }
        Some(Commands::List(cmd)) => {
            cmd.run(&api_service).await?;
        }
        Some(Commands::Fork(cmd)) => {
            cmd.run(&api_service).await?;
        }
        Some(Commands::Events(cmd)) => {
            cmd.run(&api_service).await?;
        }
        _ => {}
    }

    Ok(())
}
