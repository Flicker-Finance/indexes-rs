use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Configuration for Williams %R calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WilliamsRConfig {
    /// Lookback period for highest high and lowest low (default: 14)
    pub period: usize,
    /// Overbought threshold (default: -20.0)
    pub overbought: f64,
    /// Oversold threshold (default: -80.0)
    pub oversold: f64,
    /// Extreme overbought threshold (default: -10.0)
    pub extreme_overbought: f64,
    /// Extreme oversold threshold (default: -90.0)
    pub extreme_oversold: f64,
}

impl Default for WilliamsRConfig {
    fn default() -> Self {
        Self {
            period: 14,
            overbought: -20.0,
            oversold: -80.0,
            extreme_overbought: -10.0,
            extreme_oversold: -90.0,
        }
    }
}

/// Input data for Williams %R calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WilliamsRInput {
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
}

/// Market condition based on Williams %R value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WilliamsRMarketCondition {
    /// Williams %R above extreme overbought threshold (near 0)
    ExtremeOverbought,
    /// Williams %R above overbought threshold
    Overbought,
    /// Williams %R in normal range
    Normal,
    /// Williams %R below oversold threshold
    Oversold,
    /// Williams %R below extreme oversold threshold (near -100)
    ExtremeOversold,
    /// Not enough data yet
    Insufficient,
}

/// Output from Williams %R calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WilliamsROutput {
    /// Williams %R value (ranges from 0 to -100)
    pub williams_r: f64,
    /// Highest high in the lookback period
    pub highest_high: f64,
    /// Lowest low in the lookback period
    pub lowest_low: f64,
    /// Current close price
    pub close: f64,
    /// Price range (highest_high - lowest_low)
    pub price_range: f64,
    /// Market condition based on thresholds
    pub market_condition: WilliamsRMarketCondition,
    /// Distance from overbought level (useful for momentum analysis)
    pub distance_from_overbought: f64,
    /// Distance from oversold level (useful for momentum analysis)
    pub distance_from_oversold: f64,
}

/// Williams %R calculation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WilliamsRState {
    /// Configuration
    pub config: WilliamsRConfig,
    /// History of high prices for period lookback
    pub highs: VecDeque<f64>,
    /// History of low prices for period lookback
    pub lows: VecDeque<f64>,
    /// Current highest high in the period
    pub highest_high: f64,
    /// Current lowest low in the period
    pub lowest_low: f64,
    /// Whether we have enough data for calculation
    pub has_sufficient_data: bool,
}

impl WilliamsRState {
    pub fn new(config: WilliamsRConfig) -> Self {
        Self {
            config,
            highs: VecDeque::with_capacity(config.period),
            lows: VecDeque::with_capacity(config.period),
            highest_high: f64::NEG_INFINITY,
            lowest_low: f64::INFINITY,
            has_sufficient_data: false,
        }
    }
}

/// Error types for Williams %R calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WilliamsRError {
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
    /// Division by zero in calculation (price range is zero)
    DivisionByZero,
}
