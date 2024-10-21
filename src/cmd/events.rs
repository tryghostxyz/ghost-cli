use crate::abi_processor::process_events;
use crate::api_service::ApiService;
use crate::etherscan_client::EtherscanClient;
use crate::utils::check_and_get_conf;
use alloy_json_abi::Event;
use alloy_primitives::Address;
use clap::Parser;
use eyre::OptionExt;
use std::env;

#[derive(Clone, Debug, Default, Parser)]
pub struct EventsCommand {
    #[arg(long, short, env = "ETHERSCAN_API_KEY", help = "etherscan key for the target chain")]
    pub api_key: String,

    #[arg(long, short)]
    pub address: Address,
}

impl EventsCommand {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let config = check_and_get_conf(&[], api).await?;
        let chain = config.chain.ok_or_eyre("no chain found")?;
        env::set_var("ETHERSCAN_API_KEY", self.api_key);
        let client = EtherscanClient::new(chain)?;
        let abi = client.fetch_abi(self.address).await?;
        let ev: Vec<Event> =
            abi.events.values().flat_map(|events| events.iter().cloned()).collect();
        if ev.is_empty() {
            println!("No events found for {}", self.address);
            return Ok(());
        }

        let res = process_events(&ev);
        for s in res.0 {
            println!("{}", s);
        }
        println!("events {{");
        for ev in res.1 {
            println!("\t{}", ev);
        }
        println!("}}");
        println!("\n\nYou can copy the relevant events into your events.sol file");
        Ok(())
    }
}
