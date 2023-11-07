// src/gateway/clients/candle/candle_error.rs

/// Candle Error Handler
/// Errors specific to the Candle client.
/// Represents errors that can occur when interacting with the Candle API.

// Core Crates
use std::error::Error as StdError;
use thiserror::Error;

// Candle Crates
use candle_core::{Error as CoreError, DType};
use tokenizers::{Error as TokenError};

// Networking Crates
use crate::gateway::clients::client_error::ClientError;

#[derive(Debug, Error)]
pub enum CandleError {
    #[error("Candle IO error: {0}")]
    CandleIoError(CoreError),

    #[error("Candle core error: {0}")]
    CoreError(CoreError),

    #[error("Cuda error: {0}")]
    CudaError(CoreError),

    #[error("Decoding error: {0}")]
    DecodingError(TokenError),

    #[error("Download error: {0}")]
    DownloadError(CoreError),

    #[error("Encoding error: {0}")]
    EncodingError(TokenError),

    #[error("Generic client error: {0}")]
    GenericError(ClientError),

    #[error("Initialization error: {0}")]
    InitializationError(CoreError),

    #[error("Error loading model: {0}")]
    LoadModelError(CoreError),

    #[error("SafeTensor model: {0}")]
    SafeTensorError(CoreError),

    #[error("Tokenization error: {0}")]
    TokenError(TokenError),

    #[error("Uninitialized model error: {0}")]
    UninitializedModelError(CoreError),

    #[error("Unexpected DType: {0:?}")]
    UnexpectedDTypeError(CoreError),

    #[error("Unexpected error: {0}")]
    UnexpectedError(CoreError),

    #[error("Unsupported DType: {0:?}")]
    UnsupportedDTypeError(DType),

    #[error("Wrapped Candle error: {0}")]
    WrappedCandleError(CoreError),
}

// Convert from CoreError to CandleError
impl From<CoreError> for CandleError {
    fn from(error: CoreError) -> Self {
        match error {
            CoreError::Io(err) => {
                CandleError::CandleIoError(err)
            }

            // For variants with simple messages or no extra data, just preserve the message
            CoreError::SafeTensor(inner) => {
                CandleError::SafeTensorError(inner)
            }

            CoreError::UnexpectedDType { msg, expected, got } => {
                CandleError::UnexpectedDTypeError(format!("{}: expected {:?}, got {:?}", msg, expected, got))
            }

            // For variants with simple messages or no extra data, just preserve the message
            CoreError::Msg(msg) => {
                CandleError::UnexpectedError(msg)
            }

            CoreError::Wrapped(inner) => {
                // Attempt to downcast to known error types, or handle generically
                CandleError::WrappedCandleError(inner)
            }

            // Handle all other variants, potentially with a catch-all
            _ => CandleError::GenericError(format!("Unsupported error: {:?}", error)),
        }
    }
}

// Convert from CandleError to ClientError
impl From<CandleError> for ClientError {
    fn from(error: CandleError) -> Self {
        match error {
            CandleError::CandleIoError(err) => ClientError::SpecificError(format!("Candle IO error: {}", err)),
            CandleError::CoreError(err) => ClientError::SpecificError(format!("Candle core error: {}", err)),
            CandleError::CudaError(err) => ClientError::SpecificError(format!("Cuda error: {}", err)),
            CandleError::DecodingError(err) => ClientError::SpecificError(format!("Decoding error: {}", err)),
            CandleError::DownloadError(err) => ClientError::SpecificError(format!("Download error: {}", err)),
            CandleError::GenericError(err) => ClientError::GenericError(format!("Generic client error: {}", err)),
            CandleError::EncodingError(err) => ClientError::SpecificError(format!("Encoding error: {}", err)),
            CandleError::InitializationError(err) => ClientError::SpecificError(format!("Initialization error: {}", err)),
            CandleError::LoadModelError(err) => ClientError::SpecificError(format!("Error loading model: {}", err)),
            CandleError::SafeTensorError(err) => ClientError::SpecificError(format!("SafeTensor error: {}", err)),
            CandleError::TokenError(err) => ClientError::SpecificError(format!("Token error: {}", err)),
            CandleError::UninitializedModelError(err) => ClientError::SpecificError(format!("Uninitialized model error: {}", err)),
            CandleError::UnexpectedDTypeError(err) => ClientError::SpecificError(format!("Unexpected DType: {}", err)),
            CandleError::UnexpectedError(err) => ClientError::SpecificError(format!("Unexpected error: {}", err)),
            CandleError::UnsupportedDTypeError(dtype) => ClientError::SpecificError(format!("Unsupported DType: {:?}", dtype)),
            CandleError::WrappedCandleError(err) => ClientError::SpecificError(format!("Wrapped Candle error: {}", err)),
        }
    }
}

// Convert from TokenError to CandleError
impl From<TokenError> for CandleError {
    fn from(error: TokenError) -> Self {
        CandleError::TokenError(error)
    }
}

// Additional Error Implementations
impl From<Box<dyn StdError>> for CandleError {
    fn from(error: Box<dyn StdError>) -> Self {
        if let Some(core_err) = error.downcast_ref::<CoreError>() {
            return CandleError::from(core_err.clone());
        }
        if let Some(token_err) = error.downcast_ref::<TokenError>() {
            return CandleError::from(token_err.clone());
        }

        CandleError::WrappedCandleError(error.to_string())
    }
}