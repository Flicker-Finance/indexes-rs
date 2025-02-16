//! # MACD (Moving Average Convergence Divergence) Module
//!
//! This module implements the MACD indicator using three exponential moving averages:
//! - A fast EMA (short period)
//! - A slow EMA (long period)
//! - A signal EMA (for smoothing the MACD line)
//!
//! The MACD line is calculated as the difference between the fast and slow EMAs.
//! The signal line is calculated as the EMA of the MACD line, and the histogram is the
//! difference between the MACD line and the signal line. Additionally, a trading signal is
//! generated based on the relationship between the MACD line and the signal line.
//!
//! # Examples
//!
//! ```rust
//! use indexes_rs::v1::macd::main::MACD;
//! use indexes_rs::v1::macd::types::MACDResult;
//! use indexes_rs::v1::types::TradingSignal;
//!
//! // Create a MACD indicator with fast=12, slow=26, and signal=9 periods
//! let mut macd = MACD::new(12, 26, 9);
//!
//! // Simulate feeding prices
//! let prices = vec![44.0, 44.5, 45.0, 44.8, 45.2, 45.5, 45.3];
//! let mut result = None;
//! for price in prices {
//!     result = macd.calculate(price);
//! }
//!
//! if let Some(res) = result {
//!     println!("MACD Line: {:.2}", res.macd_line);
//!     println!("Signal Line: {:.2}", res.signal_line);
//!     println!("Histogram: {:.2}", res.histogram);
//!     println!("Trading Signal: {:?}", res.signal);
//! }
//! ```

use super::types::*;
use crate::v1::{ema::main::ExponentialMovingAverage, types::TradingSignal};

/// MACD (Moving Average Convergence Divergence) indicator.
pub struct MACD {
    pub fast_ema: ExponentialMovingAverage,
    pub slow_ema: ExponentialMovingAverage,
    pub signal_ema: ExponentialMovingAverage,
    pub histogram: Vec<f64>,
}

impl MACD {
    /// Creates a new MACD indicator.
    ///
    /// # Arguments
    ///
    /// * `fast_period` - The period for the fast EMA.
    /// * `slow_period` - The period for the slow EMA.
    /// * `signal_period` - The period for the signal EMA.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indexes_rs::v1::macd::main::MACD;
    ///
    /// let macd = MACD::new(12, 26, 9);
    /// ```
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        MACD {
            fast_ema: ExponentialMovingAverage::new(fast_period),
            slow_ema: ExponentialMovingAverage::new(slow_period),
            signal_ema: ExponentialMovingAverage::new(signal_period),
            histogram: Vec::new(),
        }
    }

    /// Updates the MACD calculation with a new price and returns the current MACD result.
    ///
    /// The method updates the fast and slow EMAs with the new price, calculates the MACD line
    /// as the difference between the fast and slow EMAs, and then updates the signal EMA using
    /// the MACD line. The histogram is computed as the difference between the MACD line and the signal line.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price.
    ///
    /// # Returns
    ///
    /// * `Some(MACDResult)` containing the MACD line, signal line, histogram, and trading signal,
    ///    if the EMAs have been sufficiently initialized.
    /// * `None` if any of the EMA calculations are not yet available.
    pub fn calculate(&mut self, price: f64) -> Option<MACDResult> {
        let fast = self.fast_ema.add_value(price)?;
        let slow = self.slow_ema.add_value(price)?;
        let macd_line = fast - slow;
        let signal_line = self.signal_ema.add_value(macd_line)?;
        let histogram = macd_line - signal_line;

        self.histogram.push(histogram);

        Some(MACDResult {
            macd_line,
            signal_line,
            histogram,
            signal: self.determine_signal(macd_line, signal_line),
        })
    }

    /// Determines the trading signal based on the MACD line and the signal line.
    ///
    /// If the MACD line is above the signal line, returns `Buy`.
    /// If the MACD line is below the signal line, returns `Sell`.
    /// Otherwise, returns `Hold`.
    ///
    /// # Arguments
    ///
    /// * `macd` - The current MACD line value.
    /// * `signal` - The current signal line value.
    ///
    /// # Returns
    ///
    /// A `TradingSignal` representing the trading recommendation.
    pub fn determine_signal(&self, macd: f64, signal: f64) -> TradingSignal {
        if macd > signal {
            TradingSignal::Buy
        } else if macd < signal {
            TradingSignal::Sell
        } else {
            TradingSignal::Hold
        }
    }
}
