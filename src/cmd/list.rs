use chrono::{DateTime, Local};
use clap::Parser;
use eyre::eyre;
use std::collections::HashMap;
use toolstr::{Color, ColumnFormatShorthand, Table, TableFormat};

use crate::api_service::ApiService;
use crate::constants::CHAIN_NAMES;

const DEFAULT_COLOR_TITLE: Color = Color::TrueColor { r: 206, g: 147, b: 249 };
const DEFAULT_COLOR_COMMENT: Color = Color::TrueColor { r: 98, g: 114, b: 164 };

#[derive(Clone, Debug, Default, Parser)]
pub struct ListCmd {}

impl ListCmd {
    pub async fn run(self, api: &ApiService) -> eyre::Result<()> {
        println!("Fetching list of graphs...");

        let resp = api.get_graphs().await?;

        if let Some(err) = resp.err {
            return Err(eyre!(err));
        }

        if resp.graphs.is_empty() {
            println!("No graphs found.");
            return Ok(());
        }

        let chain_map: HashMap<u64, &str> =
            CHAIN_NAMES.iter().map(|&(name, id)| (id, name)).collect();

        let mut table = Table::new();
        let mut format = TableFormat::default()
            .border_font_style(DEFAULT_COLOR_COMMENT)
            .label_font_style(DEFAULT_COLOR_TITLE);
        //.padding_left(1)
        //.padding_right(1);

        // Prepare data
        let data: Vec<(String, String, String, String, String)> = resp
            .graphs
            .into_iter()
            .map(|graph| {
                let chain_name = chain_map.get(&graph.chain).copied().unwrap_or("Unknown");
                (
                    graph.latest_version_id.to_string(),
                    format!("{:<20}", graph.name.trim()), // Manually left-align and set width
                    graph.description.unwrap_or_else(|| "--".to_string()),
                    format!("{} ({})", chain_name, graph.chain),
                    fmt_time(&graph.created_at).unwrap_or(graph.created_at),
                )
            })
            .collect::<Vec<_>>();

        // Add columns
        table.add_column("ID", data.iter().map(|d| d.0.clone()).collect::<Vec<_>>())?;
        table.add_column("Name", data.iter().map(|d| d.1.clone()).collect::<Vec<_>>())?;
        table.add_column("Description", data.iter().map(|d| d.2.clone()).collect::<Vec<_>>())?;
        table.add_column("Chain", data.iter().map(|d| d.3.clone()).collect::<Vec<_>>())?;
        table.add_column("Created", data.iter().map(|d| d.4.clone()).collect::<Vec<_>>())?;

        // Configure column formats
        format.add_column(ColumnFormatShorthand::new().name("ID"));
        format.add_column(ColumnFormatShorthand::new().name("Name").left_justify().max_width(24));
        format.add_column(
            ColumnFormatShorthand::new().name("Description").left_justify().max_width(30),
        );
        format.add_column(ColumnFormatShorthand::new().name("Chain").left_justify());
        format.add_column(ColumnFormatShorthand::new().name("Created"));

        // Print the table
        format.print(table)?;

        Ok(())
    }
}

fn fmt_time(dt: &str) -> Option<String> {
    let utc_time = DateTime::parse_from_rfc3339(dt).ok()?;
    let local_time = utc_time.with_timezone(&Local);
    Some(local_time.format("%Y-%m-%d %H:%M").to_string())
}
