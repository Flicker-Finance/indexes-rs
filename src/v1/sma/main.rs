//! A Simple Moving Average (SMA) calculator module.
//!
//! The Simple Moving Average is calculated by taking the arithmetic mean of a given set of values
//! over a specified period. For example, a 20-day SMA is calculated by taking the arithmetic mean
//! of the most recent 20 values.
//!
//! This implementation uses a sliding window backed by a [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
//! and maintains a running sum for improved performance. In addition to calculating the SMA value,
//! it also determines the trend (Up, Down, or Neutral) based on the change from the previous SMA.
//!
//! # Examples
//!
//! Using the SMA with a period of 3:
//!
//! ```rust
//! use indexes_rs::v1::sma::main::{SimpleMovingAverage, SMAError, SMAResult};
//! use indexes_rs::v1::types::TrendDirection;
//!
//! let mut sma = SimpleMovingAverage::new(3).unwrap();
//! sma.add_value(2.0);
//! sma.add_value(4.0);
//! sma.add_value(6.0);
//!
//! // First calculation returns Sideways trend since no previous value exists.
//! let result = sma.calculate().unwrap();
//! assert_eq!(result.value, 4.0);
//! assert_eq!(result.trend, TrendDirection::Sideways);
//! ```

pub use super::types::{SMAError, SMAResult};
use crate::v1::types::TrendDirection;
use std::collections::VecDeque;

/// A Simple Moving Average (SMA) calculator that maintains a moving window of values
/// and calculates their average along with a trend indicator.
///
/// The SMA is calculated by taking the arithmetic mean of a given set of values
/// over a specified period. The trend is determined by comparing the current SMA value
/// with the previous SMA value:
/// - If the current SMA is greater, the trend is `Up`.
/// - If it is lower, the trend is `Down`.
/// - If it is the same (or if no previous value exists), the trend is `Sideways`.
#[derive(Debug, PartialEq)]
pub struct SimpleMovingAverage {
    /// The period over which the moving average is calculated.
    pub period: usize,
    /// The collection of values in the moving window.
    values: VecDeque<f64>,
    /// The running sum of the values in the window.
    sum: f64,
    /// The previous calculated SMA value.
    last_value: Option<f64>,
}

impl SimpleMovingAverage {
    /// Creates a new `SimpleMovingAverage` instance with the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of values to include in the moving average calculation.
    ///
    /// # Returns
    ///
    /// * `Ok(SimpleMovingAverage)` - A new instance with the specified period.
    /// * `Err(SMAError)` - If the period is invalid (e.g., zero).
    ///
    /// # Examples
    ///
    /// Using a valid period:
    ///
    /// ```rust
    /// use indexes_rs::v1::sma::main::{SimpleMovingAverage, SMAError};
    ///
    /// let sma = SimpleMovingAverage::new(3);
    /// assert!(sma.is_ok());
    /// ```
    ///
    /// Using an invalid period:
    ///
    /// ```rust
    /// use indexes_rs::v1::sma::main::{SimpleMovingAverage, SMAError};
    ///
    /// let sma = SimpleMovingAverage::new(0);
    /// assert_eq!(sma, Err(SMAError::InvalidPeriod));
    /// ```
    pub fn new(period: usize) -> Result<Self, SMAError> {
        if period == 0 {
            return Err(SMAError::InvalidPeriod);
        }
        Ok(SimpleMovingAverage {
            period,
            values: VecDeque::with_capacity(period),
            sum: 0.0,
            last_value: None,
        })
    }

    /// Adds a new value to the moving window.
    ///
    /// If the window is full (i.e., the number of stored values equals the period),
    /// the oldest value is removed before adding the new one.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value to add to the moving window.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::sma::main::SimpleMovingAverage;
    ///
    /// let mut sma = SimpleMovingAverage::new(3).unwrap();
    /// sma.add_value(2.0);
    /// sma.add_value(4.0);
    /// sma.add_value(6.0);
    /// ```
    pub fn add_value(&mut self, value: f64) {
        if self.values.len() == self.period {
            if let Some(old_value) = self.values.pop_front() {
                self.sum -= old_value;
            }
        }
        self.values.push_back(value);
        self.sum += value;
    }

    /// Calculates the current Simple Moving Average and determines the trend.
    ///
    /// # Returns
    ///
    /// * `Some(SMAResult)` - The calculated SMA and trend if enough values are available.
    /// * `None` - If there aren't enough values to calculate the average.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::sma::main::{SimpleMovingAverage, SMAError, SMAResult};
    /// use indexes_rs::v1::types::TrendDirection;
    ///
    /// let mut sma = SimpleMovingAverage::new(3).unwrap();
    /// sma.add_value(2.0);
    /// sma.add_value(4.0);
    /// sma.add_value(6.0);
    ///
    /// let result = sma.calculate().unwrap();
    /// // The average of [2.0, 4.0, 6.0] is 4.0, and since no previous SMA exists, the trend is Sideways.
    /// assert_eq!(result.value, 4.0);
    /// assert_eq!(result.trend, TrendDirection::Sideways);
    /// ```
    pub fn calculate(&mut self) -> Option<SMAResult> {
        if self.values.len() < self.period {
            return None;
        }
        let current_sma = self.sum / self.period as f64;
        let trend = match self.last_value {
            Some(prev) if current_sma > prev => TrendDirection::Up,
            Some(prev) if current_sma < prev => TrendDirection::Down,
            _ => TrendDirection::Sideways,
        };
        self.last_value = Some(current_sma);
        Some(SMAResult { value: current_sma, trend })
    }
}
