use std::{fmt, fs};
use std::path::PathBuf;
use std::str::FromStr;

use alloy_chains::Chain as AlloyChain;
use serde::{Deserialize, Serialize};

use crate::constants::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Chain {
    EthMainnet,
    EthSepolia,
    BaseMainnet,
    BaseSepolia,
    BeraTestnet,
    BlastMainnet,
    AbstractTestnet,
    UniTestnet,
}

impl Chain {
    pub fn chain_id(&self) -> u64 {
        match self {
            Chain::EthMainnet => CHAIN_ETH,
            Chain::EthSepolia => CHAIN_ETH_SEPOLIA,
            Chain::BaseMainnet => CHAIN_BASE,
            Chain::BaseSepolia => CHAIN_BASE_TESTNET,
            Chain::BeraTestnet => CHAIN_BERA_TESTNET,
            Chain::BlastMainnet => CHAIN_BLAST,
            Chain::AbstractTestnet => CHAIN_ABS_TESTNET,
            Chain::UniTestnet => CHAIN_UNI_TESTNET,
        }
    }

    pub fn alloy(&self) -> AlloyChain {
        AlloyChain::from_id(self.chain_id())
    }

    pub fn options() -> Vec<&'static str> {
        vec![
            "ethereum",
            "sepolia",
            "base",
            "base-testnet",
            "bera",
            "blast",
            "abstract",
            "uni-testnet",
        ]
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self::EthMainnet
    }
}

impl TryFrom<u64> for Chain {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            CHAIN_ETH => Ok(Self::EthMainnet),
            CHAIN_ETH_SEPOLIA => Ok(Self::EthSepolia),
            CHAIN_BASE => Ok(Self::BaseMainnet),
            CHAIN_BASE_TESTNET => Ok(Self::BaseSepolia),
            CHAIN_BERA_TESTNET => Ok(Self::BeraTestnet),
            CHAIN_BLAST => Ok(Self::BlastMainnet),
            CHAIN_ABS_TESTNET => Ok(Self::AbstractTestnet),
            CHAIN_UNI_TESTNET => Ok(Self::UniTestnet),
            _ => Err(format!("Unsupported chain id: {}", value)),
        }
    }
}

impl FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(chain_id) = s.parse::<u64>() {
            Chain::try_from(chain_id)
        } else {
            let lowercase = s.to_lowercase();
            match lowercase.as_str() {
                "eth-mainnet" | "ethereum" | "eth" => Ok(Self::EthMainnet),
                "eth-sepolia" | "sepolia" => Ok(Self::EthSepolia),
                "base-mainnet" | "base" => Ok(Self::BaseMainnet),
                "base-sepolia" | "base-testnet" => Ok(Self::BaseSepolia),
                "bera-testnet" | "bera" => Ok(Self::BeraTestnet),
                "blast-mainnet" | "blast" => Ok(Self::BlastMainnet),
                "abstract-testnet" | "abstract" => Ok(Self::AbstractTestnet),
                "uni-testnet" => Ok(Self::UniTestnet),
                _ => Err(format!(
                    "Unsupported chain name: {}. Valid options are: {}",
                    s,
                    Chain::options().join(", ")
                )),
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphConfig {
    pub id: String,
    pub version_id: String,
    pub chain: Option<Chain>,
}

impl GraphConfig {
    pub fn read(path: PathBuf) -> eyre::Result<Self> {
        let file_contents = fs::read_to_string(path)?;
        let config: GraphConfig = serde_json::from_str(&file_contents)?;
        Ok(config)
    }

    pub fn write(&self, path: PathBuf) -> eyre::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CreateRequest {
    pub name: String,
    pub chain: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphFile {
    pub path: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateResponse {
    pub id: String,
    pub version_id: String,
    pub sources: Vec<GraphFile>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteResponse {}

#[derive(Serialize)]
pub struct ForkRequest {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForkResponse {
    pub id: String,
    pub version_id: String,
    pub sources: Vec<GraphFile>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenRequest {
    pub schema_code: String,
    pub events_code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileRequest {
    pub indexer_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileError {
    pub file: String,
    pub line: Option<u32>,
    pub error: String,
}

impl FileError {
    fn pretty_print(&self) -> String {
        let line_info = self.line.map_or(String::new(), |l| format!(":{}", l));
        format!("{}{}:\n  {}", self.file, line_info, self.error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(rename = "overallError")]
    pub overall_error: String,
    pub errors: Option<Vec<String>>,
    #[serde(rename = "byFileAndLineErrors")]
    pub by_file_and_line_errors: Option<Vec<FileError>>,
}

impl ErrorDetails {
    fn pretty_print(&self) -> String {
        let mut output = format!("{}\n", self.overall_error);

        if let Some(errors) = &self.errors {
            output += "\nErrors:\n";
            for (i, error) in errors.iter().enumerate() {
                output += &format!("  {}. {}\n", i + 1, error);
            }
        }

        if let Some(file_errors) = &self.by_file_and_line_errors {
            output += "\nFile Errors:\n";
            for file_error in file_errors {
                output += &format!("{}\n", file_error.pretty_print());
            }
        }

        output.trim_end().to_string()
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}

#[derive(Deserialize)]
pub struct GhostVersion {
    pub sources: Vec<GraphFile>,
}

#[derive(Deserialize)]
pub struct CodegenResponse {
    pub err: Option<ErrorDetails>,
    pub version: Option<GhostVersion>,
}

#[derive(Deserialize)]
pub struct CompileResponse {
    pub err: Option<ErrorDetails>,
    pub version: Option<GhostVersion>,
}

#[derive(Deserialize)]
pub struct DeployResponse {
    pub err: Option<ErrorDetails>,
    pub ok: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    pub latest_version_id: String,
    pub name: String,
    pub description: Option<String>,
    pub chain: u64,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct ListResponse {
    pub err: Option<ErrorDetails>,
    pub graphs: Vec<Graph>,
}

#[derive(Deserialize)]
pub struct GraphDetailsResponse {
    pub graph: Graph,
}
