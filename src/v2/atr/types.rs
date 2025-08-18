use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ATRResult {
    pub atr: f64,
    pub true_range: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OHLCData {
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

impl OHLCData {
    pub fn new(high: f64, low: f64, close: f64) -> Self {
        Self { high, low, close }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ATRError {
    InvalidPeriod,
    InsufficientData,
    InvalidPrice,
}

impl std::fmt::Display for ATRError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ATRError::InvalidPeriod => write!(f, "Period must be greater than 0"),
            ATRError::InsufficientData => write!(f, "Not enough data to calculate ATR"),
            ATRError::InvalidPrice => write!(f, "Invalid price data (NaN or Infinite)"),
        }
    }
}

impl std::error::Error for ATRError {}
