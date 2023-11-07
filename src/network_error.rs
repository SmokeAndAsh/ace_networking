// src/network_error.rs

/// Network Error Handler
/// Encapsulates general errors that occur in the Networking crate.
/// Handles the most general networking issues like connection failures, timeouts, etc.

// Core Crates
use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::{Error as SerdeError};
use thiserror::Error;
use tokio::io::{Error as TokioIoError};

// Networking Crates
use crate::gateway::gateway_error::GatewayError;
use crate::gateway::clients::client_error::ClientError;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Client error: {0}")]
    ClientError(ClientError),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Custom error: {0}")]
    CustomError(String),

    #[error("Gateway error: {0}")]
    GatewayError(GatewayError),

    #[error("HTTP error: {0}")]
    HttpError(StatusCode),

    #[error("Network IO error: {0}")]
    IoError(#[from] TokioIoError),

    #[error("Request error: {0}")]
    RequestError(ReqwestError),

    #[error("Serde error: {0}")]
    SerdeError(SerdeError),
}

// Implement the kind function for NetworkError
impl NetworkError {
    pub fn kind(&self) -> &'static str {
        match self {
            NetworkError::ClientError(_) => "ClientError",
            NetworkError::ConnectionError(_) => "ConnectionError",
            NetworkError::CustomError(_) => "CustomError",
            NetworkError::GatewayError(_) => "GatewayError",
            NetworkError::HttpError(_) => "HttpError",
            NetworkError::IoError(_) => "IoError",
            NetworkError::RequestError(_) => "RequestError",
            NetworkError::SerdeError(_) => "SerdeError",
        }
    }
}