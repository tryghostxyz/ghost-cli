use std::path::PathBuf;

use clap::{Parser, ValueHint};
use eyre::eyre;

use crate::api_service::ApiService;
use crate::types::{ForkRequest, GraphConfig};
use crate::utils::{check_and_create_dir, write_sources_and_conf};

#[derive(Clone, Debug, Default, Parser)]
pub struct ForkCmd {
    /// Fork into this directory
    #[arg(value_hint = ValueHint::DirPath, value_name = "PATH")]
    pub dir: PathBuf,

    /// ID of GhostGraph to fork
    #[arg(long)]
    pub id: String,

    /// Name for this GhostGraph. (defaults to dir name if not provided)
    #[arg(long, short)]
    pub name: Option<String>,

    /// Fork a graph and replace the current config and source files.
    #[arg(long, short)]
    pub replace: bool,
}

impl ForkCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let Self { mut dir, id, name, replace } = self;
        println!("Forking graph with ID: {}", id);
        if replace {
            GraphConfig::read(PathBuf::from("config.json"))
                .map_err(|_| eyre!("cannot read config.json. Must be in a Ghost directory"))?;
            dir = PathBuf::from(".");
        } else {
            check_and_create_dir(&dir)?;
        }
        let name = name
            .or_else(|| dir.file_name().and_then(|os_str| os_str.to_str()).map(|s| s.to_string()));
        let resp = api.fork_graph(&id, &ForkRequest { name }).await?;
        println!("Graph has been successfully forked. Setting up local files...");
        write_sources_and_conf(&dir, resp.id, resp.version_id, None, resp.sources)?;
        println!("done! Check the {:?} directory", dir);
        Ok(())
    }
}
