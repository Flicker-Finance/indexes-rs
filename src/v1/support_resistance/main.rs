//! # Support & Resistance Module
//!
//! This module implements a support/resistance indicator based on swing highs and lows.
//! It keeps a sliding window of prices (with a customizable period) and calculates support
//! and resistance levels using a specified threshold (default is 2%).
//!
//! The module provides a `calculate` method that returns an `SRResult` containing:
//! - The nearest support level
//! - The nearest resistance level
//! - A support strength (0-100%)
//! - A resistance strength (0-100%)
//! - A breakout potential (based on the weaker of the two strengths)
//! - A price position (relative to the support/resistance levels)
//!
//! # Example
//!
//! ```rust
//! use indexes_rs::v1::support_resistance::main::SupportResistance;
//! use indexes_rs::v1::support_resistance::types::{SRResult, PricePosition};
//!
//! let mut sr = SupportResistance::new(SupportResistance::DEFAULT_PERIOD, SupportResistance::DEFAULT_THRESHOLD);
//!
//! // Simulate adding prices (in a real scenario, these would be updated on every tick)
//! let prices = vec![100.0, 101.0, 102.0, 101.5, 100.5, 99.0, 98.5, 99.5, 100.0, 101.0, 102.0, 103.0];
//! let mut result: Option<SRResult> = None;
//! for price in prices {
//!     result = sr.calculate(price);
//! }
//!
//! if let Some(res) = result {
//!     println!("Nearest support: {:?}", res.nearest_support);
//!     println!("Nearest resistance: {:?}", res.nearest_resistance);
//!     println!("Support strength: {:.2}%", res.support_strength);
//!     println!("Resistance strength: {:.2}%", res.resistance_strength);
//!     println!("Breakout potential: {:.2}%", res.breakout_potential);
//!     println!("Price position: {:?}", res.price_position);
//! }
//! ```

use super::types::*; // This module should define SRResult and PricePosition

/// A Support/Resistance indicator based on a sliding window of prices and swing detection.
pub struct SupportResistance {
    period: usize,
    prices: Vec<f64>,
    swing_high_threshold: f64,
    swing_low_threshold: f64,
    support_levels: Vec<f64>,
    resistance_levels: Vec<f64>,
}

impl SupportResistance {
    /// Default period (number of prices to consider) for swing detection.
    pub const DEFAULT_PERIOD: usize = 20;
    /// Default threshold (2%): used to validate or clean up support/resistance levels.
    pub const DEFAULT_THRESHOLD: f64 = 0.02;

    /// Creates a new `SupportResistance` indicator.
    ///
    /// # Arguments
    ///
    /// * `period` - The number of prices to include in the sliding window.
    /// * `threshold` - The percentage threshold (as a decimal, e.g. 0.02 for 2%)
    ///                 to determine swing levels.
    pub fn new(period: usize, threshold: f64) -> Self {
        SupportResistance {
            period,
            prices: Vec::new(),
            swing_high_threshold: 1.0 + threshold,
            swing_low_threshold: 1.0 - threshold,
            support_levels: Vec::new(),
            resistance_levels: Vec::new(),
        }
    }

    /// Updates the indicator with a new price and returns the current support/resistance result.
    ///
    /// This method pushes the new price into the internal price window, updates the
    /// detected support and resistance levels, and returns an `SRResult` if there are
    /// enough prices.
    ///
    /// # Arguments
    ///
    /// * `price` - The new price to add.
    ///
    /// # Returns
    ///
    /// * `Some(SRResult)` if there are enough prices for calculation.
    /// * `None` if there aren't enough prices yet.
    pub fn calculate(&mut self, price: f64) -> Option<SRResult> {
        self.prices.push(price);
        // Keep the sliding window limited to at most period*2 values.
        if self.prices.len() > self.period * 2 {
            self.prices.remove(0);
        }

        // Ensure we have enough data.
        if self.prices.len() < self.period {
            return None;
        }

        self.update_levels();

        Some(SRResult {
            nearest_support: self.find_nearest_support(price),
            nearest_resistance: self.find_nearest_resistance(price),
            support_strength: self.calculate_support_strength(price),
            resistance_strength: self.calculate_resistance_strength(price),
            breakout_potential: self.calculate_breakout_potential(price),
            price_position: self.determine_price_position(price),
        })
    }

    /// Updates support and resistance levels based on the latest price window.
    ///
    /// This method checks if the current window shows a swing high or swing low,
    /// updates the respective levels, and cleans out old or invalidated levels.
    fn update_levels(&mut self) {
        if let Some(window) = self.prices.get(self.prices.len().saturating_sub(self.period)..) {
            let mid_index = window.len() / 2;
            if self.is_swing_high(window) {
                // Record the swing high (the mid value in the window).
                let swing_high = window[mid_index];
                self.resistance_levels.push(swing_high);
            }
            if self.is_swing_low(window) {
                // Record the swing low (the mid value in the window).
                let swing_low = window[mid_index];
                self.support_levels.push(swing_low);
            }
        }
        // Use the latest price for cleaning.
        let current_price = *self.prices.last().unwrap_or(&0.0);
        self.clean_levels(current_price);
    }

    /// Determines if the given window is a swing high.
    ///
    /// A swing high is defined as the middle value being higher than all other values in the window.
    fn is_swing_high(&self, window: &[f64]) -> bool {
        if window.len() < 3 {
            return false;
        }
        let mid = window.len() / 2;
        let mid_price = window[mid];
        window[..mid].iter().all(|&p| p < mid_price) && window[mid + 1..].iter().all(|&p| p < mid_price)
    }

    /// Determines if the given window is a swing low.
    ///
    /// A swing low is defined as the middle value being lower than all other values in the window.
    fn is_swing_low(&self, window: &[f64]) -> bool {
        if window.len() < 3 {
            return false;
        }
        let mid = window.len() / 2;
        let mid_price = window[mid];
        window[..mid].iter().all(|&p| p > mid_price) && window[mid + 1..].iter().all(|&p| p > mid_price)
    }

    /// Cleans out old or invalidated support and resistance levels.
    ///
    /// Levels that are too far from the current price (based on the swing thresholds)
    /// are removed.
    fn clean_levels(&mut self, current_price: f64) {
        self.support_levels.retain(|&level| level < current_price * self.swing_high_threshold);
        self.resistance_levels.retain(|&level| level > current_price * self.swing_low_threshold);
    }

    /// Finds the nearest support level below the given price.
    fn find_nearest_support(&self, price: f64) -> Option<f64> {
        self.support_levels.iter().filter(|&&s| s < price).max_by(|a, b| a.partial_cmp(b).unwrap()).copied()
    }

    /// Finds the nearest resistance level above the given price.
    fn find_nearest_resistance(&self, price: f64) -> Option<f64> {
        self.resistance_levels.iter().filter(|&&r| r > price).min_by(|a, b| a.partial_cmp(b).unwrap()).copied()
    }

    /// Calculates the strength of the support level as a percentage (0-100).
    ///
    /// Strength is determined by the relative distance between the current price and the support.
    fn calculate_support_strength(&self, price: f64) -> f64 {
        if let Some(support) = self.find_nearest_support(price) {
            let distance = (price - support).abs() / price;
            (1.0 - distance).clamp(0.0, 1.0) * 100.0
        } else {
            0.0
        }
    }

    /// Calculates the strength of the resistance level as a percentage (0-100).
    ///
    /// Strength is determined by the relative distance between the resistance and the current price.
    fn calculate_resistance_strength(&self, price: f64) -> f64 {
        if let Some(resistance) = self.find_nearest_resistance(price) {
            let distance = (resistance - price).abs() / price;
            (1.0 - distance).clamp(0.0, 1.0) * 100.0
        } else {
            0.0
        }
    }

    /// Calculates the breakout potential based on the weaker of the support or resistance strengths.
    fn calculate_breakout_potential(&self, price: f64) -> f64 {
        let support_strength = self.calculate_support_strength(price);
        let resistance_strength = self.calculate_resistance_strength(price);
        if support_strength > resistance_strength {
            resistance_strength
        } else {
            support_strength
        }
    }

    /// Determines the price position relative to the nearest support and resistance levels.
    ///
    /// Returns a `PricePosition` value indicating whether the price is in the middle, near support,
    /// near resistance, or outside the known levels.
    fn determine_price_position(&self, price: f64) -> PricePosition {
        match (self.find_nearest_support(price), self.find_nearest_resistance(price)) {
            (Some(s), Some(r)) => {
                let mid_point = (s + r) / 2.0;
                if (price - mid_point).abs() < (r - s) * 0.1 {
                    PricePosition::Middle
                } else if price > mid_point {
                    PricePosition::NearResistance
                } else {
                    PricePosition::NearSupport
                }
            }
            (Some(_), None) => PricePosition::AboveResistance,
            (None, Some(_)) => PricePosition::BelowSupport,
            (None, None) => PricePosition::Unknown,
        }
    }
}
