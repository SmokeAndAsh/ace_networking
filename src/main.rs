// src/main.rs

// Core Crates
use reqwest::Client;
use tokio;

// Network Crates
use networking::gateway::clients::candle::llama::{config::LlamaModelConfig, generator::LlamaGenerateTextRequest, model::LlamaModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the reqwest client
    let client = Client::new();

    // Create a Llama model configuration
    let llama_config = LlamaModelConfig::default();

    // Create a Llama model instance
    let llama_model = LlamaModel::new(llama_config);

    // Create a sample request
    let request = LlamaGenerateTextRequest {
        prompt: "Please provide a brief introduction to the Rust programming language.".to_string(),
        config: Default::default(),
    };

    // Call the generate_text method on the Llama model instance
    let response = llama_model.generate_text(&client, request).await?;

    // Print the response
    println!("Generated text: {}", response.generated_text);

    Ok(())
}