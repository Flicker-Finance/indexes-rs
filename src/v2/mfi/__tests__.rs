#[cfg(test)]
mod tests {
    use crate::v2::mfi::{
        main::{calculate_mfi_simple, MFI},
        types::{MFIConfig, MFIError, MFIInput, MFIMarketCondition},
    };

    #[test]
    fn test_mfi_basic_calculation() {
        let mut mfi = MFI::with_period(3).unwrap();

        // Test data with known MFI values
        let test_data = vec![
            (10.0, 8.0, 9.0, 1000.0),   // TP = 9.0
            (11.0, 9.0, 10.0, 1500.0),  // TP = 10.0 (up)
            (10.5, 8.5, 9.0, 1200.0),   // TP = 9.33 (down)
            (12.0, 10.0, 11.0, 1800.0), // TP = 11.0 (up)
        ];

        for (high, low, close, volume) in test_data {
            let input = MFIInput {
                high,
                low,
                close,
                volume,
            };
            let result = mfi.calculate(input).unwrap();

            // MFI should be between 0 and 100
            assert!(result.mfi >= 0.0 && result.mfi <= 100.0);

            // Typical price should be calculated correctly
            let expected_tp = (high + low + close) / 3.0;
            assert!((result.typical_price - expected_tp).abs() < 1e-10);
        }
    }

    #[test]
    fn test_mfi_market_conditions() {
        let config = MFIConfig {
            period: 2,
            overbought: 80.0,
            oversold: 20.0,
        };
        let mut mfi = MFI::with_config(config);

        // First input - insufficient data
        let input1 = MFIInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
            volume: 1000.0,
        };
        let result1 = mfi.calculate(input1).unwrap();
        assert_eq!(result1.market_condition, MFIMarketCondition::Insufficient);

        // Add more data to get actual MFI calculation
        let input2 = MFIInput {
            high: 15.0,
            low: 13.0,
            close: 14.0,
            volume: 2000.0,
        };
        let result2 = mfi.calculate(input2).unwrap();

        // Should have sufficient data now
        assert_ne!(result2.market_condition, MFIMarketCondition::Insufficient);
    }

    #[test]
    fn test_mfi_flow_direction() {
        let mut mfi = MFI::with_period(2).unwrap();

        let input1 = MFIInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
            volume: 1000.0,
        };
        let result1 = mfi.calculate(input1).unwrap();
        assert_eq!(result1.flow_direction, 0.0); // First calculation

        let input2 = MFIInput {
            high: 12.0,
            low: 10.0,
            close: 11.0,
            volume: 1500.0,
        };
        let result2 = mfi.calculate(input2).unwrap();
        assert_eq!(result2.flow_direction, 1.0); // TP increased

        let input3 = MFIInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
            volume: 1200.0,
        };
        let result3 = mfi.calculate(input3).unwrap();
        assert_eq!(result3.flow_direction, -1.0); // TP decreased
    }

    #[test]
    fn test_mfi_error_handling() {
        let mut mfi = MFI::new();

        // Test invalid OHLC (high < low)
        let input = MFIInput {
            high: 8.0,
            low: 10.0,
            close: 9.0,
            volume: 1000.0,
        };
        assert!(matches!(mfi.calculate(input), Err(MFIError::InvalidOHLC)));

        // Test negative volume
        let input = MFIInput {
            high: 10.0,
            low: 8.0,
            close: 9.0,
            volume: -1000.0,
        };
        assert!(matches!(
            mfi.calculate(input),
            Err(MFIError::NegativeVolume)
        ));

        // Test invalid period
        assert!(matches!(MFI::with_period(0), Err(MFIError::InvalidPeriod)));
    }

    #[test]
    fn test_mfi_simple_function() {
        let highs = vec![10.0, 11.0, 10.5, 12.0];
        let lows = vec![8.0, 9.0, 8.5, 10.0];
        let closes = vec![9.0, 10.0, 9.0, 11.0];
        let volumes = vec![1000.0, 1500.0, 1200.0, 1800.0];

        let result = calculate_mfi_simple(&highs, &lows, &closes, &volumes, 3).unwrap();
        assert_eq!(result.len(), 4);

        // All MFI values should be between 0 and 100
        for mfi_value in result {
            assert!((0.0..=100.0).contains(&mfi_value));
        }
    }
}
