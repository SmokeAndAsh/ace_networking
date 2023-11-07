// src/gateway/clients/candle/mod.rs

/// Candle API Mods
pub mod candle_error;
pub mod llama;

// Core Crates
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// Candle Crates
use candle_core::DType;
use crate::gateway::clients::candle::candle_error::CandleError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_name: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTextRequest {
    pub prompt: String,
    pub config: ModelConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateTextResponse {
    pub generated_text: String,
}

/// Candle API Utilities
/// Utilities relating to Candle API functions

// Serialize DType
// Candle's default DType is not serializable
#[derive(Debug, Serialize, Deserialize)]
pub enum SerializableDType {
    F16,
    BF16,
    F32,
}

impl From<SerializableDType> for DType {
    fn from(s: SerializableDType) -> Self {
        match s {
            SerializableDType::F16 => DType::F16,
            SerializableDType::BF16 => DType::BF16,
            SerializableDType::F32 => DType::F32,
        }
    }
}

impl TryFrom<DType> for SerializableDType {
    type Error = CandleError;

    fn try_from(d: DType) -> Result<Self, Self::Error> {
        match d {
            DType::F16 => Ok(SerializableDType::F16),
            DType::BF16 => Ok(SerializableDType::BF16),
            DType::F32 => Ok(SerializableDType::F32),
            _ => Err(CandleError::UnsupportedDTypeError(d)),
        }
    }
}