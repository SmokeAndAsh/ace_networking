// src/southbound_bus.rs

/// Southbound Bus Module
/// Manages control commands and downward directives.

use crate::gateway::clients::candle::ModelConfig;

#[derive(Debug)]
pub enum ControlCommand {
    StartModel(ModelConfig),
    StopModel(String), // Model name or ID
    UpdateModelSettings(String, ModelConfig), // Model name or ID and new settings
    // ... any other commands you need
}