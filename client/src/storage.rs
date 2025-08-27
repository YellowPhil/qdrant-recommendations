use std::collections::HashMap;

use eyre::{Result, WrapErr};
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        Condition, CreateCollectionBuilder, Distance, Filter, GetPointsBuilder, PointStruct,
        QueryPointsBuilder, RetrievedPoint, ScalarQuantizationBuilder, ScoredPoint,
        ScrollPointsBuilder, SearchBatchPointsBuilder, SearchPointsBuilder, UpsertPointsBuilder,
        Value, VectorParamsBuilder,
    },
};

pub(crate) struct Storage {
    endpoint: String,
    client: Qdrant,
}

const TOPIC_NAME_KEY: &str = "topic_name";

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

    pub(crate) async fn get_collection_info(
        &self,
        collection_name: &str,
    ) -> Result<qdrant_client::qdrant::CollectionInfo> {
        let info = self
            .client
            .collection_info(collection_name)
            .await
            .wrap_err("Failed to get collection info")?;
        Ok(info.result.unwrap())
    }

    pub(crate) async fn delete_collection(&self, collection_name: &str) -> Result<()> {
        self.client
            .delete_collection(collection_name)
            .await
            .wrap_err("Failed to delete collection")?;

        Ok(())
    }

    pub(crate) async fn collection_exists(&self, collection_name: &str) -> Result<bool> {
        match self.get_collection_info(collection_name).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
