use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Configuration for Standard Deviation calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StandardDeviationConfig {
    /// Period for standard deviation calculation (default: 20)
    pub period: usize,
    /// Whether to use sample standard deviation (n-1) or population (n) (default: sample)
    pub use_sample: bool,
}

impl Default for StandardDeviationConfig {
    fn default() -> Self {
        Self {
            period: 20,
            use_sample: true,
        }
    }
}

/// Input data for Standard Deviation calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StandardDeviationInput {
    /// Value to calculate standard deviation for
    pub value: f64,
}

/// Volatility classification based on standard deviation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VolatilityLevel {
    /// Very low volatility
    VeryLow,
    /// Low volatility
    Low,
    /// Normal volatility
    Normal,
    /// High volatility
    High,
    /// Very high volatility
    VeryHigh,
    /// Not enough data yet
    Insufficient,
}

/// Output from Standard Deviation calculation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StandardDeviationOutput {
    /// Standard deviation value
    pub std_dev: f64,
    /// Variance (std_dev squared)
    pub variance: f64,
    /// Mean of the values in the period
    pub mean: f64,
    /// Current input value
    pub current_value: f64,
    /// Z-score of current value (how many std devs from mean)
    pub z_score: f64,
    /// Coefficient of variation (std_dev / mean) - relative volatility
    pub coefficient_of_variation: f64,
    /// Volatility level classification
    pub volatility_level: VolatilityLevel,
}

/// Standard Deviation calculation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardDeviationState {
    /// Configuration
    pub config: StandardDeviationConfig,
    /// History of values for period calculation
    pub values: VecDeque<f64>,
    /// Sum of values (for mean calculation)
    pub sum: f64,
    /// Sum of squared values (for variance calculation)
    pub sum_squared: f64,
    /// Whether we have enough data for calculation
    pub has_sufficient_data: bool,
    /// Current mean value
    pub current_mean: f64,
}

impl StandardDeviationState {
    pub fn new(config: StandardDeviationConfig) -> Self {
        Self {
            config,
            values: VecDeque::with_capacity(config.period),
            sum: 0.0,
            sum_squared: 0.0,
            has_sufficient_data: false,
            current_mean: 0.0,
        }
    }
}

/// Error types for Standard Deviation calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandardDeviationError {
    /// Invalid input data
    InvalidInput(String),
    /// Invalid value (NaN or infinite)
    InvalidValue,
    /// Invalid period (must be > 1 for sample, > 0 for population)
    InvalidPeriod,
    /// Division by zero in calculation
    DivisionByZero,
}
