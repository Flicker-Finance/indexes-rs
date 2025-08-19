#[cfg(test)]
mod tests {
    use crate::v2::support_resistance::{
        main::SupportResistance,
        types::{HighLow, SupportResistanceLevels},
    };

    #[test]
    fn test_basic_level_detection() {
        let price_data = vec![
            HighLow { high: 105.0, low: 95.0 },
            HighLow { high: 110.0, low: 100.0 },
            HighLow { high: 120.0, low: 108.0 }, // Resistance
            HighLow { high: 112.0, low: 102.0 },
            HighLow { high: 115.0, low: 105.0 },
            HighLow { high: 100.0, low: 90.0 },
            HighLow { high: 98.0, low: 85.0 }, // Support
            HighLow { high: 101.0, low: 88.0 },
            HighLow { high: 105.0, low: 92.0 },
        ];
        let period = 2;
        let expected = SupportResistanceLevels {
            support: vec![85.0],
            resistance: vec![120.0],
        };
        let result = SupportResistance::calculate(&price_data, period);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_insufficient_data() {
        let price_data = vec![HighLow { high: 1.0, low: 1.0 }, HighLow { high: 2.0, low: 2.0 }, HighLow { high: 3.0, low: 3.0 }];
        let period = 2; // Requires 2*2+1 = 5 data points
        let expected = SupportResistanceLevels {
            support: vec![],
            resistance: vec![],
        };
        let result = SupportResistance::calculate(&price_data, period);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_levels_found() {
        // Monotonically increasing data, so no fractals should be found.
        let price_data = vec![
            HighLow { high: 1.0, low: 0.5 },
            HighLow { high: 2.0, low: 1.5 },
            HighLow { high: 3.0, low: 2.5 },
            HighLow { high: 4.0, low: 3.5 },
            HighLow { high: 5.0, low: 4.5 },
        ];
        let period = 1;
        let expected = SupportResistanceLevels {
            support: vec![],
            resistance: vec![],
        };
        let result = SupportResistance::calculate(&price_data, period);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_duplicate_levels_and_sorting() {
        let price_data = vec![
            HighLow { high: 110.0, low: 98.0 },
            HighLow { high: 120.0, low: 105.0 }, // R1
            HighLow { high: 115.0, low: 100.0 },
            HighLow { high: 100.0, low: 80.0 }, // S1
            HighLow { high: 105.0, low: 85.0 },
            HighLow { high: 110.0, low: 90.0 },
            HighLow { high: 120.0, low: 102.0 }, // R2 (duplicate)
            HighLow { high: 112.0, low: 95.0 },
            HighLow { high: 99.0, low: 75.0 }, // S2
            HighLow { high: 105.0, low: 80.0 },
        ];
        let period = 1;
        let expected = SupportResistanceLevels {
            support: vec![80.0, 75.0], // Sorted descending
            resistance: vec![120.0],   // Sorted ascending, duplicates removed
        };
        let result = SupportResistance::calculate(&price_data, period);
        assert_eq!(result, expected);
    }
}
