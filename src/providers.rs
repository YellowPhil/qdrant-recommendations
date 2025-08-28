use clap::{Args, Subcommand};
use embedding::{EmbeddingModel, hugging_face::HuggingFace, self_hosted::SelfHosted};
use eyre::{Result, WrapErr};

/// Provider configuration for different embedding model services
///
/// This enum allows users to configure different embedding model providers
/// through CLI subcommands, which can then be converted into EmbeddingModel
/// instances for use with the storage system.
///
/// # Use HuggingFace provider
/// 
/// qdrant-cli provider hugging-face --api-key YOUR_KEY --embedding-endpoint "https://..."
///
/// # Use self-hosted provider
/// 
/// qdrant-cli provider self-hosted --embedding-endpoint "http://localhost:8000"
/// ```
#[derive(Subcommand, Debug)]
pub(crate) enum Provider {
    /// Hugging Face embedding provider
    HuggingFace {
        #[arg(long)]
        /// Hugging Face API key
        api_key: String,

        #[arg(
            short,
            long,
            default_value = "https://router.huggingface.co/hf-inference/models/BAAI/bge-base-en-v1.5/pipeline/feature-extraction"
        )]
        /// Hugging Face endpoint
        embedding_endpoint: String,
    },

    /// Self-hosted embedding provider
    SelfHosted {
        #[arg(short, long)]
        /// Self-hosted embedding service endpoint
        embedding_endpoint: String,
    },
}

impl Provider {
    /// Convert the provider configuration into an EmbeddingModel
    ///
    /// This method takes ownership of the Provider and creates the corresponding
    /// EmbeddingModel instance. It's designed to be used after parsing CLI arguments.
    ///
    /// # Returns
    ///
    /// A `Result<Box<dyn EmbeddingModel>>` containing the created embedding model
    /// or an error if creation fails.
    ///
    pub async fn into_embedding_model(self) -> Result<Box<dyn EmbeddingModel>> {
        let embedding_model: Box<dyn EmbeddingModel> = match self {
            Provider::HuggingFace {
                api_key,
                embedding_endpoint,
            } => Box::new(
                HuggingFace::new(api_key, embedding_endpoint)
                    .await
                    .wrap_err("Failed to create hugging face embedding model")?,
            ),
            Provider::SelfHosted { embedding_endpoint } => Box::new(
                SelfHosted::new(embedding_endpoint)
                    .await
                    .wrap_err("Failed to create self hosted embedding model")?,
            ),
        };
        Ok(embedding_model)
    }
}
