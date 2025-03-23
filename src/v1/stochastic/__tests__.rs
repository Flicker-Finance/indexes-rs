#[cfg(test)]
mod tests {
    use crate::v1::stochastic::{main::StochasticOscillator, types::StochResult};

    #[test]
    fn test_insufficient_data() {
        let mut stoch = StochasticOscillator::new(14, 3, 3);
        // Feed fewer than period values.
        for price in [100.0, 101.0, 102.0, 103.0] {
            assert!(stoch.calculate(price).is_none());
        }
    }

    #[test]
    fn test_stochastic_calculation() {
        let mut stoch = StochasticOscillator::new(14, 3, 3);
        // Feed in 14 prices so that we have enough data.
        let prices = vec![
            100.0, 102.0, 101.5, 103.0, 104.0, 102.5, 101.0, 100.5, 99.5, 98.0, 97.5, 98.5, 99.0,
            100.0,
        ];
        let mut result: Option<StochResult> = None;
        for price in prices {
            result = stoch.calculate(price);
        }
        assert!(result.is_some());
        let res = result.unwrap();
        // Check that %K and %D are within 0-100.
        assert!(res.k_value >= 0.0 && res.k_value <= 100.0);
        assert!(res.d_value >= 0.0 && res.d_value <= 100.0);
    }
}
