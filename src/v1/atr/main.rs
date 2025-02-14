//! # Average True Range (ATR) Module
//!
//! This module implements a simplified Average True Range (ATR) indicator.
//! In this version, the true range is calculated as the absolute difference between the
//! current closing price and the previous closing price. (Note that the full ATR calculation
//! typically uses the high, low, and previous close values, but this simplified version
//! uses only the close values.)
//!
//! The ATR is computed as the average of the true range over a specified period.
//!
//! # Examples
//!
//! ```rust
//! use indexes_rs::v1::atr::main::ATR;
//!
//! // Create an ATR indicator with a period of 14
//! let mut atr = ATR::new(14);
//!
//! // Feed a series of closing prices
//! let prices = vec![100.0, 101.0, 100.5, 102.0, 101.5, 102.5, 103.0, 102.0, 101.0, 100.0,
//!                   100.5, 101.0, 102.0, 101.5, 102.5];
//!
//! for price in prices {
//!     if let Some(atr_value) = atr.calculate(price) {
//!         println!("ATR: {:.2}", atr_value);
//!     }
//! }
//! ```

use std::collections::VecDeque;

/// A simplified Average True Range (ATR) indicator.
///
/// This ATR calculates the true range as the absolute difference between the current closing price and the previous closing price.
/// It then computes the ATR as the average of these true ranges over a specified period.
pub struct ATR {
    /// The period over which to calculate the ATR.
    period: usize,
    /// A sliding window of true range values.
    values: VecDeque<f64>,
    /// The previous closing price.
    prev_close: Option<f64>,
}

impl ATR {
    /// Creates a new ATR indicator with the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods over which to calculate the ATR.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indexes_rs::v1::atr::main::ATR;
    ///
    /// let atr = ATR::new(14);
    /// ```
    pub fn new(period: usize) -> Self {
        ATR {
            period,
            values: VecDeque::with_capacity(period),
            prev_close: None,
        }
    }

    /// Calculates the current ATR value using the latest closing price.
    ///
    /// The true range is computed as the absolute difference between the current closing price and the previous closing price.
    /// This true range is stored in a sliding window; once the window contains `period` values, the ATR is returned as their average.
    ///
    /// # Arguments
    ///
    /// * `close` - The latest closing price.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` containing the ATR value if enough data is available.
    /// * `None` if there aren't enough values yet.
    pub fn calculate(&mut self, close: f64) -> Option<f64> {
        // Compute the true range based on the previous close, or 0.0 if none exists.
        let true_range = self.prev_close.map_or(0.0, |prev| (close - prev).abs());

        self.values.push_back(true_range);
        if self.values.len() > self.period {
            self.values.pop_front();
        }

        self.prev_close = Some(close);

        if self.values.len() == self.period {
            Some(self.values.iter().sum::<f64>() / self.period as f64)
        } else {
            None
        }
    }
}
