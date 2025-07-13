#[cfg(test)]
mod tests {
    use crate::v2::obv::{
        main::{calculate_obv_simple, OBV},
        types::{OBVError, OBVInput},
    };

    #[test]
    fn test_obv_basic_calculation() {
        let mut obv = OBV::new();

        // Test data: prices going up, down, unchanged
        let test_cases = [
            (100.0, 1000.0), // First day
            (105.0, 1500.0), // Price up -> add volume
            (103.0, 1200.0), // Price down -> subtract volume
            (103.0, 800.0),  // Price unchanged -> no change
            (107.0, 2000.0), // Price up -> add volume
        ];

        let expected_obv = [
            1000.0, // First day
            2500.0, // 1000 + 1500
            1300.0, // 2500 - 1200
            1300.0, // No change
            3300.0, // 1300 + 2000
        ];

        for (i, (close, volume)) in test_cases.iter().enumerate() {
            let input = OBVInput {
                close: *close,
                volume: *volume,
            };
            let result = obv.calculate(input).unwrap();
            assert_eq!(result.obv, expected_obv[i], "Failed at index {}", i);
        }
    }

    #[test]
    fn test_obv_flow_direction() {
        let mut obv = OBV::new();

        let input1 = OBVInput {
            close: 100.0,
            volume: 1000.0,
        };
        let result1 = obv.calculate(input1).unwrap();
        assert_eq!(result1.flow_direction, 0.0); // First calculation

        let input2 = OBVInput {
            close: 105.0,
            volume: 1500.0,
        };
        let result2 = obv.calculate(input2).unwrap();
        assert_eq!(result2.flow_direction, 1.0); // Up

        let input3 = OBVInput {
            close: 103.0,
            volume: 1200.0,
        };
        let result3 = obv.calculate(input3).unwrap();
        assert_eq!(result3.flow_direction, -1.0); // Down

        let input4 = OBVInput {
            close: 103.0,
            volume: 800.0,
        };
        let result4 = obv.calculate(input4).unwrap();
        assert_eq!(result4.flow_direction, 0.0); // Unchanged
    }

    #[test]
    fn test_obv_simple_function() {
        let closes = vec![100.0, 105.0, 103.0, 103.0, 107.0];
        let volumes = vec![1000.0, 1500.0, 1200.0, 800.0, 2000.0];

        let result = calculate_obv_simple(&closes, &volumes).unwrap();
        let expected = vec![1000.0, 2500.0, 1300.0, 1300.0, 3300.0];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_obv_error_handling() {
        let mut obv = OBV::new();

        // Test negative volume
        let input = OBVInput {
            close: 100.0,
            volume: -1000.0,
        };
        assert!(matches!(
            obv.calculate(input),
            Err(OBVError::NegativeVolume)
        ));

        // Test invalid price
        let input = OBVInput {
            close: f64::NAN,
            volume: 1000.0,
        };
        assert!(matches!(obv.calculate(input), Err(OBVError::InvalidPrice)));
    }
}
