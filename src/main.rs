use clap::{Parser, Subcommand};
use storage_client::TopicStorage;
use eyre::Result;

#[derive(Parser)]
#[command(name = "qdrant-cli")]
#[command(about = "CLI for Qdrant-based topic storage")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    // Qdrant endpoint
    #[arg(long, short, default_value = "http://localhost:6334")]
    qdrant_endpoint: String,
    // Hugging Face API key
    #[arg(long)]
    hf_api_key: String,
    // Hugging Face endpoint
    #[arg(long, default_value = "https://router.huggingface.co/hf-inference/models/BAAI/bge-base-en-v1.5/pipeline/feature-extraction")]
    hf_endpoint: String,
}

#[derive(Subcommand)]
enum Commands {
    Idea {
        #[command(subcommand)]
        subcommand: IdeaCommands,
    },
}

#[derive(Subcommand)]
enum IdeaCommands {
    New {
        #[arg(short, long)]
        topic: String,
        
        #[arg(short, long)]
        content: String,
    },
    
    Search {
        #[arg(short, long)]
        topic: Option<String>,
        
        #[arg(short, long)]
        query: String,

        #[arg(short, long, default_value = "10")]
        limit: u64,
    },
    
    List {
        #[arg(short, long, required = true)]
        topic: String,
        
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let storage = TopicStorage::new(
        &cli.qdrant_endpoint,
        &cli.hf_api_key,
        &cli.hf_endpoint,
    ).await?;
    
    match cli.command {
        Commands::Idea { subcommand } => {
            match subcommand {
                IdeaCommands::New { topic, content } => {
                    println!("Creating new topic: {}", topic);
                    storage.create_topic(&topic, &content).await?;
                    println!("✅ Topic '{}' created successfully!", topic);
                }
                
                IdeaCommands::Search { topic, query, limit } => {
                    let results = if let Some(topic) = topic {
                        println!("Searching in topic '{}' for: {}", topic, query);
                        storage.search_topic(Some(&topic), &query, limit).await?
                    } else {
                        println!("Searching for: {}", query);
                        storage.search_topic(None, &query, limit).await?
                    };
                    
                    if results.is_empty() {
                        println!("No results found.");
                    } else {
                        println!("Found {} results:", results.len());
                        for (i, result) in results.iter().enumerate() {
                            println!("  {}. {}", i + 1, result);
                        }
                    }
                }
                
                IdeaCommands::List { topic, limit } => {
                    println!("Listing topics in '{}' (limit: {})", topic, limit);
                    let results = storage.list_topic(&topic, limit).await?;
                    
                    if results.is_empty() {
                        println!("No topics found.");
                    } else {
                        println!("Found {} topics:", results.len());
                        for (i, result) in results.iter().enumerate() {
                            println!("  {}. {}", i + 1, result);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

