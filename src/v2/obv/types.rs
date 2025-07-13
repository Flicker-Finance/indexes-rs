use serde::{Deserialize, Serialize};

/// Configuration for OBV calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OBVConfig {
    /// Whether to use cumulative calculation (default: true)
    pub cumulative: bool,
}

impl Default for OBVConfig {
    fn default() -> Self {
        Self { cumulative: true }
    }
}

/// Input data for OBV calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OBVInput {
    /// Current closing price
    pub close: f64,
    /// Current volume
    pub volume: f64,
}

/// Output from OBV calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OBVOutput {
    /// On Balance Volume value
    pub obv: f64,
    /// Optional: Volume flow direction (1.0 = up, -1.0 = down, 0.0 = unchanged)
    pub flow_direction: f64,
}

/// OBV calculation state
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OBVState {
    /// Previous closing price
    pub previous_close: Option<f64>,
    /// Current cumulative OBV value
    pub cumulative_obv: f64,
    /// Configuration
    pub config: OBVConfig,
    /// Whether this is the first calculation
    pub is_first: bool,
}

impl OBVState {
    pub fn new(config: OBVConfig) -> Self {
        Self {
            previous_close: None,
            cumulative_obv: 0.0,
            config,
            is_first: true,
        }
    }
}

/// Error types for OBV calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OBVError {
    /// Invalid input data
    InvalidInput(String),
    /// Negative volume provided
    NegativeVolume,
    /// Invalid price (NaN or infinite)
    InvalidPrice,
}
