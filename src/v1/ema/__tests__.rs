#[cfg(test)]
mod tests {
    use crate::v1::ema::main::ExponentialMovingAverage;
    use approx::assert_relative_eq;

    #[test]
    fn test_ema_calculation() {
        let mut ema = ExponentialMovingAverage::new(3);

        assert_eq!(ema.add_value(2.0), Some(2.0));
        assert_relative_eq!(ema.add_value(4.0).unwrap(), 3.0, epsilon = 0.0001);
        assert_relative_eq!(ema.add_value(6.0).unwrap(), 4.5, epsilon = 0.0001);
    }

    #[test]
    fn test_alpha_calculation() {
        let ema = ExponentialMovingAverage::new(10);
        assert_relative_eq!(ema.alpha, 0.1818, epsilon = 0.0001);
    }
}
