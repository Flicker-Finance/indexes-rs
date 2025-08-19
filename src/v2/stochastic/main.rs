use crate::v2::stochastic::types::StochasticSignal;

use super::types::{OHLCData, StochasticError, StochasticResult};
use std::collections::VecDeque;

/// Stochastic Oscillator Calculator
///
/// The Stochastic Oscillator is a momentum indicator that shows the location of the close
/// relative to the high-low range over a set number of periods.
///
/// %K = (Current Close - Lowest Low) / (Highest High - Lowest Low) * 100
/// %D = SMA of %K over smoothing period
///
/// Typical settings: (14, 3, 3) or (5, 3, 3) for faster signals
pub struct StochasticOscillator {
    period: usize,               // Lookback period for highest high and lowest low
    k_smooth: usize,             // Smoothing period for %K (typically 3)
    d_period: usize,             // Period for %D (SMA of %K, typically 3)
    highs: VecDeque<f64>,        // Rolling window of highs
    lows: VecDeque<f64>,         // Rolling window of lows
    closes: VecDeque<f64>,       // Rolling window of closes
    k_values: VecDeque<f64>,     // Rolling window of %K values for %D calculation
    raw_k_values: VecDeque<f64>, // Raw %K values for smoothing
}

impl StochasticOscillator {
    /// Creates a new Stochastic Oscillator with specified parameters
    ///
    /// # Arguments
    /// * `period` - Lookback period for high/low (typically 14)
    /// * `k_smooth` - Smoothing period for %K (typically 3)
    /// * `d_period` - Period for %D calculation (typically 3)
    pub fn new(period: usize, k_smooth: usize, d_period: usize) -> Result<Self, StochasticError> {
        if period == 0 {
            return Err(StochasticError::InvalidPeriod);
        }
        if k_smooth == 0 || d_period == 0 {
            return Err(StochasticError::InvalidSmoothingPeriod);
        }

        Ok(Self {
            period,
            k_smooth,
            d_period,
            highs: VecDeque::with_capacity(period),
            lows: VecDeque::with_capacity(period),
            closes: VecDeque::with_capacity(k_smooth),
            k_values: VecDeque::with_capacity(d_period),
            raw_k_values: VecDeque::with_capacity(k_smooth),
        })
    }

    /// Creates a new Stochastic Oscillator with default parameters (14, 3, 3)
    pub fn default() -> Self {
        Self::new(14, 3, 3).unwrap()
    }

    /// Updates the oscillator with OHLC data
    pub fn update(&mut self, data: OHLCData) -> Option<StochasticResult> {
        // Validate input
        if !self.validate_ohlc(&data) {
            return None;
        }

        // Add to rolling windows
        self.add_to_windows(data.high, data.low, data.close);

        // Calculate if we have enough data
        if self.highs.len() >= self.period {
            self.calculate()
        } else {
            None
        }
    }

    /// Updates with separate high, low, close values
    pub fn update_hlc(&mut self, high: f64, low: f64, close: f64) -> Option<StochasticResult> {
        self.update(OHLCData::new(high, low, close))
    }

    /// Returns the current Stochastic values
    pub fn value(&self) -> Option<StochasticResult> {
        if self.k_values.is_empty() {
            return None;
        }

        let k = *self.k_values.back()?;
        let d = if self.k_values.len() >= self.d_period {
            self.k_values.iter().rev().take(self.d_period).sum::<f64>() / self.d_period as f64
        } else {
            self.k_values.iter().sum::<f64>() / self.k_values.len() as f64
        };

        Some(StochasticResult { k, d })
    }

    /// Resets the oscillator
    pub fn reset(&mut self) {
        self.highs.clear();
        self.lows.clear();
        self.closes.clear();
        self.k_values.clear();
        self.raw_k_values.clear();
    }

    /// Checks if the oscillator is ready (has enough data)
    pub fn is_ready(&self) -> bool {
        self.highs.len() >= self.period && !self.k_values.is_empty()
    }

    /// Get the period
    pub fn period(&self) -> usize {
        self.period
    }

    /// Batch calculation for historical data
    pub fn calculate_batch(period: usize, k_smooth: usize, d_period: usize, data: &[OHLCData]) -> Result<Vec<Option<StochasticResult>>, StochasticError> {
        let mut stoch = Self::new(period, k_smooth, d_period)?;
        let mut results = Vec::with_capacity(data.len());

        for ohlc in data {
            results.push(stoch.update(*ohlc));
        }

        Ok(results)
    }

    /// Fast Stochastic (no smoothing)
    pub fn fast(period: usize) -> Result<Self, StochasticError> {
        Self::new(period, 1, 3)
    }

    /// Slow Stochastic (standard smoothing)
    pub fn slow(period: usize) -> Result<Self, StochasticError> {
        Self::new(period, 3, 3)
    }

    /// Full Stochastic (customizable smoothing)
    pub fn full(period: usize, k_smooth: usize, d_period: usize) -> Result<Self, StochasticError> {
        Self::new(period, k_smooth, d_period)
    }

    // === Private methods ===

    fn validate_ohlc(&self, data: &OHLCData) -> bool {
        // Check for NaN or Infinite
        if data.high.is_nan() || data.low.is_nan() || data.close.is_nan() {
            return false;
        }
        if data.high.is_infinite() || data.low.is_infinite() || data.close.is_infinite() {
            return false;
        }
        // Check OHLC relationship
        if data.high < data.low {
            return false;
        }
        // Close should be between high and low (with small tolerance for rounding)
        let tolerance = 0.0001;
        if data.close > data.high + tolerance || data.close < data.low - tolerance {
            return false;
        }
        true
    }

    fn add_to_windows(&mut self, high: f64, low: f64, close: f64) {
        // Add to highs window
        if self.highs.len() >= self.period {
            self.highs.pop_front();
        }
        self.highs.push_back(high);

        // Add to lows window
        if self.lows.len() >= self.period {
            self.lows.pop_front();
        }
        self.lows.push_back(low);

        // Add to closes window (for smoothing)
        if self.closes.len() >= self.k_smooth {
            self.closes.pop_front();
        }
        self.closes.push_back(close);
    }

    fn calculate(&mut self) -> Option<StochasticResult> {
        // Find highest high and lowest low in the period
        let highest_high = self.highs.iter().fold(f64::MIN, |a, &b| a.max(b));
        let lowest_low = self.lows.iter().fold(f64::MAX, |a, &b| a.min(b));

        // Calculate raw %K
        let range = highest_high - lowest_low;
        let raw_k = if range > 0.0 {
            let current_close = *self.closes.back()?;
            ((current_close - lowest_low) / range) * 100.0
        } else {
            50.0 // If range is 0 (all prices are the same), use 50%
        };

        // Add to raw K values for smoothing
        if self.raw_k_values.len() >= self.k_smooth {
            self.raw_k_values.pop_front();
        }
        self.raw_k_values.push_back(raw_k);

        // Calculate smoothed %K if we have enough values
        if self.raw_k_values.len() >= self.k_smooth {
            let smoothed_k = self.raw_k_values.iter().sum::<f64>() / self.k_smooth as f64;

            // Add to K values for %D calculation
            if self.k_values.len() >= self.d_period {
                self.k_values.pop_front();
            }
            self.k_values.push_back(smoothed_k);

            // Calculate %D
            let d = if self.k_values.len() >= self.d_period {
                self.k_values.iter().sum::<f64>() / self.d_period as f64
            } else {
                self.k_values.iter().sum::<f64>() / self.k_values.len() as f64
            };

            Some(StochasticResult { k: smoothed_k, d })
        } else {
            None
        }
    }

    /// Get crossover signal
    pub fn signal(&self) -> Option<StochasticSignal> {
        let result = self.value()?;

        if result.k > 80.0 && result.d > 80.0 {
            Some(StochasticSignal::Overbought)
        } else if result.k < 20.0 && result.d < 20.0 {
            Some(StochasticSignal::Oversold)
        } else if result.k > result.d {
            Some(StochasticSignal::Bullish)
        } else if result.k < result.d {
            Some(StochasticSignal::Bearish)
        } else {
            Some(StochasticSignal::Neutral)
        }
    }
}
