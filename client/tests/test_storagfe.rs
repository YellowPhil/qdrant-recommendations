#[tokio::test]
async fn test_topic_storage_create_and_update() {
    let qdrant_endpoint = std::env::var("QDRANT_ENDPOINT").unwrap();
    let embedding_model_api_key = std::env::var("HF_API_KEY").unwrap();
    let embedding_model_endpoint = std::env::var("HF_EMBEDDING_MODEL_ENDPOINT").unwrap();

    let topic_storage = client::TopicStorage::new(
        &qdrant_endpoint,
        &embedding_model_api_key,
        &embedding_model_endpoint,
    ).await;

    if topic_storage.is_err() {
        println!("Skipping test - services not available");
        return;
    }

    let topic_storage = topic_storage.unwrap();

    let result = topic_storage.create_topic("test_topic", "This is test content").await;
    assert!(result.is_ok(), "Failed to create topic: {:?}", result);

    let result = topic_storage.update_topic("test_topic", "This is updated content").await;
    assert!(result.is_ok(), "Failed to update topic: {:?}", result);
}

#[tokio::test]
async fn test_topic_storage_multiple_topics() {
    let qdrant_endpoint = std::env::var("QDRANT_ENDPOINT").unwrap();
    let embedding_model_api_key = std::env::var("HF_API_KEY").unwrap();
    let embedding_model_endpoint = std::env::var("HF_EMBEDDING_MODEL_ENDPOINT").unwrap();

    let topic_storage = client::TopicStorage::new(
        &qdrant_endpoint,
        &embedding_model_api_key,
        &embedding_model_endpoint,
    ).await;

    if topic_storage.is_err() {
        println!("Skipping test - services not available");
        return;
    }

    let topic_storage = topic_storage.unwrap();

    let topics = vec![
        ("topic1", "Content for topic 1"),
        ("topic2", "Content for topic 2"),
        ("topic3", "Content for topic 3"),
    ];

    for (topic_name, content) in topics {
        let result = topic_storage.create_topic(topic_name, content).await;
        assert!(result.is_ok(), "Failed to create topic {}: {:?}", topic_name, result);
    }
}

#[tokio::test]
async fn test_topic_storage_search() {
    let qdrant_endpoint = std::env::var("QDRANT_ENDPOINT").unwrap();
    let embedding_model_api_key = std::env::var("HF_API_KEY").unwrap();
    let embedding_model_endpoint = std::env::var("HF_EMBEDDING_MODEL_ENDPOINT").unwrap();

    let topic_storage = client::TopicStorage::new(
        &qdrant_endpoint,
        &embedding_model_api_key,
        &embedding_model_endpoint,
    ).await;

    if topic_storage.is_err() {
        println!("Skipping test - services not available");
        return;
    }

    let topic_storage = topic_storage.unwrap();

    // Create a topic with specific content
    let topic_name = "search_test_topic";
    let content = "This is test content for searching";
    
    let result = topic_storage.create_topic(topic_name, content).await;
    assert!(result.is_ok(), "Failed to create topic: {:?}", result);

    // Search for the same content using the topic name and query
    let search_results = topic_storage.search_topic(topic_name, content).await;
    assert!(search_results.is_ok(), "Failed to search topic: {:?}", search_results);

    let results = search_results.unwrap();
    assert!(!results.is_empty(), "Search results should not be empty");
    assert!(results.contains(&content.to_string()), "Search results should contain the original content");
}
