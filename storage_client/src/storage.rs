use eyre::{Result, WrapErr};
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
         Condition, CreateCollectionBuilder, Distance,
        Filter, PointStruct, QueryPointsBuilder, RetrievedPoint,
        ScalarQuantizationBuilder, ScoredPoint, ScrollPointsBuilder, 
        SearchPointsBuilder, UpsertPointsBuilder,  VectorParamsBuilder,
    },
};

pub(crate) struct Storage {
    endpoint: String,
    client: Qdrant,
}

pub(super) const TOPIC_NAME_KEY: &str = "topic_name";
pub(super) const TOPIC_CONTENT_KEY: &str = "topic_content";

impl Storage {
    pub(crate) fn new(endpoint: &str) -> Result<Self> {
        let client = Qdrant::from_url(&endpoint)
            .build()
            .wrap_err("Failed to create Qdrant client")?;
        Ok(Self {
            endpoint: endpoint.to_string(),
            client,
        })
    }

    pub(crate) async fn create_collection(
        &self,
        collection_name: &str,
        vector_size: u64,
    ) -> Result<()> {
        let create_collection = CreateCollectionBuilder::new(collection_name)
            .vectors_config(VectorParamsBuilder::new(vector_size, Distance::Cosine))
            .quantization_config(ScalarQuantizationBuilder::default());

        self.client
            .create_collection(create_collection)
            .await
            .wrap_err("Failed to create collection")?;

        Ok(())
    }

    pub(crate) async fn upsert_point(
        &self,
        collection_name: &str,
        point: Vec<f32>,
        payload: Payload,
    ) -> Result<()> {
        let point = PointStruct::new(0, point, payload);
        self.client
            .upsert_points(UpsertPointsBuilder::new(collection_name, vec![point]))
            .await
            .wrap_err("Failed to upsert point")?;

        Ok(())
    }
    pub(crate) async fn list_points_by_topic(
        &self,
        collection_name: &str,
        topic_name: &str,
        limit: u32,
    ) -> Result<Vec<RetrievedPoint>> {
        let response = self
            .client
            .scroll(
                ScrollPointsBuilder::new(collection_name)
                    .filter(Filter::must([Condition::matches(
                        TOPIC_NAME_KEY,
                        topic_name.to_string(),
                    )]))
                    .limit(limit)
                    .with_vectors(false)
                    .with_payload(true),
            )
            .await
            .wrap_err("Failed to scroll points")?;
        Ok(response.result)
    }
    pub(crate) async fn get_points_by_topic(
        &self,
        collection_name: &str,
        topic_name: &str,
        query: Vec<f32>,
    ) -> Result<Vec<ScoredPoint>> {
        let response = self
            .client
            .query(
                QueryPointsBuilder::new(collection_name)
                    .query(query)
                    .filter(Filter::must([Condition::matches(
                        TOPIC_NAME_KEY,
                        topic_name.to_string(),
                    )]))
                    .with_vectors(false)
                    .with_payload(true),
            )
            .await
            .wrap_err("Failed to query points")?;
        Ok(response.result)
    }

    pub(crate) async fn search_points(
        &self,
        collection_name: &str,
        query: Vec<f32>,
        limit: u64,
    ) -> Result<Vec<ScoredPoint>> {
        let response = self
            .client
            .search_points(
                SearchPointsBuilder::new(collection_name, query, limit)
                    .with_vectors(false)
                    .with_payload(true)
            )
            .await
            .wrap_err("Failed to search points")?;
        Ok(response.result)
    }

    pub(crate) async fn get_collection_info(
        &self,
        collection_name: &str,
    ) -> Result<Option<qdrant_client::qdrant::CollectionInfo>> {
        let info = self
            .client
            .collection_info(collection_name)
            .await
            .wrap_err("Failed to get collection info")?;
        Ok(info.result)
    }

    pub(crate) async fn delete_collection(&self, collection_name: &str) -> Result<()> {
        self.client
            .delete_collection(collection_name)
            .await
            .wrap_err("Failed to delete collection")?;

        Ok(())
    }

    pub(crate) async fn collection_exists(&self, collection_name: &str) -> Result<bool> {
        let response = self
            .client
            .collection_exists(collection_name)
            .await
            .wrap_err("Failed to check if collection exists")?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_collection() {
        let storage = Storage::new("http://localhost:6334").unwrap();
        let collection_name = format!("test_collection_{}", Uuid::new_v4());

        storage
            .create_collection(&collection_name, 3)
            .await
            .unwrap();

        let collection_info = storage.get_collection_info(&collection_name).await.unwrap();
        assert_eq!(collection_info.unwrap().points_count, Some(0));

        storage.delete_collection(&collection_name).await.unwrap();
    }

    #[tokio::test]
    async fn test_upsert_point() {
        let storage = Storage::new("http://localhost:6334").unwrap();
        let collection_name = format!("test_collection_{}", Uuid::new_v4());

        storage
            .create_collection(&collection_name, 3)
            .await
            .unwrap();

        assert_eq!(
            storage.collection_exists(&collection_name).await.unwrap(),
            true
        );

        storage
            .upsert_point(
                &collection_name,
                vec![1.0, 2.0, 3.0],
                serde_json::json!({
                    "key": "value"
                })
                .try_into()
                .unwrap(),
            )
            .await
            .unwrap();
        let collection_info = storage.get_collection_info(&collection_name).await.unwrap();
        assert_eq!(collection_info.unwrap().points_count, Some(1));

        storage.delete_collection(&collection_name).await.unwrap();
    }

    #[tokio::test]
    async fn test_retrieve_point() {
        let storage = Storage::new("http://localhost:6334").unwrap();
        let collection_name = format!("test_collection_{}", Uuid::new_v4());

        storage
            .create_collection(&collection_name, 3)
            .await
            .unwrap();

        let point = storage
            .get_points_by_topic(&collection_name, "test_topic", vec![1.0, 2.0, 3.0])
            .await
            .unwrap();
        assert_eq!(point.len(), 0);

        storage
            .upsert_point(
                &collection_name,
                vec![1.0, 2.0, 3.0],
                serde_json::json!({
                    TOPIC_NAME_KEY: "test_topic"
                })
                .try_into()
                .unwrap(),
            )
            .await
            .unwrap();
        let point = storage
            .get_points_by_topic(&collection_name, "test_topic", vec![1.0, 2.0, 3.0])
            .await
            .unwrap();
        assert_eq!(point.len(), 1);

        storage.delete_collection(&collection_name).await.unwrap();
    }
    #[tokio::test]
    async fn test_scroll_points() {
        let storage = Storage::new("http://localhost:6334").unwrap();
        let collection_name = format!("test_collection_{}", Uuid::new_v4());

        storage
            .create_collection(&collection_name, 3)
            .await
            .unwrap();
        storage
            .upsert_point(
                &collection_name,
                vec![1.0, 2.0, 3.0],
                serde_json::json!({
                    TOPIC_NAME_KEY: "test_topic"
                })
                .try_into()
                .unwrap(),
            )
            .await
            .unwrap();
        let points = storage
            .list_points_by_topic(&collection_name, "test_topic", 10)
            .await
            .unwrap();
        assert_eq!(points.len(), 1);

        storage.delete_collection(&collection_name).await.unwrap();
    }
}
