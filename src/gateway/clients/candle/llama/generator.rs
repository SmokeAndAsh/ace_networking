// src/gateway/clients/candle/llama/generator.rs

/// Candle API Llama Generator
/// Contains the logic for generating text using the model and tokenizer.

// Core Crates
use serde::{Deserialize, Serialize};

// Networking Crates
use crate::gateway::clients::candle::llama::config::LlamaModelConfig;

#[derive(Debug, Serialize, serde::Deserialize)]
pub struct LlamaGenerateTextRequest {
    pub prompt: String,
    pub config: LlamaModelConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlamaGenerateTextResponse {
    pub generated_text: String,
}