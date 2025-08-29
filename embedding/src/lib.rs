pub mod hugging_face;
pub mod self_hosted;

#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("Request error: {0}")]
    RequestError(eyre::Error),
}

type Result<T> = std::result::Result<T, EmbeddingError>;

#[async_trait::async_trait]
pub trait EmbeddingModel {
    async fn embed(&self, input: &str) -> Result<Vec<f32>>;
}
