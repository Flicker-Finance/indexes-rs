//! A Relative Strength Index (RSI) calculator module.
//!
//! This module provides an implementation for calculating the Relative Strength Index
//! (RSI) over a configurable period with customizable overbought and oversold thresholds.
//!
//! The RSI is a momentum oscillator that measures the speed and change of price movements.
//! It is used to indicate overbought or oversold market conditions.
//!
//! # Examples
//!
//! Using the default thresholds (70 for overbought, 30 for oversold):
//!
//! ```rust
//! use indexes_rs::v1::rsi::main::RSI;
//! use indexes_rs::v1::sma::main::SMAError;
//! use indexes_rs::v1::rsi::types::RSIResult;
//! use indexes_rs::v1::types::TrendDirection;
//!
//! let mut rsi = RSI::new(14, None, None);
//! let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42];
//!
//! for price in prices {
//!     if let Some(result) = rsi.calculate(price) {
//!         println!("RSI: {:.2}, Condition: {:?}", result.value, result.condition);
//!     }
//! }
//! ```
//!
//! Using custom thresholds (e.g., 80 for overbought and 20 for oversold):
//!
//! ```rust
//! use indexes_rs::v1::rsi::main::RSI;
//! let mut rsi = RSI::new(14, Some(80.0), Some(20.0));
//! ```

use super::types::{MarketCondition, RSIResult};
use std::collections::VecDeque;

/// A struct for calculating the Relative Strength Index (RSI) with customizable thresholds.
///
/// The RSI calculator maintains a sliding window of gains and losses based on incoming
/// price data. Once sufficient data is collected (i.e. equal to the specified period),
/// it returns the current RSI value along with an indication of market condition.
/// Users can customize the overbought and oversold thresholds via the constructor.
pub struct RSI {
    period: usize,
    gains: VecDeque<f64>,
    losses: VecDeque<f64>,
    sum_gains: f64,
    sum_losses: f64,
    prev_price: Option<f64>,
    /// The overbought threshold (default is 70.0).
    overbought: f64,
    /// The oversold threshold (default is 30.0).
    oversold: f64,
}

impl RSI {
    /// Creates a new RSI calculator with the specified period and optional thresholds.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods over which to calculate the RSI.
    /// * `overbought` - Optional overbought threshold. If `None`, defaults to 70.0.
    /// * `oversold` - Optional oversold threshold. If `None`, defaults to 30.0.
    pub fn new(period: usize, overbought: Option<f64>, oversold: Option<f64>) -> Self {
        RSI {
            period,
            gains: VecDeque::with_capacity(period),
            losses: VecDeque::with_capacity(period),
            sum_gains: 0.0,
            sum_losses: 0.0,
            prev_price: None,
            overbought: overbought.unwrap_or(70.0),
            oversold: oversold.unwrap_or(30.0),
        }
    }

    /// Updates the RSI calculation with a new price and returns the current RSI result if available.
    ///
    /// This method processes the new `price`, updates the internal sliding window of gains
    /// and losses, and calculates the RSI once enough data has been gathered.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price data.
    ///
    /// # Returns
    ///
    /// An `Option<RSIResult>` containing:
    /// - `value`: The calculated RSI.
    /// - `condition`: The market condition determined from the RSI value.
    ///
    /// Returns `None` if insufficient data has been provided.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::rsi::main::RSI;
    /// use indexes_rs::v1::sma::main::SMAError;
    /// use indexes_rs::v1::rsi::types::RSIResult;
    /// use indexes_rs::v1::types::TrendDirection;
    ///
    /// let mut rsi = RSI::new(14, None, None);
    /// let price = 44.33;
    /// if let Some(result) = rsi.calculate(price) {
    ///     println!("RSI: {:.2}", result.value);
    /// }
    /// ```
    pub fn calculate(&mut self, price: f64) -> Option<RSIResult> {
        // If a previous price exists, compute the change and update gains/losses.
        if let Some(prev) = self.prev_price {
            let change = price - prev;
            let (gain, loss) = if change >= 0.0 { (change, 0.0) } else { (0.0, change.abs()) };

            self.gains.push_back(gain);
            self.losses.push_back(loss);
            self.sum_gains += gain;
            self.sum_losses += loss;

            // Maintain the sliding window size.
            if self.gains.len() > self.period {
                if let Some(old_gain) = self.gains.pop_front() {
                    self.sum_gains -= old_gain;
                }
                if let Some(old_loss) = self.losses.pop_front() {
                    self.sum_losses -= old_loss;
                }
            }
        }

        self.prev_price = Some(price);

        // Return None if not enough data is available.
        if self.gains.len() < self.period {
            return None;
        }

        let avg_gain = self.sum_gains / self.period as f64;
        let avg_loss = self.sum_losses / self.period as f64;

        // Calculate RS and then RSI.
        let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
        let rsi = 100.0 - (100.0 / (1.0 + rs));

        Some(RSIResult {
            value: rsi,
            condition: self.determine_condition(rsi),
        })
    }

    /// Determines the market condition based on the given RSI value and the configured thresholds.
    ///
    /// - Returns `Overbought` if RSI is greater than or equal to the overbought threshold.
    /// - Returns `Oversold` if RSI is less than or equal to the oversold threshold.
    /// - Returns `Neutral` otherwise.
    ///
    /// # Arguments
    ///
    /// * `rsi` - The RSI value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::rsi::main::RSI;
    /// use indexes_rs::v1::rsi::types::MarketCondition;
    /// let rsi = RSI::new(14, Some(80.0), Some(20.0));
    /// assert_eq!(rsi.determine_condition(85.0), MarketCondition::Overbought);
    /// ```
    pub fn determine_condition(&self, rsi: f64) -> MarketCondition {
        if rsi >= self.overbought {
            MarketCondition::Overbought
        } else if rsi <= self.oversold {
            MarketCondition::Oversold
        } else {
            MarketCondition::Neutral
        }
    }
}
