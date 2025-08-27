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
    
    #[arg(long, short, default_value = "http://localhost:6334")]
    qdrant_endpoint: String,
    #[arg(long)]
    hf_api_key: String,
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
        topic: String,
        
        #[arg(short, long)]
        query: String,
    },
    
    List {
        #[arg(short, long)]
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
                
                IdeaCommands::Search { topic, query } => {
                    println!("Searching in topic '{}' for: {}", topic, query);
                    let results = storage.search_topic(&topic, &query).await?;
                    
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

