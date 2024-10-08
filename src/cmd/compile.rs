use std::fs;
use std::path::PathBuf;

use clap::Parser;
use eyre::eyre;

use crate::api_service::ApiService;
use crate::types::CompileRequest;
use crate::utils::{check_and_get_conf, write_files};

#[derive(Clone, Debug, Default, Parser)]
pub struct CompileCmd {}

impl CompileCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let config = check_and_get_conf(&["src/indexer.sol"])?;
        println!("Running compile for id={}", config.version_id);

        let payload = CompileRequest { indexer_code: fs::read_to_string("src/indexer.sol")? };
        let resp = api.compile(&config.version_id, &payload).await?;
        if let Some(err) = resp.err {
            return Err(eyre!(err));
        }
        if let Some(version) = resp.version {
            write_files(&PathBuf::from("."), version.sources)?;
            println!("Successfully compiled. Go ahead and run `ghost deploy` to deploy the graph")
        }
        Ok(())
    }
}
