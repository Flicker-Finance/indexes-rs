use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StochasticResult {
    pub k: f64, // Fast stochastic (%K)
    pub d: f64, // Slow stochastic (%D - moving average of %K)
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StochasticSignal {
    Overbought, // Both K and D > 80
    Oversold,   // Both K and D < 20
    Bullish,    // K > D (bullish crossover)
    Bearish,    // K < D (bearish crossover)
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StochasticError {
    InvalidPeriod,
    InvalidSmoothingPeriod,
    InsufficientData,
    InvalidPrice,
}

impl std::fmt::Display for StochasticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StochasticError::InvalidPeriod => write!(f, "Period must be greater than 0"),
            StochasticError::InvalidSmoothingPeriod => {
                write!(f, "Smoothing period must be greater than 0")
            }
            StochasticError::InsufficientData => {
                write!(f, "Not enough data to calculate Stochastic")
            }
            StochasticError::InvalidPrice => {
                write!(f, "Invalid price data (NaN, Infinite, or high < low)")
            }
        }
    }
}

impl std::error::Error for StochasticError {}
