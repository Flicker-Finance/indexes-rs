use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Configuration for ADX calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ADXConfig {
    /// Period for DI and ADX calculation (default: 14)
    pub period: usize,
    /// Smoothing period for ADX (default: same as period)
    pub adx_smoothing: usize,
    /// Strong trend threshold (default: 25.0)
    pub strong_trend_threshold: f64,
    /// Very strong trend threshold (default: 50.0)
    pub very_strong_trend_threshold: f64,
}

impl Default for ADXConfig {
    fn default() -> Self {
        Self {
            period: 14,
            adx_smoothing: 14,
            strong_trend_threshold: 25.0,
            very_strong_trend_threshold: 50.0,
        }
    }
}

/// Input data for ADX calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ADXInput {
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
}

/// Trend strength classification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrendStrength {
    /// ADX below 25 - weak or no trend
    Weak,
    /// ADX between 25-50 - strong trend
    Strong,
    /// ADX above 50 - very strong trend
    VeryStrong,
    /// Not enough data yet
    Insufficient,
}

/// Trend direction based on DI comparison
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrendDirection {
    /// +DI > -DI (uptrend)
    Up,
    /// -DI > +DI (downtrend)
    Down,
    /// +DI = -DI (sideways)
    Sideways,
}

/// Output from ADX calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ADXOutput {
    /// Average Directional Index (trend strength)
    pub adx: f64,
    /// Plus Directional Indicator
    pub plus_di: f64,
    /// Minus Directional Indicator
    pub minus_di: f64,
    /// Directional Index (|+DI - -DI| / (+DI + -DI) * 100)
    pub dx: f64,
    /// True Range for current period
    pub true_range: f64,
    /// Trend strength classification
    pub trend_strength: TrendStrength,
    /// Trend direction based on DI comparison
    pub trend_direction: TrendDirection,
    /// DI spread (+DI - -DI)
    pub di_spread: f64,
}

/// Internal calculation data for a single period
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ADXPeriodData {
    /// True Range
    pub true_range: f64,
    /// Plus Directional Movement
    pub plus_dm: f64,
    /// Minus Directional Movement
    pub minus_dm: f64,
    /// Plus DI
    pub plus_di: f64,
    /// Minus DI
    pub minus_di: f64,
    /// DX value
    pub dx: f64,
}

/// ADX calculation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ADXState {
    /// Configuration
    pub config: ADXConfig,
    /// Previous high (for DM calculation)
    pub previous_high: Option<f64>,
    /// Previous low (for DM calculation)
    pub previous_low: Option<f64>,
    /// Previous close (for TR calculation)
    pub previous_close: Option<f64>,
    /// History of period data for smoothing
    pub period_data: VecDeque<ADXPeriodData>,
    /// Smoothed True Range sum
    pub smoothed_tr: Option<f64>,
    /// Smoothed Plus DM sum
    pub smoothed_plus_dm: Option<f64>,
    /// Smoothed Minus DM sum
    pub smoothed_minus_dm: Option<f64>,
    /// History of DX values for ADX calculation
    pub dx_history: VecDeque<f64>,
    /// Current ADX value (for smoothing)
    pub current_adx: Option<f64>,
    /// Whether we have enough data for DI calculation
    pub has_di_data: bool,
    /// Whether we have enough data for ADX calculation
    pub has_adx_data: bool,
    /// Is first calculation
    pub is_first: bool,
}

impl ADXState {
    pub fn new(config: ADXConfig) -> Self {
        Self {
            config,
            previous_high: None,
            previous_low: None,
            previous_close: None,
            period_data: VecDeque::with_capacity(config.period),
            smoothed_tr: None,
            smoothed_plus_dm: None,
            smoothed_minus_dm: None,
            dx_history: VecDeque::with_capacity(config.adx_smoothing),
            current_adx: None,
            has_di_data: false,
            has_adx_data: false,
            is_first: true,
        }
    }
}

/// Error types for ADX calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ADXError {
    /// Invalid input data
    InvalidInput(String),
    /// Invalid HLC relationship
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
