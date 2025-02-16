#[cfg(test)]
mod tests {
    use crate::v1::momentum::{main::Momentum, types::MomentumResult};

    #[test]
    fn test_insufficient_data() {
        let mut momentum = Momentum::new(5);
        // Feed fewer than 5 prices.
        for price in [100.0, 101.0, 102.0, 103.0] {
            assert!(momentum.calculate(price).is_none());
        }
    }

    #[test]
    fn test_momentum_calculation() {
        let mut momentum = Momentum::new(3);
        // Provide exactly 3 prices.
        let prices = vec![100.0, 102.0, 104.0];
        let mut result = None;
        for price in prices {
            result = momentum.calculate(price);
        }
        let res: MomentumResult = result.unwrap();
        // Past price is 100.0, current price is 104.0:
        // Momentum = 104 - 100 = 4.0
        // Ratio = (104 / 100) * 100 = 104%
        assert!((res.value - 4.0).abs() < 1e-6);
        assert!((res.ratio - 104.0).abs() < 1e-6);
    }

    #[test]
    fn test_division_by_zero() {
        let mut momentum = Momentum::new(4);
        // Feed exactly 4 prices so the window is full and the first value (zero) remains.
        let prices = vec![0.0, 50.0, 100.0, 150.0];
        for price in &prices {
            // We call calculate for each price, but only the last call will produce a result.
            let _ = momentum.calculate(*price);
        }
        // At this point, the window is exactly 4 values and the first is 0.0.
        // The last call (with 150.0) should have returned None.
        // To avoid sliding the window further, we do not call calculate again.
        // Instead, we create a new instance with period=4, feed exactly 4 prices, and then check the result.
        let mut momentum = Momentum::new(4);
        for price in prices {
            // The final call should produce None due to division by zero.
            if let Some(result) = momentum.calculate(price) {
                panic!("Expected None due to division by zero, but got {:?}", result);
            }
        }
    }
}
