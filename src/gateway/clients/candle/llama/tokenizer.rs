// src/gateway/clients/candle/llama/tokenizer.rs

/// Candle API Llama Tokenizer
/// Handles downloading and initializing the tokenizer.

// Candle Crates
use hf_hub::api::tokio::Api;
use tokenizers::{tokenizer::Tokenizer as HfTokenizer};

// Networking Crates
use crate::gateway::clients::candle::llama::config::LlamaModelConfig;
use crate::gateway::clients::candle::candle_error::CandleError;

pub struct LlamaTokenizer {
    pub  tokenizer: Option<HfTokenizer>,
    pub model_config: LlamaModelConfig,
}

impl LlamaTokenizer {
    pub fn new(model_config: LlamaModelConfig) -> Self {
        LlamaTokenizer {
            tokenizer: None,
            model_config,
        }
    }

    pub async fn download_and_load_tokenizer(&mut self) -> Result<(), CandleError> {
        let api = Api::new().map_err(|_| CandleError::InitializationError("Failed to create API".into()))?;
        let repo = api.model(self.model_config.model_id.clone().unwrap_or_else(|| "meta-llama/Llama-2-7b-hf".to_string()));

        let tokenizer_filename = repo.get("tokenizer.json").await.map_err(|_| CandleError::DownloadError("Failed to get tokenizer".into()))?;

        let tokenizer = HfTokenizer::from_file(&tokenizer_filename).map_err(|_| CandleError::LoadModelError("Failed to load tokenizer".into()))?;
        println!("Tokenizer loaded for model {}", self.model_config.model_id.as_ref().unwrap_or(&"default-model".to_string()));

        self.tokenizer = Some(tokenizer);

        Ok(())
    }

    pub fn encode(&self, text: &str, add_special_tokens: bool) -> Result<Vec<u32>, CandleError::SafeTensorError> {
        let tokenizer = self.tokenizer.as_ref().ok_or_else(|| CandleError::UninitializedModelError("Tokenizer is not initialized".into()))?;

        let encodings = tokenizer
            .encode(text, add_special_tokens)
            .map_err(|_| CandleError::EncodingError("Failed to encode text".into()))
            .map(|encoding| encoding.get_ids().to_vec())?;

        Ok(encodings)
    }

    pub fn decode(&self, ids: &[u32], skip_special_tokens: bool) -> Result<String, CandleError::SafeTensorError> {
        let tokenizer = self.tokenizer.as_ref().ok_or_else(|| CandleError::UninitializedModelError("Tokenizer is not initialized".into()))?;

        let decoded = tokenizer
            .decode(ids, skip_special_tokens)
            .map_err(|_| CandleError::DecodingError("Failed to decode tokens".into()))?;

        Ok(decoded)
    }
}