#[cfg(test)]
mod tests {
    use crate::v1::ema::main::ExponentialMovingAverage;

    #[test]
    fn test_initial_value() {
        let mut ema = ExponentialMovingAverage::new(10);
        // First value should be equal to the price itself.
        assert_eq!(ema.add_value(100.0).unwrap(), 100.0);
    }

    #[test]
    fn test_ema_update() {
        let mut ema = ExponentialMovingAverage::new(10);
        // After first value:
        ema.add_value(100.0);
        // Calculate second EMA value.
        let updated = ema.add_value(110.0).unwrap();
        // For a period of 10, alpha = 2 / (10 + 1) ≈ 0.1818.
        // Expected EMA: 110 * 0.1818 + 100 * (0.8182) = 19.998 + 81.82 = 101.82 (approximately).
        assert!((updated - 101.82).abs() < 0.5);
    }

    #[test]
    fn test_get_current_value() {
        let mut ema = ExponentialMovingAverage::new(10);
        assert_eq!(ema.get_current_value(), None);
        ema.add_value(100.0);
        assert_eq!(ema.get_current_value().unwrap(), 100.0);
    }
}
