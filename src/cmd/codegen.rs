use std::fs;
use std::path::PathBuf;

use clap::Parser;
use eyre::eyre;

use crate::api_service::ApiService;
use crate::types::CodegenRequest;
use crate::utils::{check_and_get_conf, write_files};

#[derive(Clone, Debug, Default, Parser)]
pub struct CodegenCmd {}

impl CodegenCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let config = check_and_get_conf(&["src/schema.sol", "src/events.sol"])?;
        println!("Running codegen for id={}", config.version_id);

        let payload = CodegenRequest {
            schema_code: fs::read_to_string("src/schema.sol")?,
            events_code: fs::read_to_string("src/events.sol")?,
        };
        let resp = api.codegen(&config.version_id, &payload).await?;
        if let Some(err) = resp.err {
            return Err(eyre!(err));
        }
        if let Some(version) = resp.version {
            write_files(&PathBuf::from("."), version.sources)?;
            println!(
                "All files saved. Go ahead and modify indexer.sol and then run `ghost compile`"
            )
        }
        Ok(())
    }
}
