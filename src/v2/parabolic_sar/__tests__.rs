#[cfg(test)]
mod tests {
    use crate::v2::parabolic_sar::{
        main::{calculate_parabolic_sar_simple, ParabolicSAR},
        types::{ParabolicSARError, ParabolicSARInput, TrendDirection},
    };

    #[test]
    fn test_parabolic_sar_basic_calculation() {
        let mut sar = ParabolicSAR::new();

        // Test data with increasing prices (uptrend)
        let test_data = vec![
            (10.0, 8.0),  // First point
            (11.0, 9.0),  // Second point - determine trend
            (12.0, 10.0), // Continue uptrend
            (13.0, 11.0), // Continue uptrend
            (12.5, 10.5), // Still uptrend
        ];

        let mut results = Vec::new();
        for (high, low) in &test_data {
            let input = ParabolicSARInput {
                high: *high,
                low: *low,
                close: None,
            };
            let result = sar.calculate(input).unwrap();
            results.push(result);
        }

        // Check that SAR values are reasonable
        assert!(results.len() == 5);

        // In an uptrend, SAR should generally be below the lows
        for (i, result) in results.iter().enumerate().skip(2) {
            if matches!(result.trend, TrendDirection::Up) {
                // SAR should be below current low in uptrend
                assert!(
                    result.sar <= test_data[i].1,
                    "SAR {} should be <= low {} at index {}",
                    result.sar,
                    test_data[i].1,
                    i
                );
            }
        }
    }

    #[test]
    fn test_parabolic_sar_trend_reversal() {
        let mut sar = ParabolicSAR::new();

        // Create data that starts up then reverses down
        // We need more data points to establish a proper trend first
        let test_data = [
            (10.0, 8.0),  // First
            (12.0, 10.0), // Up trend starts (high > prev_high)
            (14.0, 12.0), // Continue up, establish trend
            (15.0, 13.0), // Continue up, build SAR
            (13.0, 7.0),  // Sharp drop - should trigger reversal (low crosses below SAR)
        ];

        let mut results = Vec::new();
        for (i, (high, low)) in test_data.iter().enumerate() {
            let input = ParabolicSARInput {
                high: *high,
                low: *low,
                close: None,
            };
            let result = sar.calculate(input).unwrap();
            results.push(result);
            println!(
                "Period {}: High={}, Low={}, SAR={:.4}, Trend={:?}, Reversal={}",
                i, high, low, result.sar, result.trend, result.trend_reversal
            );
        }

        // Check that we eventually get a trend reversal
        let has_reversal = results.iter().any(|r| r.trend_reversal);
        assert!(
            has_reversal,
            "Expected at least one trend reversal in the data"
        );

        // The reversal should happen when the low crosses below the SAR
        // Find the reversal and verify it makes sense
        for (i, result) in results.iter().enumerate() {
            if result.trend_reversal {
                println!("Reversal detected at period {}: {:?}", i, result);
                assert_eq!(result.trend, TrendDirection::Down);
                break;
            }
        }
    }

    #[test]
    fn test_parabolic_sar_acceleration_factor() {
        let mut sar = ParabolicSAR::new();

        // Test that AF increases with new extreme points
        let test_data = [
            (10.0, 8.0),
            (11.0, 9.0),  // Start uptrend
            (12.0, 10.0), // New high - AF should increase
            (13.0, 11.0), // New high - AF should increase again
        ];

        let mut prev_af = 0.0;
        for (i, (high, low)) in test_data.iter().enumerate() {
            let input = ParabolicSARInput {
                high: *high,
                low: *low,
                close: None,
            };
            let result = sar.calculate(input).unwrap();

            if i > 1 && !result.trend_reversal {
                // AF should increase when we have new extreme points
                if result.extreme_point > test_data[i - 1].0 {
                    assert!(result.acceleration_factor >= prev_af);
                }
            }
            prev_af = result.acceleration_factor;
        }
    }

    #[test]
    fn test_parabolic_sar_error_handling() {
        let mut sar = ParabolicSAR::new();

        // Test invalid HL (high < low)
        let input = ParabolicSARInput {
            high: 8.0,
            low: 10.0,
            close: None,
        };
        assert!(matches!(
            sar.calculate(input),
            Err(ParabolicSARError::InvalidHL)
        ));

        // Test invalid acceleration parameters
        assert!(matches!(
            ParabolicSAR::with_acceleration(0.0, 0.02, 0.20),
            Err(ParabolicSARError::InvalidAcceleration)
        ));

        assert!(matches!(
            ParabolicSAR::with_acceleration(0.02, 0.0, 0.20),
            Err(ParabolicSARError::InvalidAcceleration)
        ));

        assert!(matches!(
            ParabolicSAR::with_acceleration(0.20, 0.02, 0.10),
            Err(ParabolicSARError::InvalidAcceleration)
        ));
    }

    #[test]
    fn test_parabolic_sar_simple_function() {
        let highs = vec![10.0, 11.0, 12.0, 13.0, 12.5];
        let lows = vec![8.0, 9.0, 10.0, 11.0, 10.5];

        let result = calculate_parabolic_sar_simple(&highs, &lows, None, None, None).unwrap();
        assert_eq!(result.len(), 5);

        // All SAR values should be finite
        for sar_value in result {
            assert!(sar_value.is_finite());
        }
    }

    #[test]
    fn test_trend_direction_consistency() {
        let mut sar = ParabolicSAR::new();

        // Strong uptrend data
        let uptrend_data = vec![
            (10.0, 8.0),
            (11.0, 9.0),
            (12.0, 10.0),
            (13.0, 11.0),
            (14.0, 12.0),
        ];

        for (high, low) in uptrend_data {
            let input = ParabolicSARInput {
                high,
                low,
                close: None,
            };
            let result = sar.calculate(input).unwrap();

            // After the first two setup periods, should be in uptrend
            if sar.state.trend_periods > 2 && !result.trend_reversal {
                assert_eq!(result.trend, TrendDirection::Up);
            }
        }
    }
}
