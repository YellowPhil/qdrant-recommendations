use super::{EmbeddingError, EmbeddingModel};
use eyre::{WrapErr, eyre};
use reqwest::Client;

pub(crate) struct HuggingFace {
    api_key: String,
    endpoint: String,
    client: Client,
}

impl HuggingFace {
    async fn new(api_key: String, endpoint: String) -> eyre::Result<Self> {
        let client = Client::new();
        client
            .get("https://huggingface.co/api/whoami-v2")
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .wrap_err("Failed to validate api key")?;
        Ok(Self {
            api_key,
            endpoint,
            client,
        })
    }
}

#[async_trait::async_trait]
impl EmbeddingModel for HuggingFace {
    async fn embed(&self, input: String) -> Result<Vec<f32>, EmbeddingError> {
        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "inputs": input,
            }))
            .send()
            .await
            .map_err(|e| EmbeddingError::RequestError(eyre!("Failed to send request: {}", e)))?;

        let body = response
            .json::<Vec<f32>>()
            .await
            .map_err(|e| EmbeddingError::RequestError(eyre!("Failed to parse response: {}", e)))?;
        Ok(body)
    }
}
