use qdrant_client::Qdrant;
use eyre::{Result, WrapErr};

pub struct Storage {
    endpoint: String,
    client: Qdrant,
}

impl Storage {
    pub fn new(endpoint: &str) -> Result<Self> {
        let client = Qdrant::from_url(&endpoint).build().wrap_err("Failed to create Qdrant client")?;
        Ok(Self { endpoint: endpoint.to_string(), client })
    }
    fn client(&self) -> &Qdrant {
        &self.client
    }
}