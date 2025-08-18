use super::types::{HighLow, SupportResistanceLevels};

/// A stateless struct to encapsulate the support and resistance calculation logic.
pub struct SupportResistance;

impl SupportResistance {
    /// Calculates support and resistance levels from a slice of HighLow price data.
    ///
    /// This method identifies levels using a fractal-based approach, looking for a candle
    /// whose high is higher (for resistance) or low is lower (for support) than the
    /// `period` candles on both sides.
    ///
    /// # Arguments
    ///
    /// * `price_data` - A slice of `HighLow` structs representing the price history.
    /// * `period` - The number of candles to check on each side of a potential fractal.
    ///
    /// # Returns
    ///
    /// A `SupportResistanceLevels` struct containing the identified levels.
    pub fn calculate(price_data: &[HighLow], period: usize) -> SupportResistanceLevels {
        // The total window size is `period` candles left, the current candle, and `period` candles right.
        let window_size = 2 * period + 1;

        // Not enough data to form a single window, so no levels can be found.
        if period == 0 || price_data.len() < window_size {
            return SupportResistanceLevels {
                support: vec![],
                resistance: vec![],
            };
        }

        let mut supports = Vec::new();
        let mut resistances = Vec::new();

        // Iterate through the data points where a full window can be formed.
        for i in period..(price_data.len() - period) {
            let current_high = price_data[i].high;
            let current_low = price_data[i].low;

            // Check for a resistance fractal (a peak).
            // The current high must be strictly greater than all other highs in the window.
            let is_resistance = (i - period..=i + period)
                .filter(|&j| j != i) // Exclude the current candle from comparison
                .all(|j| price_data[j].high < current_high);

            // Check for a support fractal (a trough).
            // The current low must be strictly lower than all other lows in the window.
            let is_support = (i - period..=i + period)
                .filter(|&j| j != i) // Exclude the current candle from comparison
                .all(|j| price_data[j].low > current_low);

            if is_resistance {
                resistances.push(current_high);
            }
            if is_support {
                supports.push(current_low);
            }
        }

        // Sort and remove duplicates to get clean lists of levels.
        supports.sort_by(|a, b| b.partial_cmp(a).unwrap()); // Descending order
        supports.dedup();

        resistances.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Ascending order
        resistances.dedup();

        SupportResistanceLevels {
            support: supports,
            resistance: resistances,
        }
    }
}
