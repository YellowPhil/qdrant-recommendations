use ell
mbedding::{EmbeddingError, EmbeddingModel};
use qdrant_client::Payload;

use crate::storage::{TOPIC_CONTENT_KEY, TOPIC_NAME_KEY};

pub mod storage;

#[derive(Debug, thiserror::Error)]
pub enum TopicStorageError {
    #[error("Qdrant error: {0}")]
    QdrantError(String),
    #[error("Embedding error: {0}")]
    EmbeddingError(EmbeddingError),
}

type Result<T> = std::result::Result<T, TopicStorageError>;

pub struct TopicStorage<T: EmbeddingModel> {
    storage: storage::Storage,
    qdrant_collection_name: String,
    embedding_model: T,
}

impl<T: EmbeddingModel> TopicStorage<T> {
    pub async fn new(qdrant_endpoint: &str, embedding_model: T) -> Result<Self> {
        let storage = storage::Storage::new(qdrant_endpoint)
            .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?;
        let qdrant_collection_name = "topic_storage".to_string();

        Ok(Self {
            storage,
            qdrant_collection_name,
            embedding_model,
        })
    }

    pub async fn create_topic(&self, topic_name: &str, content: &str) -> Result<()> {
        let embedding = self
            .embedding_model
            .embed(content.to_string())
            .await
            .map_err(|e| TopicStorageError::EmbeddingError(e))?;

        if !self
            .storage
            .collection_exists(&self.qdrant_collection_name)
            .await
            .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?
        {
            self.storage
                .create_collection(&self.qdrant_collection_name, embedding.len() as u64)
                .await
                .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?;
        }
        self.update_topic(topic_name, content).await?;
        Ok(())
    }

    pub async fn update_topic(&self, topic_name: &str, content: &str) -> Result<()> {
        let embedding = self
            .embedding_model
            .embed(content.to_string())
            .await
            .map_err(|e| TopicStorageError::EmbeddingError(e))?;

        let payload: Payload = serde_json::json!({
            TOPIC_NAME_KEY: topic_name,
            TOPIC_CONTENT_KEY: content,
        })
        .try_into()
        .map_err(|_| {
            TopicStorageError::QdrantError(
                "Failed to convert payload to Qdrant payload".to_string(),
            )
        })?;

        self.storage
            .upsert_point(&self.qdrant_collection_name, embedding, payload)
            .await
            .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?;

        Ok(())
    }
    pub async fn search_topic(
        &self,
        topic: Option<&str>,
        query: &str,
        limit: u64,
    ) -> Result<Vec<String>> {
        let embedding = self
            .embedding_model
            .embed(query.to_string())
            .await
            .map_err(|e| TopicStorageError::EmbeddingError(e))?;

        let results = if let Some(topic) = topic {
            self.storage
                .get_points_by_topic(&self.qdrant_collection_name, topic, embedding)
                .await
                .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?
        } else {
            self.storage
                .search_points(&self.qdrant_collection_name, embedding, limit)
                .await
                .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?
        };
        Ok(results
            .iter()
            .map(|r| r.payload.get(TOPIC_CONTENT_KEY).unwrap().to_string())
            .collect())
    }
    pub async fn list_topic(&self, topic: &str, limit: u32) -> Result<Vec<String>> {
        let results = self
            .storage
            .list_points_by_topic(&self.qdrant_collection_name, topic, limit)
            .await
            .map_err(|e| TopicStorageError::QdrantError(e.to_string()))?;
        Ok(results
            .iter()
            .map(|r| r.payload.get(TOPIC_CONTENT_KEY).unwrap().to_string())
            .collect())
    }
}
