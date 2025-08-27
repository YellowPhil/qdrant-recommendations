pub mod hugging_face;

#[derive(Debug, thiserror::Error)]
pub enum EmbeddingError {
    #[error("Request error: {0}")]
    RequestError(eyre::Error),
}

type Result<T> = std::result::Result<T, EmbeddingError>;

#[async_trait::async_trait]
pub trait EmbeddingModel {
    async fn embed(&self, input: String) -> Result<Vec<f32>>;
}
