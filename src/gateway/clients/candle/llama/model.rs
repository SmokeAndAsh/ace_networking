// src/gateway/clients/candle/llama/model.rs

/// Candle API Llama Model
/// Contains code related to downloading weights and initializing the LlamaModel.

// Core Crates
use std::option::Option;

// Candle Crates
use candle_core::{Device, DType, Tensor};
use candle_nn::var_builder::VarBuilder;
use hf_hub::api::tokio::Api;

use candle_transformers::models::llama as model;
use model::{Config, Llama};
use tokenizers::Tokenizer;

// Networking Crates
use crate::gateway::clients::candle::candle_error::CandleError;
use crate::gateway::clients::candle::llama::config::LlamaModelConfig;
use crate::gateway::clients::candle::llama::generator::{LlamaGenerateTextRequest, LlamaGenerateTextResponse};
use crate::gateway::clients::candle::llama::tokenizer::LlamaTokenizer;

pub struct LlamaModel {
    pub model: Option<Llama>,
    pub config: LlamaModelConfig,
    pub tokenizer: LlamaTokenizer,
}

impl LlamaModel {
    pub fn new(config: LlamaModelConfig, tokenizer: LlamaTokenizer) -> Self {
        LlamaModel {
            model: None,
            config,
            tokenizer,
        }
    }

    pub async fn download_weights(&self) -> Result<String, CandleError> {
        let api = Api::new().map_err(|_| CandleError::InitializationError)?;
        let model_id = self.config.model_id.clone().unwrap_or_else(|| "meta-llama/Llama-2-7b-hf".to_string());
        let repo = api.model(model_id);

        let weights_filename = repo.get("model.safetensors").await
            .map_err(|_| CandleError::DownloadError)?;

        Ok(weights_filename.to_string_lossy().into_owned())
    }

    pub fn get_tokenizer(&self) -> Result<&Tokenizer, CandleError> {
        self.tokenizer.ok_or(CandleError::UninitializedModelError)
    }

    pub async fn initialize_model(&mut self, weights_path: &str) -> Result<(), CandleError> {

        println!("Building Llama tokenizer...");
        self.tokenizer.download_and_load_tokenizer().await?;

        let device = Device::new_cuda(0)?;
        let dtype = DType::F16;
        let config = Config::config_7b_v2(false);

        let cache = model::Cache::new(false, dtype, &config, &device)?;
        let weights = candle_core::safetensors::load(weights_path, &device)
            .map_err(|_| CandleError::LoadModelError);
        let vb = VarBuilder::from_tensors(weights, dtype, &device);

        println!("Building Llama model...");
        let model = Llama::load(vb, &cache, &config)?;
        self.model = Some(model);

        Ok(())
    }

    pub async fn generate_text(&self, request: LlamaGenerateTextRequest) -> Result<LlamaGenerateTextResponse, CandleError> {
        use candle_transformers::generation::LogitsProcessor;

        // Ensure model is initialized
        let model = self.model.as_ref().ok_or(CandleError::UninitializedModelError);
        let device = Device::new_cuda(0)?;
        let tokenizer = &self.tokenizer;

        // Start the generation process
        println!("Starting the text generation...");
        let prompt = &request.prompt;
        let mut tokens = self.tokenizer.encode(prompt, true)?;

        // Initialize the logits processor with the configuration from the request
        let mut logits_processor = LogitsProcessor::new(
            self.config.seed,
            Some(self.config.temperature.unwrap_or(1.0)),
            Some(self.config.top_p.unwrap_or(0.9)),
        );

        let mut index_pos = 0;
        let mut token_generated = 0;
        for _ in 0..self.config.sample_len {
            let context_size = tokens.len(); // Assuming no kv_cache for simplicity
            let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
            let input = Tensor::new(ctxt, &device)?.unsqueeze(0)?;
            let logits = model.forward(&input)?;
            let logits = logits.squeeze(0)?;

            // Apply repeat penalty if configured
            let logits = if self.config.repeat_penalty != 1.0 {
                let start_at = tokens.len().saturating_sub(self.config.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.config.repeat_penalty,
                    &tokens[start_at..],
                )?
            } else {
                logits
            };

            index_pos += ctxt.len();

            let next_token = logits_processor.sample(&logits)?;
            token_generated += 1;
            tokens.push(next_token);

            // Check for end-of-sequence token and break if found
            if let Some(eos_token_id) = tokenizer.token_to_id("<|endoftext|>") {
                if Some(next_token) == eos_token_id {
                    break;
                }
            }
        }

        // Decode the tokens into a string
        let generated_text = tokens.iter()
            .map(|&id| self.tokenizer.decode(&[id], false).unwrap_or_default())
            .collect::<String>();

        Ok(LlamaGenerateTextResponse { generated_text })
    }
}