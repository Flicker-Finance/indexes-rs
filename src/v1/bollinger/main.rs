//! # Bollinger Bands Module
//!
//! This module implements Bollinger Bands using a Simple Moving Average (SMA) for the middle band
//! and the standard deviation of prices for the band width. The upper band is defined as the middle
//! band plus a specified multiplier times the standard deviation, and the lower band is defined as the
//! middle band minus that value.
//!
//! # Examples
//!
//! ```rust
//! use indexes_rs::v1::bollinger::main::BollingerBands;
//! use indexes_rs::v1::bollinger::types::BBResult;
//!
//! // Create a BollingerBands indicator with a period of 20 and multiplier of 2.0
//! let mut bb = BollingerBands::new(20, 2.0).unwrap();
//!
//! // Feed in prices (e.g., closing prices)
//! let prices = vec![
//!     100.0, 101.0, 102.0, 101.5, 100.5, 102.0, 103.0, 102.5, 104.0, 105.0,
//!     104.5, 105.5, 106.0, 107.0, 106.5, 108.0, 107.5, 108.5, 109.0, 110.0,
//! ];
//!
//! if let Some(result) = prices.into_iter().fold(None, |_, price| bb.calculate(price)) {
//!     println!("Upper Band: {:.2}", result.upper);
//!     println!("Middle Band: {:.2}", result.middle);
//!     println!("Lower Band: {:.2}", result.lower);
//! }
//! ```

use super::types::BBResult;
use crate::v1::sma::main::{SMAError, SimpleMovingAverage};
use std::collections::VecDeque;

/// Bollinger Bands indicator.
pub struct BollingerBands {
    sma: SimpleMovingAverage,
    period: usize,
    multiplier: f64,
    values: VecDeque<f64>,
}

impl BollingerBands {
    /// Creates a new BollingerBands indicator.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of values for the moving average and standard deviation.
    /// * `multiplier` - The multiplier applied to the standard deviation to determine band width.
    ///
    /// # Returns
    ///
    /// * `Ok(BollingerBands)` on success, or `Err(SMAError)` if the underlying SMA creation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::bollinger::main::BollingerBands;
    ///
    /// let bb = BollingerBands::new(20, 2.0);
    /// assert!(bb.is_ok());
    /// ```
    pub fn new(period: usize, multiplier: f64) -> Result<Self, SMAError> {
        Ok(BollingerBands {
            sma: SimpleMovingAverage::new(period)?,
            period,
            multiplier,
            values: VecDeque::with_capacity(period),
        })
    }

    /// Calculates the Bollinger Bands for the given price.
    ///
    /// The method updates the internal SMA and sliding window of prices.
    /// It returns a `BBResult` containing the upper, middle, and lower bands when enough data is available.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price.
    ///
    /// # Returns
    ///
    /// * `Some(BBResult)` if enough data is available.
    /// * `None` if not enough data has been collected.
    pub fn calculate(&mut self, price: f64) -> Option<BBResult> {
        self.sma.add_value(price);
        self.values.push_back(price);
        if self.values.len() > self.period {
            self.values.pop_front();
        }
        let middle = self.sma.calculate()?;
        let std_dev = self.calculate_std_dev(middle.value)?;
        let band_width = std_dev * self.multiplier;
        Some(BBResult {
            upper: middle.value + band_width,
            middle: middle.value,
            lower: middle.value - band_width,
        })
    }

    /// Calculates the standard deviation of the prices in the current window.
    ///
    /// # Arguments
    ///
    /// * `mean` - The mean (middle band) value.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` containing the standard deviation if the window is full.
    /// * `None` if the window does not yet contain enough values.
    fn calculate_std_dev(&self, mean: f64) -> Option<f64> {
        if self.values.len() < self.period {
            return None;
        }
        let variance = self
            .values
            .iter()
            .map(|&value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<f64>()
            / self.period as f64;
        Some(variance.sqrt())
    }
}
