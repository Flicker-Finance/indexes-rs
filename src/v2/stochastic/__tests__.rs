#[cfg(test)]
mod tests {
    use super::super::main::StochasticOscillator;
    use super::super::types::{OHLCData, StochasticError};
    use crate::v2::stochastic::types::StochasticSignal;

    const EPSILON: f64 = 0.001;

    fn assert_close(a: f64, b: f64, epsilon: f64) {
        assert!((a - b).abs() < epsilon, "Values not close: {} vs {}, diff: {}", a, b, (a - b).abs());
    }

    #[test]
    fn test_new_stochastic() {
        let stoch = StochasticOscillator::new(14, 3, 3).unwrap();
        assert_eq!(stoch.period(), 14);
        assert!(!stoch.is_ready());
        assert_eq!(stoch.value(), None);
    }

    #[test]
    fn test_invalid_parameters() {
        assert!(matches!(StochasticOscillator::new(0, 3, 3), Err(StochasticError::InvalidPeriod)));
        assert!(matches!(StochasticOscillator::new(14, 0, 3), Err(StochasticError::InvalidSmoothingPeriod)));
        assert!(matches!(StochasticOscillator::new(14, 3, 0), Err(StochasticError::InvalidSmoothingPeriod)));
    }

    #[test]
    fn test_basic_calculation() {
        let mut stoch = StochasticOscillator::new(5, 1, 3).unwrap(); // No K smoothing for simplicity

        // Create data where we know the expected result
        let data = [
            OHLCData::new(10.0, 8.0, 9.0),
            OHLCData::new(11.0, 9.0, 10.0),
            OHLCData::new(12.0, 10.0, 11.0),
            OHLCData::new(13.0, 11.0, 12.0),
            OHLCData::new(14.0, 12.0, 13.0), // Highest: 14, Lowest: 8, Close: 13
        ];

        // Process first 4 - not enough data
        for i in 0..4 {
            assert!(stoch.update(data[i]).is_none());
        }

        // Fifth data point - should calculate
        let result = stoch.update(data[4]).unwrap();

        // %K = (13 - 8) / (14 - 8) * 100 = 5/6 * 100 = 83.33
        assert_close(result.k, 83.333, EPSILON);

        // %D is just %K for first value (only one K value)
        assert_close(result.d, 83.333, EPSILON);
    }

    #[test]
    fn test_smoothed_k() {
        let mut stoch = StochasticOscillator::new(3, 3, 3).unwrap();

        // Data designed for easy calculation
        let data = [
            OHLCData::new(10.0, 5.0, 7.0), // Raw K will be calculated from these
            OHLCData::new(12.0, 6.0, 9.0),
            OHLCData::new(15.0, 8.0, 12.0),  // H:15, L:5, C:12 -> Raw K = 70%
            OHLCData::new(14.0, 10.0, 11.0), // H:15, L:6, C:11 -> Raw K = 55.55%
            OHLCData::new(13.0, 9.0, 10.0),  // H:15, L:8, C:10 -> Raw K = 28.57%
        ];

        // First two updates - no result yet (need at least period=3)
        assert!(stoch.update(data[0]).is_none());
        assert!(stoch.update(data[1]).is_none());

        // Third update - period requirement met, but only 1 raw K value
        // Need k_smooth=3 raw K values for smoothing
        assert!(stoch.update(data[2]).is_none());

        // Fourth update - have 2 raw K values, still need 3 for smoothing
        assert!(stoch.update(data[3]).is_none());

        // Fifth update - finally have 3 raw K values, can calculate smoothed K
        let result = stoch.update(data[4]);
        assert!(result.is_some());

        // Verify the smoothed K calculation
        // Raw K values: 70%, 55.55%, 28.57%
        // Smoothed K = (70 + 55.55 + 28.57) / 3 ≈ 51.37%
        let values = result.unwrap();
        assert!((values.k - 51.37).abs() < 0.1);

        // D should equal K since we only have one K value so far
        assert!((values.d - values.k).abs() < 0.001);
    }

    #[test]
    fn test_overbought_oversold() {
        let mut stoch = StochasticOscillator::new(5, 3, 3).unwrap();

        // Create overbought scenario (prices near highs)
        for i in 0..10 {
            let base = 100.0 + i as f64;
            stoch.update(OHLCData::new(base + 1.0, base, base + 0.9));
        }

        assert!(stoch.value().unwrap().k > 80.0);

        // Create oversold scenario (prices near lows)
        stoch.reset();
        for i in 0..10 {
            let base = 100.0 - i as f64;
            stoch.update(OHLCData::new(base, base - 1.0, base - 0.9));
        }

        assert!(stoch.value().unwrap().k < 20.0);
    }

    #[test]
    fn test_crossover_signals() {
        let mut stoch = StochasticOscillator::new(14, 3, 3).unwrap();

        // Generate trending data
        for i in 0..20 {
            let base = 100.0 + i as f64 * 0.5;
            stoch.update(OHLCData::new(base + 1.0, base - 1.0, base));
        }

        let result = stoch.value().unwrap();
        let signal = stoch.signal().unwrap();

        // Check signal logic
        if result.k > result.d {
            assert!(matches!(signal, StochasticSignal::Bullish | StochasticSignal::Overbought));
        } else if result.k < result.d {
            assert!(matches!(signal, StochasticSignal::Bearish | StochasticSignal::Oversold));
        }
    }

    #[test]
    fn test_invalid_data() {
        let mut stoch = StochasticOscillator::default();

        // NaN values
        assert!(stoch.update(OHLCData::new(f64::NAN, 10.0, 10.0)).is_none());
        assert!(stoch.update(OHLCData::new(10.0, f64::NAN, 10.0)).is_none());
        assert!(stoch.update(OHLCData::new(10.0, 10.0, f64::NAN)).is_none());

        // Infinite values
        assert!(stoch.update(OHLCData::new(f64::INFINITY, 10.0, 10.0)).is_none());

        // Invalid OHLC (high < low)
        assert!(stoch.update(OHLCData::new(10.0, 15.0, 12.0)).is_none());

        // Close outside high-low range
        assert!(stoch.update(OHLCData::new(10.0, 5.0, 15.0)).is_none());
    }

    #[test]
    fn test_flat_market() {
        let mut stoch = StochasticOscillator::new(5, 1, 3).unwrap();

        // All prices the same (flat market)
        for _ in 0..10 {
            stoch.update(OHLCData::new(100.0, 100.0, 100.0));
        }

        let result = stoch.value().unwrap();
        // When high = low, stochastic should be 50%
        assert_close(result.k, 50.0, EPSILON);
    }

    #[test]
    fn test_reset() {
        let mut stoch = StochasticOscillator::new(5, 3, 3).unwrap();

        // Add some data
        for i in 0..10 {
            let base = 100.0 + i as f64;
            stoch.update(OHLCData::new(base + 1.0, base - 1.0, base));
        }

        assert!(stoch.is_ready());
        assert!(stoch.value().is_some());

        // Reset
        stoch.reset();
        assert!(!stoch.is_ready());
        assert!(stoch.value().is_none());
    }

    #[test]
    fn test_fast_slow_full() {
        // Fast Stochastic (no smoothing)
        let fast = StochasticOscillator::fast(14).unwrap();
        assert_eq!(fast.period(), 14);

        // Slow Stochastic (standard smoothing)
        let slow = StochasticOscillator::slow(14).unwrap();
        assert_eq!(slow.period(), 14);

        // Full Stochastic (custom smoothing)
        let full = StochasticOscillator::full(21, 5, 5).unwrap();
        assert_eq!(full.period(), 21);
    }

    #[test]
    fn test_batch_calculation() {
        let data = vec![
            OHLCData::new(10.0, 8.0, 9.0),
            OHLCData::new(11.0, 9.0, 10.0),
            OHLCData::new(12.0, 10.0, 11.0),
            OHLCData::new(13.0, 11.0, 12.0),
            OHLCData::new(14.0, 12.0, 13.0),
            OHLCData::new(13.5, 11.5, 12.5),
        ];

        let results = StochasticOscillator::calculate_batch(3, 1, 3, &data).unwrap();

        assert_eq!(results.len(), 6);
        assert!(results[0].is_none());
        assert!(results[1].is_none());
        assert!(results[2].is_some());
        assert!(results[3].is_some());
    }

    #[test]
    fn test_extreme_values() {
        let mut stoch = StochasticOscillator::new(5, 1, 3).unwrap();

        // Test with extreme high
        for i in 0..4 {
            let base = 100.0 + i as f64;
            stoch.update(OHLCData::new(base + 1.0, base, base + 0.5));
        }

        // Last one at the high
        let result = stoch.update(OHLCData::new(110.0, 100.0, 110.0)).unwrap();
        assert_close(result.k, 100.0, EPSILON); // Should be 100%

        // Test with extreme low
        stoch.reset();
        for i in 0..4 {
            let base = 100.0 + i as f64;
            stoch.update(OHLCData::new(base + 1.0, base, base + 0.5));
        }

        // Last one at the low
        let result = stoch.update(OHLCData::new(104.0, 95.0, 95.0)).unwrap();
        assert_close(result.k, 0.0, EPSILON); // Should be 0%
    }

    #[test]
    fn test_real_world_scenario() {
        let mut stoch = StochasticOscillator::default(); // 14, 3, 3

        // Simulate uptrend
        for i in 0..20 {
            let base = 100.0 + i as f64 * 2.0;
            let high = base + 3.0;
            let low = base - 1.0;
            let close = base + 2.0; // Closing near highs
            stoch.update(OHLCData::new(high, low, close));
        }

        let uptrend_result = stoch.value().unwrap();
        assert!(uptrend_result.k > 70.0, "Uptrend should show high stochastic");

        // Simulate downtrend
        for i in 0..20 {
            let base = 140.0 - i as f64 * 2.0;
            let high = base + 1.0;
            let low = base - 3.0;
            let close = base - 2.0; // Closing near lows
            stoch.update(OHLCData::new(high, low, close));
        }

        let downtrend_result = stoch.value().unwrap();
        assert!(downtrend_result.k < 30.0, "Downtrend should show low stochastic");
    }
}
