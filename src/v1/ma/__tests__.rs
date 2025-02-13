//! Tests for the Moving Averages module.

#[cfg(test)]
mod tests {
    use crate::v1::ma::main::{EMAValues, MovingAverageResults, MovingAverages, SMAValues};

    /// Helper function to simulate price data.
    fn simulate_prices(ma: &mut MovingAverages, prices: &[f64]) -> MovingAverageResults {
        let mut last_result = None;
        for &price in prices {
            last_result = Some(ma.calculate(price));
        }
        last_result.unwrap()
    }

    #[test]
    fn test_moving_averages_initially_none() {
        let mut ma = MovingAverages::default();
        // With just one price, SMA should be None (not enough values for a full period)
        // while EMA should be seeded with the first value.
        let result = ma.calculate(100.0);
        let expected_sma = SMAValues {
            short: None,
            medium: None,
            long: None,
        };
        let expected_ema = EMAValues {
            short: Some(100.0),
            medium: Some(100.0),
            long: Some(100.0),
        };
        assert_eq!(result.sma, expected_sma);
        assert_eq!(result.ema, expected_ema);
        // MACD may or may not be available on the first price.
    }

    #[test]
    fn test_sma_and_ema_update() {
        let mut ma = MovingAverages::default();
        // Simulate a stream of prices sufficient for the short period SMA (20) and EMA.
        let prices: Vec<f64> = (1..=25).map(|x| x as f64).collect();
        let result = simulate_prices(&mut ma, &prices);

        // For SMA, since the window is of fixed length, we expect a valid value for short SMA.
        // Medium and long may still be None if there aren't enough data points.
        assert!(result.sma.short.is_some());
        assert!(result.sma.medium.is_none() || result.sma.medium.unwrap().value > 0.0);
        // EMA values should be available if the implementation initializes them after the first value.
        // (Depending on your EMA implementation, they might always be Some.)
        // Here we simply check that the short EMA is available.
        assert!(result.ema.short.is_some());
    }

    #[test]
    fn test_macd_calculation() {
        let mut ma = MovingAverages::default();
        // Simulate prices that should generate a MACD result.
        let prices = vec![10.0, 10.2, 10.4, 10.3, 10.5, 10.6, 10.8, 10.7, 10.9, 11.0, 10.8, 10.7];
        let result = simulate_prices(&mut ma, &prices);
        // The MACD result is optional. For a sufficient data stream, it should be Some.
        assert!(result.macd.is_some());
        // Optionally, perform further tests on the MACD values if needed.
        if let Some(macd_result) = result.macd {
            // For example, check that the MACD line is a valid number.
            assert!(macd_result.macd_line.is_finite());
        }
    }
}
