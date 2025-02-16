//! # Momentum Indicator Module
//!
//! This module implements a Momentum indicator.
//! The Momentum indicator calculates the difference between the current price and the price
//! from a specified number of periods ago. In addition, it computes a momentum ratio, defined as
//! the current price as a percentage of the past price.
//!
//! # Examples
//!
//! ```rust
//! use indexes_rs::v1::momentum::main::Momentum;
//! use indexes_rs::v1::momentum::types::MomentumResult;
//!
//! let mut momentum = Momentum::new(14);
//! let prices = vec![100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0, 112.0, 113.0];
//!
//! // After feeding in at least 14 prices, calculate() returns a result.
//! if let Some(result) = momentum.calculate(113.0) {
//!     println!("Momentum: {:.2}", result.value);
//!     println!("Momentum Ratio: {:.2}%", result.ratio);
//! }
//! ```

use super::types::MomentumResult;
use std::collections::VecDeque;

/// A Momentum indicator that calculates the change between the current price and the price
/// from a given number of periods ago.
pub struct Momentum {
    /// The period over which to calculate momentum.
    period: usize,
    /// A sliding window of recent prices.
    values: VecDeque<f64>,
}

impl Momentum {
    /// The default period for the Momentum indicator.
    pub const DEFAULT_PERIOD: usize = 14;

    /// Creates a new `Momentum` indicator with the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods over which to calculate momentum.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indexes_rs::v1::momentum::main::Momentum;
    ///
    /// let momentum = Momentum::new(14);
    /// ```
    pub fn new(period: usize) -> Self {
        Momentum {
            period,
            values: VecDeque::with_capacity(period),
        }
    }

    /// Calculates the current momentum and momentum ratio.
    ///
    /// The momentum is computed as the difference between the current price and the price from `period` periods ago.
    /// The momentum ratio is computed as `(current_price / past_price) * 100.0`.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price.
    ///
    /// # Returns
    ///
    /// * `Some(MomentumResult)` if there are enough data points and the past price is not zero.
    /// * `None` if there aren't enough values or if the past price is zero (to avoid division by zero).
    ///
    /// # Example
    ///
    /// ```rust
    /// use indexes_rs::v1::momentum::main::Momentum;
    /// use indexes_rs::v1::momentum::types::MomentumResult;
    ///
    /// let mut momentum = Momentum::new(3);
    /// momentum.calculate(100.0); // first value, not enough data
    /// momentum.calculate(102.0);
    /// if let Some(result) = momentum.calculate(104.0) {
    ///     // Past price is 100.0, so momentum = 104.0 - 100.0 = 4.0
    ///     // and momentum ratio = (104.0 / 100.0) * 100 = 104.0%
    ///     assert_eq!(result.value, 4.0);
    ///     assert_eq!(result.ratio, 104.0);
    /// }
    /// ```
    pub fn calculate(&mut self, price: f64) -> Option<MomentumResult> {
        self.values.push_back(price);
        if self.values.len() > self.period {
            self.values.pop_front();
        }
        if self.values.len() < self.period {
            return None;
        }
        let past_price = *self.values.front()?;
        if past_price == 0.0 {
            return None; // Avoid division by zero.
        }
        let momentum = price - past_price;
        let momentum_ratio = (price / past_price) * 100.0;
        Some(MomentumResult {
            value: momentum,
            ratio: momentum_ratio,
        })
    }
}
