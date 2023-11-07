// src/gateway/clients/client_error.rs

/// Client Error Handler
/// Generic error handler for client-related issues.
/// Handles errors that are common across different clients.

// Core Crates
use thiserror::Error;

// Networking Crates
use crate::gateway::gateway_error::GatewayError;
use crate::network_error::NetworkError;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Generic client error: {0}")]
    GenericError(String),

    #[error("Rate limiting error: {0}")]
    RateLimitError(String),

    #[error("Client-specific error: {0}")]
    SpecificError(String),
}

// Convert from ClientError to GatewayError
impl From<ClientError> for GatewayError {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::AuthenticationError(client_error) => GatewayError::ClientError(format!("Authentication error: {}", client_error)),
            ClientError::GenericError(client_error) => GatewayError::ClientError(format!("Generic client error: {}", client_error)),
            ClientError::RateLimitError(client_error) => GatewayError::ClientError(format!("Rate limit error: {}", client_error)),
            ClientError::SpecificError(client_error) => GatewayError::ClientError(format!("Client-specific error: {}", client_error)),
        }
    }
}

// Convert from ClientError to NetworkError
impl From<ClientError> for NetworkError {
    fn from(error: ClientError) -> Self {
        NetworkError::ClientError(error)
    }
}