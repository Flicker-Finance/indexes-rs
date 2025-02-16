//! # Rate of Change (ROC) Module
//!
//! This module implements a Rate of Change (ROC) indicator.
//! The ROC is calculated as the percentage change between the current price and the price
//! from a specified number of periods ago. Additionally, it calculates a normalized momentum,
//! an acceleration (change in ROC from the previous value), and generates a trading signal.
//!
//! Typical usage example:
//!
//! ```rust
//! use indexes_rs::v1::roc::main::ROC;
//! use indexes_rs::v1::types::TradingSignal;
//!
//! let mut roc = ROC::new(12);
//! let prices = vec![100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0, 112.0];
//! let mut result = None;
//! for price in prices {
//!     result = roc.calculate(price);
//! }
//! if let Some(res) = result {
//!     println!("ROC value: {:.2}%", res.value);
//!     println!("Momentum: {:.2}%", res.momentum);
//!     println!("Acceleration: {:?}", res.acceleration);
//! }
//! ```

use super::types::ROCResult;
use crate::v1::types::TradingSignal;
use std::collections::VecDeque;

/// A Rate of Change (ROC) indicator.
pub struct ROC {
    period: usize,
    values: VecDeque<f64>,
    prev_roc: Option<f64>,
}

impl ROC {
    /// The default period for ROC calculation.
    pub const DEFAULT_PERIOD: usize = 12;
    /// The threshold used to generate trading signals.
    pub const SIGNAL_THRESHOLD: f64 = 2.0;

    /// Creates a new ROC indicator with the given period.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods over which to calculate the ROC.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indexes_rs::v1::roc::main::ROC;
    ///
    /// let roc = ROC::new(12);
    /// ```
    pub fn new(period: usize) -> Self {
        ROC {
            period,
            values: VecDeque::with_capacity(period + 1),
            prev_roc: None,
        }
    }

    /// Calculates the current ROC value based on the latest price.
    ///
    /// This method updates the sliding window of prices and computes:
    /// - The ROC value (percentage change between the current price and the price from `period` periods ago).
    /// - A normalized momentum value.
    /// - The acceleration (difference between the current and previous ROC).
    /// - A trading signal based on the ROC.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price.
    ///
    /// # Returns
    ///
    /// * `Some(ROCResult)` if there is enough data; otherwise, `None`.
    pub fn calculate(&mut self, price: f64) -> Option<ROCResult> {
        // Update the sliding window.
        self.values.push_back(price);
        if self.values.len() > self.period + 1 {
            self.values.pop_front();
        }

        // Not enough data to calculate ROC.
        if self.values.len() <= self.period {
            return None;
        }

        let old_price = *self.values.front()?;
        // Avoid division by zero: if the old price is 0, return None.
        if old_price == 0.0 {
            return None;
        }

        let current_roc = ((price - old_price) / old_price) * 100.0;

        // Calculate acceleration if previous ROC exists.
        let acceleration = self.prev_roc.map(|prev| current_roc - prev);
        self.prev_roc = Some(current_roc);

        Some(ROCResult {
            value: current_roc,
            momentum: self.normalize_momentum(current_roc),
            acceleration,
            signal: self.get_signal(current_roc),
        })
    }

    /// Normalizes the ROC value to a momentum value between -100 and 100.
    ///
    /// This function assumes typical ROC values range approximately from -10 to +10.
    fn normalize_momentum(&self, roc: f64) -> f64 {
        // Normalize so that a value of 10 becomes 100%, and -10 becomes -100%.
        let normalized = (roc / 10.0) * 100.0;
        normalized.clamp(-100.0, 100.0)
    }

    /// Generates a trading signal based on the ROC value.
    ///
    /// If ROC > SIGNAL_THRESHOLD, returns `Buy`.
    /// If ROC < -SIGNAL_THRESHOLD, returns `Sell`.
    /// Otherwise, returns `Hold`.
    fn get_signal(&self, roc: f64) -> TradingSignal {
        if roc > Self::SIGNAL_THRESHOLD {
            TradingSignal::Buy
        } else if roc < -Self::SIGNAL_THRESHOLD {
            TradingSignal::Sell
        } else {
            TradingSignal::Hold
        }
    }
}
