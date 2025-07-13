use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Configuration for MFI calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MFIConfig {
    /// Period for MFI calculation (default: 14)
    pub period: usize,
    /// Overbought threshold (default: 80.0)
    pub overbought: f64,
    /// Oversold threshold (default: 20.0)
    pub oversold: f64,
}

impl Default for MFIConfig {
    fn default() -> Self {
        Self {
            period: 14,
            overbought: 80.0,
            oversold: 20.0,
        }
    }
}

/// Input data for MFI calculation (OHLCV)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MFIInput {
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Volume
    pub volume: f64,
}

/// Raw Money Flow data point
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MoneyFlow {
    /// Typical price ((H+L+C)/3)
    pub typical_price: f64,
    /// Raw money flow (typical_price * volume)
    pub raw_money_flow: f64,
    /// Money flow direction (1.0 = positive, -1.0 = negative, 0.0 = neutral)
    pub flow_direction: f64,
}

/// Output from MFI calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MFIOutput {
    /// Money Flow Index value (0-100)
    pub mfi: f64,
    /// Current typical price
    pub typical_price: f64,
    /// Current raw money flow
    pub raw_money_flow: f64,
    /// Money flow direction
    pub flow_direction: f64,
    /// Market condition based on thresholds
    pub market_condition: MFIMarketCondition,
}

/// Market condition based on MFI value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MFIMarketCondition {
    /// MFI above overbought threshold
    Overbought,
    /// MFI below oversold threshold
    Oversold,
    /// MFI in normal range
    Normal,
    /// Not enough data yet
    Insufficient,
}

/// MFI calculation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MFIState {
    /// Configuration
    pub config: MFIConfig,
    /// History of money flow data points
    pub money_flows: VecDeque<MoneyFlow>,
    /// Previous typical price for comparison
    pub previous_typical_price: Option<f64>,
    /// Sum of positive money flows in current period
    pub positive_money_flow_sum: f64,
    /// Sum of negative money flows in current period
    pub negative_money_flow_sum: f64,
    /// Whether we have enough data for calculation
    pub has_sufficient_data: bool,
}

impl MFIState {
    pub fn new(config: MFIConfig) -> Self {
        Self {
            config,
            money_flows: VecDeque::with_capacity(config.period),
            previous_typical_price: None,
            positive_money_flow_sum: 0.0,
            negative_money_flow_sum: 0.0,
            has_sufficient_data: false,
        }
    }
}

/// Error types for MFI calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MFIError {
    /// Invalid input data
    InvalidInput(String),
    /// Invalid OHLC relationship (e.g., high < low)
    InvalidOHLC,
    /// Negative volume
    NegativeVolume,
    /// Invalid price (NaN or infinite)
    InvalidPrice,
    /// Invalid period (must be > 0)
    InvalidPeriod,
    /// Invalid threshold values
    InvalidThresholds,
    /// Division by zero in calculation
    DivisionByZero,
}
