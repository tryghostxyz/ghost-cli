use clap::Parser;
use eyre::eyre;

use crate::api_service::ApiService;
use crate::utils::check_and_get_conf;

#[derive(Clone, Debug, Default, Parser)]
pub struct DeployCmd {}

impl DeployCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let config = check_and_get_conf(&[], api).await?;
        println!("Running deploy for id={}", config.version_id);

        let resp = api.deploy(&config.version_id).await?;
        if let Some(err) = resp.err {
            return Err(eyre!(err));
        }
        if let Some(_ok) = resp.ok {
            println!("Successfully deployed. \n");
            println!(
                "View online at {}/graphs/{}/versions/{}/editor",
                api.web_base_url(),
                config.id,
                config.version_id
            );
        }
        Ok(())
    }
}
