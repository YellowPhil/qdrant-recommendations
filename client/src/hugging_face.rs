use eyre::{Result, WrapErr};
use reqwest::Client;

pub(crate) struct HuggingFace {
    api_key: String,
    endpoint: String,
    client: Client,
}

impl HuggingFace {
    pub(crate) async fn new(api_key: String, endpoint: String) -> Result<Self> {
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

    pub(crate) async fn embed(&self, input: String) -> Result<Vec<f32>> {
        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "inputs": input,
            }))
            .send()
            .await
            .wrap_err("Failed to send request")?;

        let body = response
            .json::<Vec<f32>>()
            .await
            .wrap_err("Failed to parse response")?;
        Ok(body)
    }
}
