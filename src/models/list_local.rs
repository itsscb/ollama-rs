use serde::Deserialize;

use crate::Ollama;

use super::Model;

impl Ollama {
    pub async fn list_local_models(&self) -> crate::error::Result<Vec<Model>> {
        let uri = format!("{}/api/tags", self.uri());
        let res = self
            .reqwest_client
            .get(uri)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            return Err(res.text().await.unwrap_or_else(|e| e.to_string()).into());
        }

        let res = res.bytes().await.map_err(|e| e.to_string())?;
        let res =
            serde_json::from_slice::<ListLocalModelsResponse>(&res).map_err(|e| e.to_string())?;

        Ok(res.models)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ListLocalModelsResponse {
    models: Vec<Model>,
}
