//! # Stochastic Oscillator Module
//!
//! This module implements a Stochastic Oscillator indicator. It computes %K and %D values based
//! on a sliding window of prices. The oscillator can be smoothed using a `%K smoothing` parameter.
//!
//! The indicator also generates a trading signal, a condition, a crossover type, and a strength value.
//!
//! # Example
//!
//! ```rust
//! use indexes_rs::v1::stochastic::main::StochasticOscillator;
//! use indexes_rs::v1::stochastic::types::{StochResult, StochSignal, StochCondition, StochCrossover};
//!
//! let mut stoch = StochasticOscillator::new(
//!     StochasticOscillator::DEFAULT_PERIOD,
//!     StochasticOscillator::DEFAULT_K_SMOOTH,
//!     StochasticOscillator::DEFAULT_D_PERIOD
//! );
//!
//! // Feed in a series of prices (for example, closing prices).
//! let prices = vec![100.0, 102.0, 101.5, 103.0, 104.0, 102.5, 101.0, 100.5, 99.5, 98.0, 97.5, 98.5];
//! let mut result: Option<StochResult> = None;
//! for price in prices {
//!     result = stoch.calculate(price);
//! }
//!
//! if let Some(res) = result {
//!     println!("K value: {:.2}", res.k_value);
//!     println!("D value: {:.2}", res.d_value);
//!     println!("Signal: {:?}", res.signal);
//!     println!("Condition: {:?}", res.condition);
//!     println!("Crossover: {:?}", res.crossover);
//!     println!("Strength: {:.2}%", res.strength);
//! }
//! ```

use super::types::*; // This module should define StochResult, StochSignal, StochCondition, and StochCrossover.

/// A Stochastic Oscillator indicator.
pub struct StochasticOscillator {
    period: usize,   // %K period (typically 14)
    k_smooth: usize, // %K smoothing period (typically 3)
    d_period: usize, // %D period (typically 3)
    prices: Vec<f64>,
    highs: Vec<f64>,
    lows: Vec<f64>,
    /// Stores the smoothed %K values (used for %D calculation).
    k_values: Vec<f64>,
    /// Stores raw %K values for smoothing calculation.
    raw_k_values: Vec<f64>,
}

impl StochasticOscillator {
    /// Default %K period (typically 14).
    pub const DEFAULT_PERIOD: usize = 14;
    /// Default %K smoothing period (typically 3).
    pub const DEFAULT_K_SMOOTH: usize = 3;
    /// Default %D period (typically 3).
    pub const DEFAULT_D_PERIOD: usize = 3;

    const OVERBOUGHT_THRESHOLD: f64 = 80.0;
    const OVERSOLD_THRESHOLD: f64 = 20.0;

    /// Creates a new StochasticOscillator with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `period` - The period for %K calculation.
    /// * `k_smooth` - The smoothing period for %K.
    /// * `d_period` - The period for %D calculation.
    pub fn new(period: usize, k_smooth: usize, d_period: usize) -> Self {
        StochasticOscillator {
            period,
            k_smooth,
            d_period,
            prices: Vec::new(),
            highs: Vec::new(),
            lows: Vec::new(),
            k_values: Vec::new(),
            raw_k_values: Vec::new(),
        }
    }

    /// Updates the oscillator with a new price and returns the current oscillator result.
    ///
    /// The oscillator calculates the raw %K value based on the current window of prices,
    /// applies smoothing if enough values exist, and then computes the %D value. It also
    /// generates a trading signal, a condition, and detects a crossover.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price.
    ///
    /// # Returns
    ///
    /// * `Some(StochResult)` if there is sufficient data, else `None`.
    pub fn calculate(&mut self, price: f64) -> Option<StochResult> {
        // Update price history (for simplicity, highs and lows are the same as price).
        self.prices.push(price);
        self.highs.push(price);
        self.lows.push(price);

        // Keep sliding window for price/high/low arrays.
        if self.prices.len() > self.period {
            self.prices.remove(0);
            self.highs.remove(0);
            self.lows.remove(0);
        }

        if self.prices.len() < self.period {
            return None;
        }

        // Determine the highest high and lowest low in the window.
        let highest_high = self.highs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let lowest_low = self.lows.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        // Compute raw %K value. If highest equals lowest, use 50.
        let raw_k = if highest_high == lowest_low {
            50.0
        } else {
            ((price - lowest_low) / (highest_high - lowest_low)) * 100.0
        };

        // Store raw %K for smoothing.
        self.raw_k_values.push(raw_k);
        if self.raw_k_values.len() > self.k_smooth {
            self.raw_k_values.remove(0);
        }

        // Compute the smoothed %K value if we have enough raw values; otherwise, use raw_k.
        let k = if self.raw_k_values.len() < self.k_smooth {
            raw_k
        } else {
            self.raw_k_values.iter().sum::<f64>() / self.k_smooth as f64
        };

        // Store the smoothed %K value for %D calculation.
        self.k_values.push(k);
        if self.k_values.len() > self.d_period {
            self.k_values.remove(0);
        }

        let d = self.calculate_d();
        let signal = self.generate_signal(k, d);
        let condition = self.determine_condition(k);
        let crossover = self.detect_crossover(k, d);
        let strength = self.calculate_strength(k, d);

        Some(StochResult {
            k_value: k,
            d_value: d,
            signal,
            condition,
            crossover,
            strength,
        })
    }

    /// Calculates the %D value as the average of the last `d_period` %K values.
    fn calculate_d(&self) -> f64 {
        if self.k_values.len() < self.d_period {
            self.k_values.last().copied().unwrap_or(50.0)
        } else {
            self.k_values.iter().sum::<f64>() / self.d_period as f64
        }
    }

    /// Generates a trading signal based on %K and %D.
    fn generate_signal(&self, k: f64, d: f64) -> StochSignal {
        match (k, d) {
            (k, d) if k > d && k < Self::OVERBOUGHT_THRESHOLD => StochSignal::Buy,
            (k, d) if k < d && k > Self::OVERSOLD_THRESHOLD => StochSignal::Sell,
            (k, _) if k >= Self::OVERBOUGHT_THRESHOLD => StochSignal::Overbought,
            (k, _) if k <= Self::OVERSOLD_THRESHOLD => StochSignal::Oversold,
            _ => StochSignal::Neutral,
        }
    }

    /// Determines the current condition of the oscillator based on %K.
    fn determine_condition(&self, k: f64) -> StochCondition {
        match k {
            k if k >= Self::OVERBOUGHT_THRESHOLD => StochCondition::Overbought,
            k if k <= Self::OVERSOLD_THRESHOLD => StochCondition::Oversold,
            k if k > 50.0 => StochCondition::Strong,
            k if k < 50.0 => StochCondition::Weak,
            _ => StochCondition::Neutral,
        }
    }

    /// Detects if a crossover occurred between %K and %D.
    ///
    /// A bullish crossover is detected when %K crosses above %D, and bearish when %K crosses below %D.
    fn detect_crossover(&self, k: f64, d: f64) -> StochCrossover {
        if self.k_values.len() < 2 {
            return StochCrossover::None;
        }
        let prev_k = self.k_values[self.k_values.len() - 2];
        if k > d && prev_k <= d {
            StochCrossover::Bullish
        } else if k < d && prev_k >= d {
            StochCrossover::Bearish
        } else {
            StochCrossover::None
        }
    }

    /// Calculates an overall strength value (0-100) for the oscillator.
    ///
    /// Combines the deviation of %K from 50 and the absolute difference between %K and %D.
    fn calculate_strength(&self, k: f64, d: f64) -> f64 {
        let trend_strength = (k - 50.0).abs() / 50.0;
        let momentum = (k - d).abs() / 20.0; // Normalized difference
        ((trend_strength + momentum) / 2.0 * 100.0).min(100.0)
    }
}
