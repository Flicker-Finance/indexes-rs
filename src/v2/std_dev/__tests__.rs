#[cfg(test)]
mod tests {
    use crate::v2::std_dev::{
        main::{
            calculate_standard_deviation_simple, rolling_standard_deviation, StandardDeviation,
        },
        types::{StandardDeviationError, StandardDeviationInput},
    };

    #[test]
    fn test_std_dev_basic_calculation() {
        let mut std_dev = StandardDeviation::with_period(3).unwrap();

        // Test with known values: [1, 2, 3]
        // Mean = 2
        // For sample std dev: Variance = ((1-2)² + (2-2)² + (3-2)²) / (3-1) = (1 + 0 + 1) / 2 = 1
        // Std Dev = √1 = 1
        let test_values = vec![1.0, 2.0, 3.0];

        let mut results = Vec::new();
        for value in test_values {
            let input = StandardDeviationInput { value };
            let result = std_dev.calculate(input).unwrap();
            results.push(result);
        }

        // Check the final result (when we have all 3 values)
        let final_result = &results[2];
        assert!((final_result.mean - 2.0).abs() < 1e-10);
        // For sample variance: 2/2 = 1
        assert!((final_result.variance - 1.0).abs() < 1e-10);
        assert!((final_result.std_dev - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_std_dev_sample_vs_population() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        // Population standard deviation
        let mut pop_std_dev = StandardDeviation::population(3).unwrap();
        // Sample standard deviation
        let mut sample_std_dev = StandardDeviation::sample(3).unwrap();

        for value in values {
            let input = StandardDeviationInput { value };
            let _ = pop_std_dev.calculate(input).unwrap();
            let _ = sample_std_dev.calculate(input).unwrap();
        }

        // Sample std dev should be larger than population std dev
        let pop_result = pop_std_dev
            .calculate(StandardDeviationInput { value: 5.0 })
            .unwrap();
        let sample_result = sample_std_dev
            .calculate(StandardDeviationInput { value: 5.0 })
            .unwrap();

        assert!(sample_result.std_dev > pop_result.std_dev);
    }

    #[test]
    fn test_std_dev_z_score() {
        let mut std_dev = StandardDeviation::with_period(5).unwrap();

        // Add values with known mean and std dev: [10, 12, 14, 16, 18]
        // Mean = 14, sample variance = 10, sample std dev = sqrt(10) ≈ 3.16
        let values = vec![10.0, 12.0, 14.0, 16.0, 18.0];

        for value in values {
            let input = StandardDeviationInput { value };
            let _ = std_dev.calculate(input).unwrap();
        }

        // Get the state after the normal values
        let baseline_state = std_dev.get_state().clone();

        // Test z-score calculation with a normal value first
        let normal_input = StandardDeviationInput { value: 14.0 }; // Should be close to mean
        let normal_result = std_dev.calculate(normal_input).unwrap();

        // Z-score for the mean should be close to 0
        assert!(normal_result.z_score.abs() < 1.0);

        // Reset to baseline and test with a more extreme outlier
        std_dev.set_state(baseline_state);

        // Test with a value that's definitely an outlier based on the original data
        let outlier_input = StandardDeviationInput { value: 30.0 }; // Much higher
        let outlier_result = std_dev.calculate(outlier_input).unwrap();

        // Z-score should be positive and significant
        assert!(
            outlier_result.z_score > 1.0,
            "Z-score {} should be > 1.0",
            outlier_result.z_score
        );
        println!(
            "Outlier Z-score: {}, Std Dev: {}, Mean: {}",
            outlier_result.z_score, outlier_result.std_dev, outlier_result.mean
        );
    }

    #[test]
    fn test_std_dev_coefficient_of_variation() {
        let mut std_dev = StandardDeviation::with_period(3).unwrap();

        let values = vec![100.0, 110.0, 90.0];
        let mut final_result = None;

        for value in values {
            let input = StandardDeviationInput { value };
            final_result = Some(std_dev.calculate(input).unwrap());
        }

        let result = final_result.unwrap();

        // CV should be (std_dev / mean) * 100
        let expected_cv = (result.std_dev / result.mean.abs()) * 100.0;
        assert!((result.coefficient_of_variation - expected_cv).abs() < 1e-10);
    }

    #[test]
    fn test_std_dev_volatility_classification() {
        let mut low_vol = StandardDeviation::with_period(3).unwrap();
        let mut high_vol = StandardDeviation::with_period(3).unwrap();

        // Low volatility data (tight range)
        let low_vol_values = vec![100.0, 100.1, 99.9];
        for value in low_vol_values {
            let input = StandardDeviationInput { value };
            let _ = low_vol.calculate(input).unwrap();
        }

        // High volatility data (wide range)
        let high_vol_values = vec![100.0, 150.0, 50.0];
        for value in high_vol_values {
            let input = StandardDeviationInput { value };
            let _ = high_vol.calculate(input).unwrap();
        }

        let low_result = low_vol
            .calculate(StandardDeviationInput { value: 100.0 })
            .unwrap();
        let high_result = high_vol
            .calculate(StandardDeviationInput { value: 100.0 })
            .unwrap();

        // High volatility should have higher coefficient of variation
        assert!(high_result.coefficient_of_variation > low_result.coefficient_of_variation);
    }

    #[test]
    fn test_std_dev_identical_values() {
        let mut std_dev = StandardDeviation::with_period(3).unwrap();

        // All identical values should have zero standard deviation
        let values = vec![5.0, 5.0, 5.0];
        let mut final_result = None;

        for value in values {
            let input = StandardDeviationInput { value };
            final_result = Some(std_dev.calculate(input).unwrap());
        }

        let result = final_result.unwrap();

        assert_eq!(result.std_dev, 0.0);
        assert_eq!(result.variance, 0.0);
        assert_eq!(result.mean, 5.0);
        assert_eq!(result.z_score, 0.0);
    }

    #[test]
    fn test_std_dev_error_handling() {
        // Test invalid period
        assert!(matches!(
            StandardDeviation::with_period(0),
            Err(StandardDeviationError::InvalidPeriod)
        ));
        assert!(matches!(
            StandardDeviation::sample(1),
            Err(StandardDeviationError::InvalidPeriod)
        ));

        // Test invalid value
        let mut std_dev = StandardDeviation::new();
        let invalid_input = StandardDeviationInput { value: f64::NAN };
        assert!(matches!(
            std_dev.calculate(invalid_input),
            Err(StandardDeviationError::InvalidValue)
        ));
    }

    #[test]
    fn test_std_dev_simple_function() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let result = calculate_standard_deviation_simple(&values, 3, true).unwrap();
        assert_eq!(result.len(), 5);

        // All standard deviation values should be finite and non-negative
        for std_dev_value in result {
            assert!(std_dev_value.is_finite());
            assert!(std_dev_value >= 0.0);
        }
    }

    #[test]
    fn test_rolling_standard_deviation() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let result = rolling_standard_deviation(&values, 3, false).unwrap();
        assert_eq!(result.len(), 5);

        // Check that we get reasonable results
        for std_dev_value in result {
            assert!(std_dev_value.is_finite());
            assert!(std_dev_value >= 0.0);
        }
    }

    #[test]
    fn test_std_dev_state_management() {
        let mut std_dev = StandardDeviation::with_period(3).unwrap();

        // Add some data
        for i in 1..=5 {
            let input = StandardDeviationInput { value: i as f64 };
            let _ = std_dev.calculate(input).unwrap();
        }

        // Get state
        let state = std_dev.get_state().clone();

        // Reset and verify
        std_dev.reset();
        assert!(!std_dev.get_state().has_sufficient_data);

        // Restore state
        std_dev.set_state(state);
        assert!(std_dev.get_state().has_sufficient_data);
    }
}
