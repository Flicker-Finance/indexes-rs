#[cfg(test)]
mod tests {
    use crate::v1::support_resistance::main::SupportResistance;

    #[test]
    fn test_insufficient_prices() {
        let mut sr = SupportResistance::new(SupportResistance::DEFAULT_PERIOD, SupportResistance::DEFAULT_THRESHOLD);
        // With fewer than DEFAULT_PERIOD prices, calculate() should return None.
        for price in [100.0; 10] {
            assert_eq!(sr.calculate(price), None);
        }
    }

    #[test]
    fn test_support_resistance_calculation() {
        let mut sr = SupportResistance::new(5, 0.02);
        let prices = vec![100.0, 102.0, 101.0, 103.0, 104.0, 102.0, 101.0, 100.0, 99.0, 98.0];
        let mut last_result = None;
        for price in prices {
            last_result = sr.calculate(price);
        }
        let result = last_result.unwrap();
        // Here, you can assert properties of the result. For example:
        // Check that support is below current price and resistance above current price.
        if let Some(support) = result.nearest_support {
            assert!(support < 98.0 || support < 100.0);
        }
        if let Some(resistance) = result.nearest_resistance {
            assert!(resistance > 102.0 || resistance > 104.0);
        }
        // Further assertions can be added based on your expected behavior.
    }
}
