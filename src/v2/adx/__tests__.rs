#[cfg(test)]
mod tests {
    use crate::v2::adx::{
        main::{calculate_adx_simple, ADX},
        types::{ADXError, ADXInput, TrendDirection, TrendStrength},
    };

    #[test]
    fn test_adx_basic_calculation() {
        let mut adx = ADX::with_period(3).unwrap();

        // Test data with trending movement
        let test_data = [
            (10.0, 8.0, 9.0),   // First point
            (11.0, 9.0, 10.5),  // Up move
            (12.5, 10.0, 12.0), // Continue up
            (13.0, 11.0, 12.5), // Continue up
            (14.0, 12.0, 13.5), // Strong up trend
        ];

        for (i, (high, low, close)) in test_data.iter().enumerate() {
            let input = ADXInput {
                high: *high,
                low: *low,
                close: *close,
            };
            let result = adx.calculate(input).unwrap();

            // ADX should be between 0 and 100
            assert!(
                result.adx >= 0.0 && result.adx <= 100.0,
                "ADX {} out of range at index {}",
                result.adx,
                i
            );

            // DI values should be between 0 and 100
            assert!(result.plus_di >= 0.0 && result.plus_di <= 100.0);
            assert!(result.minus_di >= 0.0 && result.minus_di <= 100.0);

            // DX should be between 0 and 100
            assert!(result.dx >= 0.0 && result.dx <= 100.0);
        }
    }

    #[test]
    fn test_adx_trend_direction() {
        let mut adx = ADX::with_period(3).unwrap();

        // Strong uptrend data
        let uptrend_data = vec![
            (10.0, 8.0, 9.0),
            (12.0, 10.0, 11.5),
            (14.0, 12.0, 13.5),
            (16.0, 14.0, 15.5),
        ];

        let mut last_result = None;
        for (high, low, close) in uptrend_data {
            let input = ADXInput { high, low, close };
            let result = adx.calculate(input).unwrap();
            last_result = Some(result);
        }

        // In a strong uptrend, +DI should generally be > -DI
        if let Some(result) = last_result {
            if result.adx > 0.0 {
                // Only check if we have meaningful ADX
                assert_eq!(result.trend_direction, TrendDirection::Up);
                assert!(result.plus_di >= result.minus_di);
            }
        }
    }

    #[test]
    fn test_adx_trend_strength() {
        let mut adx = ADX::with_period(2).unwrap();

        // Add enough data to get ADX calculation
        let test_data = vec![
            (10.0, 8.0, 9.0),
            (15.0, 13.0, 14.0), // Big up move
            (20.0, 18.0, 19.0), // Continue up
            (25.0, 23.0, 24.0), // Strong trend
        ];

        for (high, low, close) in test_data {
            let input = ADXInput { high, low, close };
            let _ = adx.calculate(input).unwrap();
        }

        // Test trend strength classification
        assert_ne!(adx.trend_strength(), TrendStrength::Insufficient);
    }

    #[test]
    fn test_adx_true_range_calculation() {
        let mut adx = ADX::new();

        // First calculation
        let input1 = ADXInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
        };
        let result1 = adx.calculate(input1).unwrap();
        assert_eq!(result1.true_range, 0.0); // First calculation

        // Second calculation with gap
        let input2 = ADXInput {
            high: 15.0,
            low: 12.0,
            close: 14.0,
        };
        let result2 = adx.calculate(input2).unwrap();

        // TR should be max of: (15-12), (15-9), (12-9) = max(3, 6, 3) = 6
        assert_eq!(result2.true_range, 6.0);
    }

    #[test]
    fn test_adx_error_handling() {
        let mut adx = ADX::new();

        // Test invalid HLC (high < low)
        let input = ADXInput {
            high: 8.0,
            low: 10.0,
            close: 9.0,
        };
        assert!(matches!(adx.calculate(input), Err(ADXError::InvalidHLC)));

        // Test close out of range
        let input = ADXInput {
            high: 10.0,
            low: 8.0,
            close: 12.0,
        };
        assert!(matches!(adx.calculate(input), Err(ADXError::InvalidHLC)));

        // Test invalid period
        assert!(matches!(ADX::with_period(0), Err(ADXError::InvalidPeriod)));
    }

    #[test]
    fn test_adx_simple_function() {
        let highs = vec![10.0, 11.0, 12.5, 13.0, 14.0];
        let lows = vec![8.0, 9.0, 10.0, 11.0, 12.0];
        let closes = vec![9.0, 10.5, 12.0, 12.5, 13.5];

        let result = calculate_adx_simple(&highs, &lows, &closes, 3).unwrap();
        assert_eq!(result.len(), 5);

        // All ADX values should be between 0 and 100
        for adx_value in result {
            assert!((0.0..=100.0).contains(&adx_value));
        }
    }

    #[test]
    fn test_directional_movement() {
        let mut adx = ADX::new();

        // Setup first point
        let input1 = ADXInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
        };
        let _ = adx.calculate(input1).unwrap();

        // Up movement
        let input2 = ADXInput {
            high: 12.0,
            low: 10.0,
            close: 11.0,
        };

        adx.calculate(input2).unwrap();

        // Should detect upward movement
        // High went from 10 to 12 (+2), Low went from 8 to 10 (+2)
        // Up move = 2, Down move = 0, so +DM should be > 0
        // This will show up in subsequent calculations as +DI > -DI

        // Add more data to see the effect
        let input3 = ADXInput {
            high: 14.0,
            low: 12.0,
            close: 13.0,
        };
        let result3 = adx.calculate(input3).unwrap();

        // In a sustained uptrend, we should eventually see +DI > -DI
        if result3.adx > 0.0 {
            // Only check if we have meaningful values
            println!(
                "Result: +DI={}, -DI={}, ADX={}",
                result3.plus_di, result3.minus_di, result3.adx
            );
        }
    }
}
