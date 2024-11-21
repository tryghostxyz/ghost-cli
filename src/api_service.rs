use eyre::{eyre, Report};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::types::{
    CodegenRequest, CodegenResponse, CompileRequest, CompileResponse, CreateRequest,
    CreateResponse, DeleteResponse, DeployResponse, ForkRequest, ForkResponse, Graph,
    GraphDetailsResponse, GraphFile, ListResponse,
};

pub struct ApiService {
    client: Client,
    base_url: String,
    api_key: String,
    web_base_url: String,
}

impl ApiService {
    pub fn new(base_url: String, api_key: String, web_base_url: String) -> Self {
        Self { client: Client::new(), base_url, api_key, web_base_url }
    }

    pub fn web_base_url(&self) -> String {
        self.web_base_url.clone()
    }

    pub async fn create_graph(&self, req: CreateRequest) -> eyre::Result<CreateResponse> {
        let url = format!("{}/gg/cli/graphs", self.base_url);
        let response =
            self.client.post(&url).header("GG-KEY", &self.api_key).json(&req).send().await?;

        let create_resp: CreateResponseInternal = response.json().await?;
        create_resp.try_into()
    }

    pub async fn codegen(
        &self,
        id: &str,
        payload: &CodegenRequest,
    ) -> eyre::Result<CodegenResponse> {
        let url = format!("{}/gg/cli/graphs/{}/codegen", self.base_url, id);
        let response =
            self.client.post(&url).header("GG-KEY", &self.api_key).json(payload).send().await?;
        let json: Value = response.json().await?;
        let codegen_resp: CodegenResponse = serde_json::from_value(json)
            .map_err(|e| eyre!("Failed to deserialize CodegenResponse: {}", e))?;
        Ok(codegen_resp)
    }

    pub async fn compile(
        &self,
        id: &str,
        payload: &CompileRequest,
    ) -> eyre::Result<CompileResponse> {
        let url = format!("{}/gg/cli/graphs/{}/compile", self.base_url, id);
        let response =
            self.client.post(&url).header("GG-KEY", &self.api_key).json(payload).send().await?;

        let json: Value = response.json().await?;
        let compile_res: CompileResponse = serde_json::from_value(json)
            .map_err(|e| eyre!("Failed to deserialize CompileResponse: {}", e))?;
        Ok(compile_res)
    }

    pub async fn deploy(&self, id: &str) -> eyre::Result<DeployResponse> {
        let url = format!("{}/gg/cli/graphs/{}/deploy", self.base_url, id);
        let response = self.client.post(&url).header("GG-KEY", &self.api_key).send().await?;

        let deploy_res: DeployResponse = serde_json::from_value(response.json().await?)
            .map_err(|e| eyre!("Failed to deserialize DeployResponse: {}", e))?;
        Ok(deploy_res)
    }

    pub async fn get_graph(&self, id: &str) -> eyre::Result<Graph> {
        let url = format!("{}/gg/cli/graphs/{}", self.base_url, id);
        let response = self.client.get(&url).header("GG-KEY", &self.api_key).send().await?;
        let json = response.json().await?;
        let graph: GraphDetailsResponse = serde_json::from_value(json)
            .map_err(|e| eyre!("Failed to deserialize Graph response: {}", e))?;
        Ok(graph.graph)
    }

    pub async fn get_graphs(&self) -> eyre::Result<ListResponse> {
        let url = format!("{}/gg/cli/list", self.base_url);
        let response = self.client.get(&url).header("GG-KEY", &self.api_key).send().await?;

        let list_response: ListResponse = serde_json::from_value(response.json().await?)
            .map_err(|e| eyre!("Failed to deserialize ListResponse: {}", e))?;
        Ok(list_response)
    }

    pub async fn fork_graph(&self, id: &str, payload: &ForkRequest) -> eyre::Result<ForkResponse> {
        let url = format!("{}/gg/cli/graphs/{}/fork", self.base_url, id);
        let response =
            self.client.post(&url).header("GG-KEY", &self.api_key).json(payload).send().await?;

        let fork_response: ForkResponseInternal = serde_json::from_value(response.json().await?)
            .map_err(|e| eyre!("Failed to deserialize ForkResponse: {}", e))?;
        fork_response.try_into()
    }

    pub async fn delete_graph(&self, id: &str) -> eyre::Result<DeleteResponse> {
        let url = format!("{}/gg/cli/graphs/{}", self.base_url, id);
        let response = self.client.delete(&url).header("GG-KEY", &self.api_key).send().await?;

        let delete_response: DeleteResponseInternal =
            serde_json::from_value(response.json().await?)
                .map_err(|e| eyre!("Failed to deserialize DeleteResponse: {}", e))?;
        delete_response.try_into()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeleteResponseInternal {
    pub ok: bool,
}

impl TryInto<DeleteResponse> for DeleteResponseInternal {
    type Error = Report;

    fn try_into(self) -> eyre::Result<DeleteResponse> {
        match self {
            DeleteResponseInternal { ok: true } => Ok(DeleteResponse {}),
            _ => Err(eyre!("Unexpected API response")),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateResponseInternal {
    pub ok: bool,
    pub id: Option<String>,
    pub version_id: Option<String>,
    pub sources: Option<Vec<GraphFile>>,
}

impl TryInto<CreateResponse> for CreateResponseInternal {
    type Error = Report;

    fn try_into(self) -> eyre::Result<CreateResponse> {
        match self {
            CreateResponseInternal {
                ok: true,
                id: Some(id),
                version_id: Some(version_id),
                sources: Some(sources),
            } => Ok(CreateResponse { id, version_id, sources }),
            _ => Err(eyre!("Unexpected API response")),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForkResponseInternal {
    pub ok: bool,
    pub ghost_graph_id: Option<String>,
    pub ghost_graph_version_id: Option<String>,
    pub sources: Option<Vec<GraphFile>>,
}

impl TryInto<ForkResponse> for ForkResponseInternal {
    type Error = Report;

    fn try_into(self) -> eyre::Result<ForkResponse> {
        match self {
            ForkResponseInternal {
                ok: true,
                ghost_graph_id: Some(id),
                ghost_graph_version_id: Some(version_id),
                sources: Some(sources),
            } => Ok(ForkResponse { id, version_id, sources }),
            _ => Err(eyre!("Unexpected API response")),
        }
    }
}
