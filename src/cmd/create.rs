use std::path::PathBuf;

use clap::{Parser, ValueHint};

use crate::api_service::ApiService;
use crate::types::{Chain, CreateRequest};
use crate::utils::{check_and_create_dir, write_sources_and_conf};

#[derive(Clone, Debug, Default, Parser)]
pub struct CreateCmd {
    /// The directory for the new GhostGraph.
    #[arg(value_hint = ValueHint::DirPath, value_name = "PATH")]
    pub dir: PathBuf,
    /// Chain
    #[arg(
        long,
        short,
        help = "Specify the chain. Options: eth-mainnet, eth-sepolia, base-mainnet, base-sepolia, bera-testnet, blast-mainnet, abstract-testnet, or chain id"
    )]
    pub chain: Chain,

    /// Name for this GhostGraph. (defaults to dir name if not provided)
    #[arg(long, short)]
    pub name: Option<String>,
}

impl CreateCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let Self { dir, chain, name } = self;
        check_and_create_dir(&dir)?;

        let name = name.unwrap_or_else(|| {
            dir.file_name()
                .and_then(|os_str| os_str.to_str())
                .map(String::from)
                .unwrap_or_else(|| "My Index".to_string())
        });
        let payload = CreateRequest { chain: chain.chain_id(), name };
        let resp = api.create_graph(payload).await?;
        println!("Success! Created a new graph");
        println!(
            "View online at {}/graphs/{}/versions/{}/editor",
            api.web_base_url(),
            resp.id,
            resp.version_id
        );
        println!("\nInitializing files...");
        write_sources_and_conf(&dir, resp.id, resp.version_id, resp.sources)?;
        println!("done! Check the {:?} directory", dir);

        Ok(())
    }
}
