// src/gateway/clients/candle/llama/config.rs

/// Candle API Llama Config
///  Defines the LlamaModelConfig and any other configuration-related code.

// Core Crates
use clap::Parser;
use serde::{Deserialize, Serialize};

// Networking Crates
use crate::gateway::clients::candle::SerializableDType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run on CPU rather than on GPU.
    #[arg(long)]
    cpu: bool,

    /// The temperature used to generate samples.
    #[arg(long)]
    temperature: Option<f64>,

    /// Nucleus sampling probability cutoff.
    #[arg(long)]
    top_p: Option<f64>,

    /// The seed to use when generating random samples.
    #[arg(long, default_value_t = 299792458)]
    seed: u64,

    /// The length of the sample to generate (in tokens).
    #[arg(long, default_value_t = 100)]
    sample_len: usize,

    /// The initial prompt.
    #[arg(long)]
    prompt: Option<String>,

    /// Use different dtype than f16
    #[arg(long)]
    dtype: Option<String>,

    #[arg(long)]
    model_id: Option<String>,

    #[arg(long)]
    revision: Option<String>,

    #[arg(long)]
    use_flash_attn: bool,

    /// Penalty to be applied for repeating tokens, 1. means no penalty.
    #[arg(long, default_value_t = 1.0)]
    repeat_penalty: f32,

    /// The context size to consider for the repeat penalty.
    #[arg(long, default_value_t = 64)]
    repeat_last_n: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlamaModelConfig {
    pub cpu: bool,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub seed: u64,
    pub sample_len: usize,
    pub dtype: Option<SerializableDType>,
    pub model_id: Option<String>,
    pub revision: Option<String>,
    pub use_flash_attn: bool,
    pub repeat_penalty: f32,
    pub repeat_last_n: usize,
}

impl Default for LlamaModelConfig {
    fn default() -> Self {
        Self {
            cpu: false, // default to using GPU if available
            temperature: Some(1.0), // default temperature
            top_p: Some(0.9), // default nucleus sampling probability cutoff
            seed: 299792458, // default seed
            sample_len: 100, // default sample length
            dtype: Some(SerializableDType::F16), // default data type
            model_id: None, // model_id can be set later as needed
            revision: None, // revision can be set later as needed
            use_flash_attn: false, // default attention mechanism
            repeat_penalty: 1.0, // default penalty for repeating tokens
            repeat_last_n: 64, // default context size for repeat penalty
        }
    }
}