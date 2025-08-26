use crate::hugging_face::HuggingFace;

mod hugging_face;
mod storage;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("HF_API_KEY").unwrap();
    let endpoint = "https://router.huggingface.co/hf-inference/models/BAAI/bge-base-en-v1.5/pipeline/feature-extraction";

    let hf = HuggingFace::new(api_key, endpoint.to_string());
    let embedding = hf.embed("Hello, world!".to_string()).await;
}
