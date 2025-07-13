#[cfg(test)]
mod tests {
    use crate::v2::cci::{
        main::{calculate_cci_simple, CCI},
        types::{CCIConfig, CCIError, CCIInput, CCIMarketCondition},
    };

    #[test]
    fn test_cci_basic_calculation() {
        let mut cci = CCI::with_period(3).unwrap();

        // Test data with known characteristics
        let test_data = [
            (10.0, 8.0, 9.0),   // TP = 9.0
            (11.0, 9.0, 10.0),  // TP = 10.0
            (12.0, 10.0, 11.0), // TP = 11.0 - first CCI calculation
            (13.0, 11.0, 12.0), // TP = 12.0 - trend continues
            (14.0, 12.0, 13.0), // TP = 13.0 - strong trend
        ];

        for (i, (high, low, close)) in test_data.iter().enumerate() {
            let input = CCIInput {
                high: *high,
                low: *low,
                close: *close,
            };
            let result = cci.calculate(input).unwrap();

            // Typical price should be calculated correctly
            let expected_tp = (high + low + close) / 3.0;
            assert!((result.typical_price - expected_tp).abs() < 1e-10);

            // CCI should be finite
            assert!(
                result.cci.is_finite(),
                "CCI should be finite at index {}",
                i
            );

            // If we have sufficient data, mean deviation should be >= 0
            if i >= 2 {
                // After 3 periods
                assert!(result.mean_deviation >= 0.0);
            }
        }
    }

    #[test]
    fn test_cci_trending_market() {
        let mut cci = CCI::with_period(4).unwrap();

        // Strong uptrending data
        let uptrend_data = vec![
            (10.0, 8.0, 9.0),
            (12.0, 10.0, 11.0),
            (14.0, 12.0, 13.0),
            (16.0, 14.0, 15.0), // Should give positive CCI
            (18.0, 16.0, 17.0), // Should give higher positive CCI
        ];

        let mut results = Vec::new();
        for (high, low, close) in uptrend_data {
            let input = CCIInput { high, low, close };
            let result = cci.calculate(input).unwrap();
            results.push(result);
        }

        // In a strong uptrend, CCI should eventually become positive
        let last_result = &results[results.len() - 1];
        if last_result.market_condition != CCIMarketCondition::Insufficient {
            // Should show upward momentum
            assert!(last_result.cci > 0.0, "CCI should be positive in uptrend");
        }
    }

    #[test]
    fn test_cci_market_conditions() {
        let config = CCIConfig {
            period: 3,
            overbought: 100.0,
            oversold: -100.0,
            extreme_overbought: 200.0,
            extreme_oversold: -200.0,
        };
        let mut cci = CCI::with_config(config);

        // Test insufficient data
        let input1 = CCIInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
        };
        let result1 = cci.calculate(input1).unwrap();
        assert_eq!(result1.market_condition, CCIMarketCondition::Insufficient);

        // Add more data
        let input2 = CCIInput {
            high: 11.0,
            low: 9.0,
            close: 10.0,
        };
        let result2 = cci.calculate(input2).unwrap();
        assert_eq!(result2.market_condition, CCIMarketCondition::Insufficient);

        // Now should have sufficient data
        let input3 = CCIInput {
            high: 12.0,
            low: 10.0,
            close: 11.0,
        };
        let result3 = cci.calculate(input3).unwrap();
        assert_ne!(result3.market_condition, CCIMarketCondition::Insufficient);
    }

    #[test]
    fn test_cci_typical_price() {
        let mut cci = CCI::new();

        let input = CCIInput {
            high: 15.0,
            low: 10.0,
            close: 12.0,
        };
        let result = cci.calculate(input).unwrap();

        // TP = (15 + 10 + 12) / 3 = 37 / 3 = 12.333...
        let expected_tp = (15.0 + 10.0 + 12.0) / 3.0;
        assert!((result.typical_price - expected_tp).abs() < 1e-10);
    }

    #[test]
    fn test_cci_zero_deviation() {
        let mut cci = CCI::with_period(3).unwrap();

        // Identical prices - should result in zero deviation
        let identical_data = vec![
            (10.0, 10.0, 10.0), // TP = 10.0
            (10.0, 10.0, 10.0), // TP = 10.0
            (10.0, 10.0, 10.0), // TP = 10.0 - all same, zero deviation
        ];

        for (high, low, close) in identical_data {
            let input = CCIInput { high, low, close };
            let result = cci.calculate(input).unwrap();

            if result.market_condition != CCIMarketCondition::Insufficient {
                // With identical prices, mean deviation should be 0, CCI should be 0
                assert_eq!(result.mean_deviation, 0.0);
                assert_eq!(result.cci, 0.0);
            }
        }
    }

    #[test]
    fn test_cci_error_handling() {
        let mut cci = CCI::new();

        // Test invalid HLC (high < low)
        let input = CCIInput {
            high: 8.0,
            low: 10.0,
            close: 9.0,
        };
        assert!(matches!(cci.calculate(input), Err(CCIError::InvalidHLC)));

        // Test close out of range
        let input = CCIInput {
            high: 10.0,
            low: 8.0,
            close: 12.0,
        };
        assert!(matches!(cci.calculate(input), Err(CCIError::InvalidHLC)));

        // Test invalid period
        assert!(matches!(CCI::with_period(0), Err(CCIError::InvalidPeriod)));

        // Test invalid thresholds
        assert!(matches!(
            CCI::with_thresholds(20, -100.0, 100.0, 50.0, -200.0), // overbought < oversold
            Err(CCIError::InvalidThresholds)
        ));
    }

    #[test]
    fn test_cci_simple_function() {
        let highs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let lows = vec![8.0, 9.0, 10.0, 11.0, 12.0];
        let closes = vec![9.0, 10.0, 11.0, 12.0, 13.0];

        let result = calculate_cci_simple(&highs, &lows, &closes, 3).unwrap();
        assert_eq!(result.len(), 5);

        // All CCI values should be finite
        for cci_value in result {
            assert!(cci_value.is_finite());
        }
    }

    #[test]
    fn test_cci_boundary_conditions() {
        let mut cci = CCI::with_period(3).unwrap();

        // Test with very small price ranges
        let small_range_data = vec![
            (10.001, 10.000, 10.0005),
            (10.002, 10.001, 10.0015),
            (10.003, 10.002, 10.0025),
        ];

        for (high, low, close) in small_range_data {
            let input = CCIInput { high, low, close };
            let result = cci.calculate(input).unwrap();

            // Should handle small ranges without issues
            assert!(result.cci.is_finite());
            assert!(result.mean_deviation >= 0.0);
        }
    }

    #[test]
    fn test_cci_helper_methods() {
        let cci = CCI::new();

        // Test threshold checking methods
        assert!(cci.is_overbought(150.0));
        assert!(!cci.is_overbought(50.0));

        assert!(cci.is_oversold(-150.0));
        assert!(!cci.is_oversold(-50.0));

        assert!(cci.is_extreme_condition(250.0));
        assert!(cci.is_extreme_condition(-250.0));
        assert!(!cci.is_extreme_condition(50.0));
    }
}
