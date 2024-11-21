use clap::Parser;

use crate::api_service::ApiService;

#[derive(Clone, Debug, Default, Parser)]
pub struct DeleteCmd {
    /// ID of GhostGraph to delete
    #[arg(long)]
    pub id: String,
}

impl DeleteCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        println!("Deleting graph with ID: {}", self.id);
        api.delete_graph(&self.id).await?;
        println!("Successfully deleted the graph");
        Ok(())
    }
}
