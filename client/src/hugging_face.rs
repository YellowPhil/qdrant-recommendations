use reqwest::Client;
use eyre::{Result, WrapErr};

pub struct HuggingFace {
    api_key: String,
    endpoint: String,
    client: Client,
}

impl HuggingFace {
    pub fn new(api_key: String, endpoint: String) -> Self {
        let client = Client::new();
        Self { api_key, endpoint, client }
    }

    pub async fn embed(&self, input: String) -> Result<Vec<f64>> {
        let response = self.client.post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "inputs": input,
            }))
            .send()
            .await
            .wrap_err("Failed to send request")?;

        let body = response.json::<Vec<f64>>().await.wrap_err("Failed to parse response")?;
        Ok(body)
    }
}