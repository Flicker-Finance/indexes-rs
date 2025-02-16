//! # Exponential Moving Average (EMA) Module
//!
//! This module implements an Exponential Moving Average (EMA) indicator.
//! The EMA applies a smoothing factor to give more weight to recent prices.
//! The smoothing factor (alpha) is computed as:
//!
//! \[\alpha = \frac{2}{\text{period} + 1}\]
//!
//! On the first data point, the EMA is simply set to the price. On subsequent calls,
//! the EMA is updated using the formula:
//!
//! \[\text{EMA}_{\text{new}} = \text{price} \times \alpha + \text{EMA}_{\text{prev}} \times (1 - \alpha)\]
//!
//! # Examples
//!
//! ```rust
//! use indexes_rs::v1::ema::main::ExponentialMovingAverage;
//!
//! // Create an EMA indicator with a period of 10.
//! let mut ema = ExponentialMovingAverage::new(10);
//!
//! // The first value sets the EMA to the price itself.
//! assert_eq!(ema.add_value(100.0).unwrap(), 100.0);
//!
//! // Subsequent values update the EMA using the smoothing factor.
//! let second = ema.add_value(105.0).unwrap();
//! println!("Updated EMA: {:.2}", second);
//!
//! // Get the current EMA value.
//! assert_eq!(ema.get_current_value().unwrap(), second);
//! ```

/// An Exponential Moving Average (EMA) indicator.
pub struct ExponentialMovingAverage {
    /// The smoothing factor (alpha).
    pub alpha: f64,
    /// The current EMA value.
    pub current_ema: Option<f64>,
}

impl ExponentialMovingAverage {
    /// Creates a new `ExponentialMovingAverage` indicator with the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods for the EMA calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::ema::main::ExponentialMovingAverage;
    ///
    /// let ema = ExponentialMovingAverage::new(10);
    /// ```
    pub fn new(period: usize) -> Self {
        ExponentialMovingAverage {
            alpha: 2.0 / (period as f64 + 1.0),
            current_ema: None,
        }
    }

    /// Adds a new price value to update the EMA.
    ///
    /// If no previous EMA exists, the current price is used as the initial EMA.
    /// Otherwise, the EMA is updated using:
    ///
    /// \[\text{EMA}_{\text{new}} = \text{price} \times \alpha + \text{EMA}_{\text{prev}} \times (1 - \alpha)\]
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` containing the updated EMA value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::ema::main::ExponentialMovingAverage;
    ///
    /// let mut ema = ExponentialMovingAverage::new(10);
    /// let first = ema.add_value(100.0).unwrap();
    /// // first EMA is equal to 100.0
    /// assert_eq!(first, 100.0);
    /// let second = ema.add_value(105.0).unwrap();
    /// println!("Updated EMA: {:.2}", second);
    /// ```
    pub fn add_value(&mut self, price: f64) -> Option<f64> {
        self.current_ema = Some(match self.current_ema {
            Some(ema) => price * self.alpha + ema * (1.0 - self.alpha),
            None => price,
        });
        self.current_ema
    }

    /// Returns the current EMA value.
    ///
    /// # Returns
    ///
    /// * `Some(f64)` if the EMA has been computed.
    /// * `None` if no values have been added yet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use indexes_rs::v1::ema::main::ExponentialMovingAverage;
    ///
    /// let mut ema = ExponentialMovingAverage::new(10);
    /// ema.add_value(100.0);
    /// assert_eq!(ema.get_current_value().unwrap(), 100.0);
    /// ```
    pub fn get_current_value(&self) -> Option<f64> {
        self.current_ema
    }
}
