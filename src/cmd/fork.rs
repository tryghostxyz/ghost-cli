use crate::api_service::ApiService;
use crate::types::ForkRequest;
use crate::utils::{check_and_create_dir, write_sources_and_conf};
use clap::{Parser, ValueHint};
use std::path::PathBuf;

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
}

impl ForkCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        let Self { dir, id, name } = self;
        println!("Forking graph with ID: {}", id);
        check_and_create_dir(&dir)?;
        let name: Option<String> = name
            .or_else(|| dir.file_name().and_then(|os_str| os_str.to_str()).map(|s| s.to_string()));
        let resp = api.fork_graph(&id, &ForkRequest { name }).await?;
        println!("Graph has been successfully forked. Setting up local files...");
        write_sources_and_conf(&dir, resp.id, resp.version_id, resp.sources)?;
        println!("done! Check the {:?} directory", dir);
        Ok(())
    }
}
