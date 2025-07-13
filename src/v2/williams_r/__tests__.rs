#[cfg(test)]
mod tests {
    use crate::v2::williams_r::{
        main::{calculate_williams_r_simple, WilliamsR},
        types::{WilliamsRConfig, WilliamsRError, WilliamsRInput, WilliamsRMarketCondition},
    };

    #[test]
    fn test_williams_r_basic_calculation() {
        let mut williams_r = WilliamsR::with_period(3).unwrap();

        // Test data with known characteristics
        let test_data = [
            (10.0, 8.0, 9.0),   // First point
            (12.0, 9.0, 11.0),  // Second point
            (11.0, 8.5, 10.0),  // Third point - first calculation
            (13.0, 10.0, 12.5), // Close near high - should be overbought
            (9.0, 7.0, 7.5),    // Close near low - should be oversold
        ];

        for (i, (high, low, close)) in test_data.iter().enumerate() {
            let input = WilliamsRInput {
                high: *high,
                low: *low,
                close: *close,
            };
            let result = williams_r.calculate(input).unwrap();

            // Williams %R should be between 0 and -100
            assert!(
                result.williams_r >= -100.0 && result.williams_r <= 0.0,
                "Williams %R {} out of range at index {}",
                result.williams_r,
                i
            );

            // Highest high should be >= current high
            if i >= 2 {
                // After sufficient data
                assert!(result.highest_high >= *high);
                assert!(result.lowest_low <= *low);
            }
        }
    }

    #[test]
    fn test_williams_r_overbought_oversold() {
        let mut williams_r = WilliamsR::with_period(3).unwrap();

        // Setup initial data
        let setup_data = vec![(10.0, 8.0, 9.0), (12.0, 9.0, 10.0), (11.0, 8.0, 9.5)];

        for (high, low, close) in setup_data {
            let input = WilliamsRInput { high, low, close };
            let _ = williams_r.calculate(input).unwrap();
        }

        // Test overbought condition (close near highest high)
        // HH=12, LL=8, Close=11.8 should give Williams %R near 0 (overbought)
        let overbought_input = WilliamsRInput {
            high: 12.0,
            low: 10.0,
            close: 11.8,
        };
        let overbought_result = williams_r.calculate(overbought_input).unwrap();

        // Should be overbought (close to 0)
        assert!(overbought_result.williams_r > -20.0, "Should be overbought");
        assert!(williams_r.is_overbought(overbought_result.williams_r));

        // Test oversold condition (close near lowest low)
        // Using a close near the lowest low should give Williams %R near -100 (oversold)
        let oversold_input = WilliamsRInput {
            high: 9.0,
            low: 7.0,
            close: 7.2,
        };
        let oversold_result = williams_r.calculate(oversold_input).unwrap();

        // Should be oversold (close to -100)
        assert!(oversold_result.williams_r < -80.0, "Should be oversold");
        assert!(williams_r.is_oversold(oversold_result.williams_r));
    }

    #[test]
    fn test_williams_r_formula() {
        let mut williams_r = WilliamsR::with_period(3).unwrap();

        // Setup known data
        let setup_data = vec![(15.0, 10.0, 12.0), (16.0, 11.0, 13.0), (14.0, 9.0, 11.0)];

        for (high, low, close) in setup_data {
            let input = WilliamsRInput { high, low, close };
            let _ = williams_r.calculate(input).unwrap();
        }

        // Test with known values: HH=16, LL=9, Close=12
        let test_input = WilliamsRInput {
            high: 13.0,
            low: 11.0,
            close: 12.0,
        };
        let result = williams_r.calculate(test_input).unwrap();

        // Williams %R = (16 - 12) / (16 - 9) × -100 = 4/7 × -100 ≈ -57.14
        let expected = ((16.0 - 12.0) / (16.0 - 9.0)) * -100.0;
        assert!(
            (result.williams_r - expected).abs() < 0.01,
            "Expected {}, got {}",
            expected,
            result.williams_r
        );
    }

    #[test]
    fn test_williams_r_extreme_conditions() {
        let config = WilliamsRConfig {
            period: 3,
            overbought: -20.0,
            oversold: -80.0,
            extreme_overbought: -10.0,
            extreme_oversold: -90.0,
        };
        let mut williams_r = WilliamsR::with_config(config);

        // Setup data
        let setup_data = vec![(10.0, 8.0, 9.0), (12.0, 9.0, 10.0), (11.0, 8.0, 9.5)];

        for (high, low, close) in setup_data {
            let input = WilliamsRInput { high, low, close };
            let _ = williams_r.calculate(input).unwrap();
        }

        // Test extreme overbought
        assert!(williams_r.is_extreme_condition(-5.0)); // Very overbought

        // Test extreme oversold
        assert!(williams_r.is_extreme_condition(-95.0)); // Very oversold

        // Test normal condition
        assert!(!williams_r.is_extreme_condition(-50.0)); // Normal range
    }

    #[test]
    fn test_williams_r_signal_strength() {
        let williams_r = WilliamsR::new();

        // Test signal strength calculation
        assert!((williams_r.signal_strength(-5.0) - 0.9).abs() < 0.1); // Strong overbought
        assert!((williams_r.signal_strength(-95.0) - 0.9).abs() < 0.1); // Strong oversold
        assert!((williams_r.signal_strength(-50.0) - 0.0).abs() < 0.1); // Neutral
    }

    #[test]
    fn test_williams_r_zero_range() {
        let mut williams_r = WilliamsR::with_period(3).unwrap();

        // Test with identical prices (zero range)
        let identical_data = vec![(10.0, 10.0, 10.0), (10.0, 10.0, 10.0), (10.0, 10.0, 10.0)];

        for (high, low, close) in identical_data {
            let input = WilliamsRInput { high, low, close };
            let result = williams_r.calculate(input).unwrap();

            if result.market_condition != WilliamsRMarketCondition::Insufficient {
                // With zero range, should return middle value
                assert_eq!(result.williams_r, -50.0);
                assert_eq!(result.price_range, 0.0);
            }
        }
    }

    #[test]
    fn test_williams_r_error_handling() {
        let mut williams_r = WilliamsR::new();

        // Test invalid HLC (high < low)
        let input = WilliamsRInput {
            high: 8.0,
            low: 10.0,
            close: 9.0,
        };
        assert!(matches!(
            williams_r.calculate(input),
            Err(WilliamsRError::InvalidHLC)
        ));

        // Test close out of range
        let input = WilliamsRInput {
            high: 10.0,
            low: 8.0,
            close: 12.0,
        };
        assert!(matches!(
            williams_r.calculate(input),
            Err(WilliamsRError::InvalidHLC)
        ));

        // Test invalid period
        assert!(matches!(
            WilliamsR::with_period(0),
            Err(WilliamsRError::InvalidPeriod)
        ));

        // Test invalid thresholds (positive values)
        assert!(matches!(
            WilliamsR::with_thresholds(14, 20.0, -80.0, -10.0, -90.0),
            Err(WilliamsRError::InvalidThresholds)
        ));
    }

    #[test]
    fn test_williams_r_simple_function() {
        let highs = vec![10.0, 12.0, 11.0, 13.0, 9.0];
        let lows = vec![8.0, 9.0, 8.5, 10.0, 7.0];
        let closes = vec![9.0, 11.0, 10.0, 12.5, 7.5];

        let result = calculate_williams_r_simple(&highs, &lows, &closes, 3).unwrap();
        assert_eq!(result.len(), 5);

        // All Williams %R values should be between 0 and -100
        for wr_value in result {
            assert!(wr_value >= -100.0 && wr_value <= 0.0);
        }
    }

    #[test]
    fn test_williams_r_market_conditions() {
        let mut williams_r = WilliamsR::with_period(2).unwrap();

        // Test insufficient data
        let input1 = WilliamsRInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
        };
        let result1 = williams_r.calculate(input1).unwrap();
        assert_eq!(
            result1.market_condition,
            WilliamsRMarketCondition::Insufficient
        );

        // Add more data to get sufficient data
        let input2 = WilliamsRInput {
            high: 12.0,
            low: 10.0,
            close: 11.0,
        };
        let result2 = williams_r.calculate(input2).unwrap();
        assert_ne!(
            result2.market_condition,
            WilliamsRMarketCondition::Insufficient
        );
    }
}
