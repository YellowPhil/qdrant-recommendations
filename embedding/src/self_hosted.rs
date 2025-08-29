use super::{EmbeddingError, EmbeddingModel};
use eyre::eyre;

pub struct SelfHosted {
    endpoint: String,
    client: reqwest::Client,
}

impl SelfHosted {
    pub async fn new(endpoint: String) -> eyre::Result<Self> {
        Ok(Self {
            endpoint,
            client: reqwest::Client::new(),
        })
    }
}

#[async_trait::async_trait]
impl EmbeddingModel for SelfHosted {
    async fn embed(&self, input: &str) -> Result<Vec<f32>, EmbeddingError> {
        let response = self
            .client
            .post(&self.endpoint)
            .json(&input)
            .send()
            .await
            .map_err(|e| EmbeddingError::RequestError(eyre!("Failed to send request: {}", e)))?;
        Ok(response
            .json::<Vec<f32>>()
            .await
            .map_err(|e| EmbeddingError::RequestError(eyre!("Failed to parse response: {}", e)))?)
    }
}
