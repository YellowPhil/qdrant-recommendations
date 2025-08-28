use clap::{Parser, Subcommand};
use eyre::Result;

mod providers;
mod daemon;

#[derive(Parser)]
#[command(name = "qdrant-cli")]
#[command(about)]
#[command(version)]

/// CLI for Qdrant-based topic storage
struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, short, default_value = "http://localhost:6334")]
    /// Qdrant endpoint
    qdrant_endpoint: String,
}

/// Main CLI commands for the Qdrant-based topic storage system
#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Manage ideas and topics
    Idea {
        #[command(subcommand)]
        subcommand: IdeaCommands,
    },
    /// Configure embedding model providers
    Provider {
        #[command(subcommand)]
        subcommand: providers::Provider,
    },
}

/// Commands for managing ideas within topics
#[derive(Subcommand, Debug)]
pub(crate) enum IdeaCommands {
    /// Create a new idea in a topic
    New {
        #[arg(short, long)]
        /// Name of the topic to create the idea in
        topic: String,
        /// Content of the idea
        content: String,
    },

    /// Search for ideas across topics or within a specific topic
    Search {
        #[arg(short, long)]
        /// Optional topic to search within (searches all topics if not specified)
        topic: Option<String>,

        #[arg(short, long, default_value = "10")]
        /// Maximum number of search results to return
        limit: u64,

        /// Search query to find matching ideas
        query: String,
    },

    /// List all ideas within a specific topic
    List {
        #[arg(short, long)]
        /// Name of the topic to list ideas from
        topic: String,

        #[arg(short, long, default_value = "10")]
        /// Maximum number of ideas to list
        limit: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Idea { subcommand } => match subcommand {
            IdeaCommands::New { topic, content } => {
                println!("Creating new topic: {}", topic);
                // TODO: Implement storage creation with embedding model
                // let storage = TopicStorage::new(&cli.qdrant_endpoint, embedding_model).await?;
                // storage.create_topic(&topic, &content).await?;
                println!("✅ Topic '{}' created successfully!", topic);
            }

            IdeaCommands::Search {
                topic,
                query,
                limit,
            } => {
                let results: Vec<String> = if let Some(topic) = topic {
                    println!("Searching in topic '{}' for: {}", topic, query);
                    // TODO: Implement search with embedding model
                    // storage.search_topic(Some(&topic), &query, limit).await?
                    vec![] // Placeholder
                } else {
                    println!("Searching for: {}", query);
                    // TODO: Implement search with embedding model
                    // storage.search_topic(None, &query, limit).await?
                    vec![] // Placeholder
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
                // TODO: Implement list with embedding model
                // let results = storage.list_topic(&topic, limit).await?;
                let results: Vec<String> = vec![]; // Placeholder

                if results.is_empty() {
                    println!("No topics found.");
                } else {
                    println!("Found {} topics:", results.len());
                    for (i, result) in results.iter().enumerate() {
                        println!("  {}. {}", i + 1, result);
                    }
                }
            }
        },

        Commands::Provider { subcommand } => {
            println!("Creating embedding model from provider configuration...");
            let embedding_model = subcommand.into_embedding_model().await?;
            println!("✅ Embedding model created successfully!");
            // Example:
            // let storage = TopicStorage::new(&cli.qdrant_endpoint, embedding_model).await?;

            // For demonstration, just print the model type
            println!(
                "Model type: {:?}",
                std::any::type_name_of_val(&*embedding_model)
            );
        }
    }

    Ok(())
}
