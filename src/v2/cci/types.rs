use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Configuration for CCI calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CCIConfig {
    /// Period for CCI calculation (default: 20)
    pub period: usize,
    /// Overbought threshold (default: 100.0)
    pub overbought: f64,
    /// Oversold threshold (default: -100.0)
    pub oversold: f64,
    /// Extreme overbought threshold (default: 200.0)
    pub extreme_overbought: f64,
    /// Extreme oversold threshold (default: -200.0)
    pub extreme_oversold: f64,
}

impl Default for CCIConfig {
    fn default() -> Self {
        Self {
            period: 20,
            overbought: 100.0,
            oversold: -100.0,
            extreme_overbought: 200.0,
            extreme_oversold: -200.0,
        }
    }
}

/// Input data for CCI calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CCIInput {
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
}

/// Market condition based on CCI value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CCIMarketCondition {
    /// CCI above extreme overbought threshold
    ExtremeOverbought,
    /// CCI above overbought threshold
    Overbought,
    /// CCI in normal range
    Normal,
    /// CCI below oversold threshold
    Oversold,
    /// CCI below extreme oversold threshold
    ExtremeOversold,
    /// Not enough data yet
    Insufficient,
}

/// Output from CCI calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CCIOutput {
    /// Commodity Channel Index value
    pub cci: f64,
    /// Current typical price ((H+L+C)/3)
    pub typical_price: f64,
    /// Simple moving average of typical price
    pub sma_tp: f64,
    /// Mean absolute deviation
    pub mean_deviation: f64,
    /// Market condition based on thresholds
    pub market_condition: CCIMarketCondition,
    /// Distance from zero (absolute CCI value)
    pub distance_from_zero: f64,
}

/// CCI calculation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CCIState {
    /// Configuration
    pub config: CCIConfig,
    /// History of typical prices
    pub typical_prices: VecDeque<f64>,
    /// Sum of typical prices (for SMA calculation)
    pub tp_sum: f64,
    /// Whether we have enough data for calculation
    pub has_sufficient_data: bool,
}

impl CCIState {
    pub fn new(config: CCIConfig) -> Self {
        Self {
            config,
            typical_prices: VecDeque::with_capacity(config.period),
            tp_sum: 0.0,
            has_sufficient_data: false,
        }
    }
}

/// Error types for CCI calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CCIError {
    /// Invalid input data
    InvalidInput(String),
    /// Invalid HLC relationship (e.g., high < low)
    InvalidHLC,
    /// Invalid price (NaN or infinite)
    InvalidPrice,
    /// Invalid period (must be > 0)
    InvalidPeriod,
    /// Invalid threshold values
    InvalidThresholds,
    /// Division by zero in calculation
    DivisionByZero,
}
