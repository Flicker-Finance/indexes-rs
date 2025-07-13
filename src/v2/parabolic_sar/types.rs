use serde::{Deserialize, Serialize};

/// Configuration for Parabolic SAR calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParabolicSARConfig {
    /// Initial acceleration factor (default: 0.02)
    pub acceleration_start: f64,
    /// Acceleration increment (default: 0.02)
    pub acceleration_increment: f64,
    /// Maximum acceleration factor (default: 0.20)
    pub acceleration_maximum: f64,
}

impl Default for ParabolicSARConfig {
    fn default() -> Self {
        Self {
            acceleration_start: 0.02,
            acceleration_increment: 0.02,
            acceleration_maximum: 0.20,
        }
    }
}

/// Input data for Parabolic SAR calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParabolicSARInput {
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price (optional, used for initial trend determination)
    pub close: Option<f64>,
}

/// Current trend direction
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrendDirection {
    /// Uptrend (SAR below price)
    Up,
    /// Downtrend (SAR above price)
    Down,
}

/// Output from Parabolic SAR calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ParabolicSAROutput {
    /// Parabolic SAR value
    pub sar: f64,
    /// Current trend direction
    pub trend: TrendDirection,
    /// Current acceleration factor
    pub acceleration_factor: f64,
    /// Extreme point (highest high in uptrend, lowest low in downtrend)
    pub extreme_point: f64,
    /// Whether a trend reversal occurred
    pub trend_reversal: bool,
    /// Number of periods in current trend
    pub trend_periods: usize,
}

/// Parabolic SAR calculation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParabolicSARState {
    /// Configuration
    pub config: ParabolicSARConfig,
    /// Current trend direction
    pub trend: Option<TrendDirection>,
    /// Current SAR value
    pub current_sar: Option<f64>,
    /// Current acceleration factor
    pub acceleration_factor: f64,
    /// Extreme point in current trend
    pub extreme_point: Option<f64>,
    /// Previous period's high
    pub previous_high: Option<f64>,
    /// Previous period's low
    pub previous_low: Option<f64>,
    /// Previous period's close
    pub previous_close: Option<f64>,
    /// Number of periods in current trend
    pub trend_periods: usize,
    /// Whether this is the first calculation
    pub is_first: bool,
    /// Whether this is the second calculation
    pub is_second: bool,
}

impl ParabolicSARState {
    pub fn new(config: ParabolicSARConfig) -> Self {
        Self {
            config,
            trend: None,
            current_sar: None,
            acceleration_factor: config.acceleration_start,
            extreme_point: None,
            previous_high: None,
            previous_low: None,
            previous_close: None,
            trend_periods: 0,
            is_first: true,
            is_second: false,
        }
    }
}

/// Error types for Parabolic SAR calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParabolicSARError {
    /// Invalid input data
    InvalidInput(String),
    /// Invalid HL relationship (high < low)
    InvalidHL,
    /// Invalid price (NaN or infinite)
    InvalidPrice,
    /// Invalid acceleration parameters
    InvalidAcceleration,
    /// Close price out of range (not between high and low)
    CloseOutOfRange,
}
