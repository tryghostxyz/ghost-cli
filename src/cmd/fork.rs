use std::path::PathBuf;

use clap::{Parser, ValueHint};
use eyre::{eyre, OptionExt};

use crate::api_service::ApiService;
use crate::types::{ForkRequest, GraphConfig};
use crate::utils::{check_and_create_dir, write_sources_and_conf};

#[derive(Clone, Debug, Default, Parser)]
pub struct ForkCmd {
    /// Fork into this directory
    #[arg(
        value_hint = ValueHint::DirPath,
        value_name = "PATH",
        default_value = "."
    )]
    pub dir: PathBuf,

    /// ID of GhostGraph to fork
    #[arg(long)]
    pub id: Option<String>,

    /// Name for this GhostGraph. (defaults to dir name if not provided)
    #[arg(long, short)]
    pub name: Option<String>,

    /// Fork a graph and replace the current config and source files.
    #[arg(long, short)]
    pub replace: bool,
}

impl ForkCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let (dir, id) = if self.replace {
            let conf = GraphConfig::read(PathBuf::from("config.json"))
                .map_err(|_| eyre!("cannot read config.json. Must be in a Ghost directory"))?;
            (PathBuf::from("."), self.id.or(Some(conf.id)))
        } else {
            check_and_create_dir(&self.dir)?;
            (self.dir, self.id)
        };

        let id = id.ok_or_eyre("must pass --id if not --replace")?;
        println!("Forking graph with ID: {}", id);
        let name = self
            .name
            .or_else(|| dir.file_name().and_then(|os_str| os_str.to_str()).map(String::from));

        let resp = api.fork_graph(&id, &ForkRequest { name }).await?;
        println!("Graph has been successfully forked. Setting up local files...");

        write_sources_and_conf(&dir, resp.id, resp.version_id, None, resp.sources)?;
        println!("done! Check the {:?} directory", dir);

        Ok(())
    }
}
