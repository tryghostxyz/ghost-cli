use crate::types::Chain;
use crate::utils::cache_path;
use alloy_json_abi::JsonAbi;
use alloy_primitives::Address;
use foundry_block_explorers::Client;
use std::time::Duration;

const MAX_PROXY_REDIRECTS: usize = 3;

pub struct EtherscanClient {
    client: Client,
}

impl EtherscanClient {
    pub fn new(chain: Chain) -> eyre::Result<Self> {
        let mut client = Client::new_from_env(chain.alloy())?;
        client.set_cache("data/etherscan_cache", Duration::from_secs(3600));
        if let Some(cache) = cache_path() {
            client.set_cache(cache, Duration::from_secs(3600));
        }
        Ok(Self { client })
    }

    pub async fn fetch_abi(&self, address: Address) -> eyre::Result<JsonAbi> {
        let mut current_address = address;

        for _ in 0..MAX_PROXY_REDIRECTS {
            let code = self.client.contract_source_code(current_address).await?;
            let item = code.items.first().ok_or_else(|| eyre::eyre!("No item found"))?;

            if let Some(implementation_address) = item.implementation {
                current_address = implementation_address;
            } else {
                return Ok(item.abi()?);
            }
        }

        Err(eyre::eyre!("ABI not found after 3 redirects"))
    }
}
