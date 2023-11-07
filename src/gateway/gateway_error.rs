// src/gateway/api_error.rs

/// Gateway Error Handler
/// Handles errors that occur within the gateway module, like data processing, routing, etc.
/// Not specific to a particular client.

// Core Crates
use thiserror::Error;

// Networking Crates
use crate::gateway::clients::client_error::ClientError;
use crate::network_error::NetworkError;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Gateway error: {0}")]
    GatewayError(String),

    #[error("Client error: {0}")]
    ClientError(ClientError),
}

// Convert from GatewayError to ClientError
impl From<GatewayError> for ClientError {
    fn from(error: GatewayError) -> Self {
        match error {
            GatewayError::ClientError(client_error) => client_error,
            GatewayError::GatewayError(msg) => ClientError::GenericError(msg),
        }
    }
}

// Convert from GatewayError to NetworkError
impl From<GatewayError> for NetworkError {
    fn from(error: GatewayError) -> Self {
        match error {
            GatewayError::ClientError(client_error) => NetworkError::ClientError(client_error),
            GatewayError::GatewayError(msg) => NetworkError::CustomError(msg),
        }
    }
}