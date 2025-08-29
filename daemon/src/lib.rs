use bincode::{Decode, Encode};

pub mod daemon;

#[derive(Debug, Decode, Encode)]
pub enum Request {
    CreateTopic(CreateTopicRequest),
    UpdateTopic(UpdateTopicRequest),
    SearchTopic(SearchTopicRequest),
    ListTopic(ListTopicRequest),
}

#[derive(Debug, Decode, Encode)]
pub enum Response {
    CreateTopic(CreateTopicResponse),
    UpdateTopic(UpdateTopicResponse),
    SearchTopic(SearchTopicResponse),
    ListTopic(ListTopicResponse),
}

#[derive(Debug, Decode, Encode)]
pub struct CreateTopicRequest {
    pub topic_name: String,
    pub content: String,
}

#[derive(Debug, Decode, Encode)]
pub struct CreateTopicResponse {
    pub success: bool,
}

#[derive(Debug, Decode, Encode)]
pub struct UpdateTopicRequest {
    pub topic_name: String,
    pub content: String,
}

#[derive(Debug, Decode, Encode)]
pub struct UpdateTopicResponse {
    pub success: bool,
}

#[derive(Debug, Decode, Encode)]
pub struct SearchTopicRequest {
    pub topic_name: Option<String>,
    pub query: String,
    pub limit: u64,
}

#[derive(Debug, Decode, Encode)]
pub struct SearchTopicResponse {
    pub results: Vec<String>,
}

#[derive(Debug, Decode, Encode)]
pub struct ListTopicRequest {
    pub topic_name: String,
    pub limit: u32,
}

#[derive(Debug, Decode, Encode)]
pub struct ListTopicResponse {
    pub results: Vec<String>,
}