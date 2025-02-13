//! # Moving Averages Module
//!
//! This module provides a unified interface for calculating multiple moving averages
//! along with a MACD indicator. It includes:
//!
//! - **SMA (Simple Moving Average):** Calculated for short, medium, and long periods.
//! - **EMA (Exponential Moving Average):** Calculated for short, medium, and long periods.
//! - **MACD (Moving Average Convergence Divergence):**
//!
//! ## Example
//!
//! ```rust
//! use indexes_rs::v1::ma::main::{MovingAverages, MovingAverageResults};
//!
//! // Create a new moving averages calculator with default parameters.
//! let mut ma = MovingAverages::default();
//!
//! // Or supply custom parameters:
//! // SMA: 10, 30, 100; EMA: 10, 30, 100; MACD: fast=8, slow=17, signal=9
//! // let mut ma = MovingAverages::with_params(
//! //     Some(10), Some(30), Some(100),
//! //     Some(10), Some(30), Some(100),
//! //     Some(8), Some(17), Some(9)
//! // ).unwrap();
//!
//! // Simulate a stream of prices.
//! let prices = vec![10.0, 10.5, 11.0, 10.8, 11.2, 11.5, 11.3];
//!
//! for price in prices {
//!     let results = ma.calculate(price);
//!     println!("SMA: {:?}, EMA: {:?}, MACD: {:?}", results.sma, results.ema, results.macd);
//! }
//! ```

use serde::Serialize;

use crate::v1::{
    ema::main::ExponentialMovingAverage,
    macd::{main::MACD, types::MACDResult},
    sma::main::{SMAError, SMAResult, SimpleMovingAverage},
};

/// A container for the different moving average indicators.
pub struct MovingAverages {
    pub sma: SMAPeriods,
    pub ema: EMAPeriods,
    pub macd: MACD,
}

/// Holds SMA indicators for different periods.
pub struct SMAPeriods {
    /// Short period SMA.
    pub short: SimpleMovingAverage,
    /// Medium period SMA.
    pub medium: SimpleMovingAverage,
    /// Long period SMA.
    pub long: SimpleMovingAverage,
}

/// Holds EMA indicators for different periods.
pub struct EMAPeriods {
    /// Short period EMA.
    pub short: ExponentialMovingAverage,
    /// Medium period EMA.
    pub medium: ExponentialMovingAverage,
    /// Long period EMA.
    pub long: ExponentialMovingAverage,
}

/// The consolidated results for moving averages.
#[derive(Debug)]
pub struct MovingAverageResults {
    /// The Simple Moving Average values.
    pub sma: SMAValues,
    /// The Exponential Moving Average values.
    pub ema: EMAValues,
    /// The MACD result (if available).
    pub macd: Option<MACDResult>,
}

/// SMA values for different periods.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SMAValues {
    /// Short period SMA value.
    pub short: Option<SMAResult>,
    /// Medium period SMA value.
    pub medium: Option<SMAResult>,
    /// Long period SMA value.
    pub long: Option<SMAResult>,
}

/// EMA values for different periods.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct EMAValues {
    /// Short period EMA value.
    pub short: Option<f64>,
    /// Medium period EMA value.
    pub medium: Option<f64>,
    /// Long period EMA value.
    pub long: Option<f64>,
}

impl Default for MovingAverages {
    /// Creates a new `MovingAverages` instance using default parameters.
    ///
    /// - SMA periods: short = 20, medium = 50, long = 200
    /// - EMA periods: short = 20, medium = 50, long = 200
    /// - MACD parameters: fast = 12, slow = 26, signal = 9
    fn default() -> Self {
        Self::with_params(None, None, None, None, None, None, None, None, None).expect("Default parameters should always be valid")
    }
}

impl MovingAverages {
    /// Creates a new `MovingAverages` instance with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `sma_short` - Optional SMA short period (default: 20).
    /// * `sma_medium` - Optional SMA medium period (default: 50).
    /// * `sma_long` - Optional SMA long period (default: 200).
    /// * `ema_short` - Optional EMA short period (default: 20).
    /// * `ema_medium` - Optional EMA medium period (default: 50).
    /// * `ema_long` - Optional EMA long period (default: 200).
    /// * `macd_fast` - Optional MACD fast period (default: 12).
    /// * `macd_slow` - Optional MACD slow period (default: 26).
    /// * `macd_signal` - Optional MACD signal period (default: 9).
    pub fn with_params(
        sma_short: Option<usize>,
        sma_medium: Option<usize>,
        sma_long: Option<usize>,
        ema_short: Option<usize>,
        ema_medium: Option<usize>,
        ema_long: Option<usize>,
        macd_fast: Option<usize>,
        macd_slow: Option<usize>,
        macd_signal: Option<usize>,
    ) -> Result<Self, SMAError> {
        Ok(MovingAverages {
            sma: SMAPeriods::new(sma_short, sma_medium, sma_long)?,
            ema: EMAPeriods::new_with_params(ema_short, ema_medium, ema_long),
            macd: MACD::new(macd_fast.unwrap_or(12), macd_slow.unwrap_or(26), macd_signal.unwrap_or(9)),
        })
    }

    /// Updates all moving averages with the new price and returns the consolidated results.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price value.
    pub fn calculate(&mut self, price: f64) -> MovingAverageResults {
        self.sma.update(price);
        self.ema.update(price);

        MovingAverageResults {
            sma: self.sma.get_values(),
            ema: self.ema.get_values(),
            macd: self.macd.calculate(price),
        }
    }
}

impl SMAPeriods {
    /// Creates a new set of SMA indicators with the following periods:
    /// - Short: defaults to 20 if not provided.
    /// - Medium: defaults to 50 if not provided.
    /// - Long: defaults to 200 if not provided.
    pub fn new(short: Option<usize>, medium: Option<usize>, long: Option<usize>) -> Result<Self, SMAError> {
        Ok(SMAPeriods {
            short: SimpleMovingAverage::new(short.unwrap_or(20))?,
            medium: SimpleMovingAverage::new(medium.unwrap_or(50))?,
            long: SimpleMovingAverage::new(long.unwrap_or(200))?,
        })
    }

    /// Updates each SMA indicator with the new price.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price value.
    pub fn update(&mut self, price: f64) {
        self.short.add_value(price);
        self.medium.add_value(price);
        self.long.add_value(price);
    }

    /// Retrieves the current SMA values.
    pub fn get_values(&mut self) -> SMAValues {
        SMAValues {
            short: self.short.calculate(),
            medium: self.medium.calculate(),
            long: self.long.calculate(),
        }
    }
}

impl EMAPeriods {
    /// Creates a new set of EMA indicators with the following periods:
    /// - Short: defaults to 20 if not provided.
    /// - Medium: defaults to 50 if not provided.
    /// - Long: defaults to 200 if not provided.
    pub fn new_with_params(short: Option<usize>, medium: Option<usize>, long: Option<usize>) -> Self {
        EMAPeriods {
            short: ExponentialMovingAverage::new(short.unwrap_or(20)),
            medium: ExponentialMovingAverage::new(medium.unwrap_or(50)),
            long: ExponentialMovingAverage::new(long.unwrap_or(200)),
        }
    }

    /// Updates each EMA indicator with the new price.
    ///
    /// # Arguments
    ///
    /// * `price` - The latest price value.
    pub fn update(&mut self, price: f64) {
        self.short.add_value(price);
        self.medium.add_value(price);
        self.long.add_value(price);
    }

    /// Retrieves the current EMA values.
    pub fn get_values(&self) -> EMAValues {
        EMAValues {
            short: self.short.get_current_value(),
            medium: self.medium.get_current_value(),
            long: self.long.get_current_value(),
        }
    }
}
