#[cfg(test)]
mod tests {
    use super::super::main::ATR;
    use super::super::types::{ATRError, OHLCData};

    const EPSILON: f64 = 0.0001;

    fn assert_close(a: f64, b: f64, epsilon: f64) {
        assert!((a - b).abs() < epsilon, "Values not close: {} vs {}, diff: {}", a, b, (a - b).abs());
    }

    #[test]
    fn test_new_atr() {
        let atr = ATR::new(14).unwrap();
        assert_eq!(atr.period(), 14);
        assert!(!atr.is_ready());
        assert_eq!(atr.value(), None);
    }

    #[test]
    fn test_invalid_period() {
        let result = ATR::new(0);
        assert!(matches!(result, Err(ATRError::InvalidPeriod)));
    }

    #[test]
    fn test_basic_calculation() {
        let mut atr = ATR::new(3).unwrap();

        // Sample OHLC data
        let data = [
            OHLCData::new(10.0, 8.0, 9.0),   // TR = 2.0
            OHLCData::new(11.0, 9.0, 10.0),  // TR = max(2.0, 2.0, 0.0) = 2.0
            OHLCData::new(12.0, 10.0, 11.0), // TR = max(2.0, 2.0, 0.0) = 2.0
            OHLCData::new(13.0, 10.5, 12.0), // TR = max(2.5, 2.0, 0.5) = 2.5
        ];

        // First 3 periods: building initial ATR
        assert!(atr.update(data[0]).is_none());
        assert!(atr.update(data[1]).is_none());

        let result = atr.update(data[2]).unwrap();
        assert_close(result.atr, 2.0, EPSILON); // (2.0 + 2.0 + 2.0) / 3
        assert!(atr.is_ready());

        // Fourth period: Wilder's smoothing
        let result = atr.update(data[3]).unwrap();
        // ATR = (2.0 * 2 + 2.5) / 3 = 6.5 / 3 = 2.1667
        assert_close(result.atr, 2.1667, EPSILON);
        assert_close(result.true_range, 2.5, EPSILON);
    }

    #[test]
    fn test_true_range_calculation() {
        let mut atr = ATR::new(14).unwrap();

        // First candle: TR = High - Low
        let result = atr.update(OHLCData::new(100.0, 95.0, 98.0));
        assert!(result.is_none()); // Not enough data yet

        // Second candle with gap up
        // Previous close: 98.0, Current: H=105, L=102, C=104
        // TR = max(105-102, |105-98|, |102-98|) = max(3, 7, 4) = 7
        atr.update(OHLCData::new(105.0, 102.0, 104.0));

        // Verify the true range is calculated correctly
        // (We need to wait for initialization to see the result)
    }

    #[test]
    fn test_gap_scenarios() {
        let mut atr = ATR::new(2).unwrap();

        // Gap up scenario
        atr.update(OHLCData::new(50.0, 48.0, 49.0));
        let result = atr.update(OHLCData::new(53.0, 51.0, 52.0));

        // TR for second candle: max(53-51=2, |53-49|=4, |51-49|=2) = 4
        // Initial ATR = (2 + 4) / 2 = 3
        assert!(result.is_some());
        assert_close(result.unwrap().atr, 3.0, EPSILON);

        // Gap down scenario
        let mut atr2 = ATR::new(2).unwrap();
        atr2.update(OHLCData::new(50.0, 48.0, 49.0));
        let result = atr2.update(OHLCData::new(47.0, 45.0, 46.0));

        // TR for second candle: max(47-45=2, |47-49|=2, |45-49|=4) = 4
        // Initial ATR = (2 + 4) / 2 = 3
        assert!(result.is_some());
        assert_close(result.unwrap().atr, 3.0, EPSILON);
    }

    #[test]
    fn test_invalid_data() {
        let mut atr = ATR::new(14).unwrap();

        // NaN values
        assert!(atr.update(OHLCData::new(f64::NAN, 10.0, 10.0)).is_none());
        assert!(atr.update(OHLCData::new(10.0, f64::NAN, 10.0)).is_none());
        assert!(atr.update(OHLCData::new(10.0, 10.0, f64::NAN)).is_none());

        // Infinite values
        assert!(atr.update(OHLCData::new(f64::INFINITY, 10.0, 10.0)).is_none());

        // Invalid OHLC (high < low)
        assert!(atr.update(OHLCData::new(10.0, 15.0, 12.0)).is_none());
    }

    #[test]
    fn test_close_only_fallback() {
        let mut atr = ATR::new(5).unwrap();

        // Using close-only method
        for i in 0..10 {
            let close = 100.0 + i as f64;
            atr.update_close_only(close);
        }

        assert!(atr.is_ready());
        assert!(atr.value().is_some());
    }

    #[test]
    fn test_reset() {
        let mut atr = ATR::new(3).unwrap();

        // Build up ATR
        atr.update(OHLCData::new(10.0, 8.0, 9.0));
        atr.update(OHLCData::new(11.0, 9.0, 10.0));
        atr.update(OHLCData::new(12.0, 10.0, 11.0));

        assert!(atr.is_ready());
        assert!(atr.value().is_some());

        // Reset
        atr.reset();
        assert!(!atr.is_ready());
        assert!(atr.value().is_none());
    }

    #[test]
    fn test_batch_calculation() {
        let data = vec![
            OHLCData::new(10.0, 8.0, 9.0),
            OHLCData::new(11.0, 9.0, 10.0),
            OHLCData::new(12.0, 10.0, 11.0),
            OHLCData::new(13.0, 10.5, 12.0),
            OHLCData::new(12.5, 11.0, 11.5),
        ];

        let results = ATR::calculate_batch(3, &data).unwrap();

        assert_eq!(results.len(), 5);
        assert!(results[0].is_none());
        assert!(results[1].is_none());
        assert!(results[2].is_some());
        assert!(results[3].is_some());
        assert!(results[4].is_some());
    }

    #[test]
    fn test_wilders_smoothing() {
        let mut atr = ATR::new(14).unwrap();

        // Generate data with CHANGING volatility to see smoothing effect
        let mut data = Vec::new();

        // First 14 candles with consistent range
        for i in 0..14 {
            let base = 100.0 + i as f64 * 0.1;
            data.push(OHLCData::new(base + 1.0, base - 1.0, base));
        }

        // Process initial period (all have TR = 2.0)
        for i in 0..14 {
            atr.update(data[i]);
        }

        assert!(atr.is_ready());
        let initial_atr = atr.value().unwrap();
        assert_close(initial_atr, 2.0, EPSILON); // Should be 2.0 (consistent TR of 2.0)

        // Add a candle with DIFFERENT volatility
        // This will have a different True Range
        let volatile_candle = OHLCData::new(115.0, 110.0, 112.0);
        let result = atr.update(volatile_candle).unwrap();

        // With Wilder's smoothing: new_atr = (prev_atr * 13 + new_tr) / 14
        // prev_atr = 2.0, new_tr should be around 5.0 (115-110)
        // Actually, TR = max(115-110=5, |115-101.3|=13.7, |110-101.3|=8.7) = 13.7
        // new_atr = (2.0 * 13 + 13.7) / 14 = 39.7 / 14 ≈ 2.84

        assert!(result.atr != initial_atr, "ATR should change with different volatility");
        assert!(result.atr > initial_atr, "ATR should increase with higher volatility");
    }

    #[test]
    fn test_wilders_smoothing_formula() {
        let mut atr = ATR::new(5).unwrap(); // Smaller period for easier calculation

        // Build initial ATR with consistent data
        for _ in 0..5 {
            atr.update(OHLCData::new(102.0, 98.0, 100.0)); // TR = 4.0
        }

        let initial_atr = atr.value().unwrap();
        assert_close(initial_atr, 4.0, EPSILON);

        // Add new data with different TR
        let result = atr.update(OHLCData::new(108.0, 98.0, 103.0)).unwrap();
        // TR = max(108-98=10, |108-100|=8, |98-100|=2) = 10

        // Wilder's formula: new_atr = (prev_atr * (n-1) + new_tr) / n
        // new_atr = (4.0 * 4 + 10.0) / 5 = 26.0 / 5 = 5.2
        assert_close(result.atr, 5.2, EPSILON);

        // Verify true range is reported correctly
        assert_close(result.true_range, 10.0, EPSILON);
    }

    #[test]
    fn test_smoothing_convergence() {
        let mut atr = ATR::new(14).unwrap();

        // Start with low volatility
        for _ in 0..14 {
            atr.update(OHLCData::new(101.0, 99.0, 100.0)); // TR = 2.0
        }

        let low_vol_atr = atr.value().unwrap();

        // Switch to high volatility and watch ATR gradually adjust
        let mut prev_atr = low_vol_atr;
        for i in 0..20 {
            atr.update(OHLCData::new(110.0, 90.0, 100.0)); // TR = 20.0
            let current_atr = atr.value().unwrap();

            if i > 0 {
                // ATR should be gradually increasing toward the new volatility level
                assert!(current_atr > prev_atr, "ATR should increase at iteration {i}");
            }
            prev_atr = current_atr;
        }

        // After many periods, ATR should be close to the new true range
        let final_atr = atr.value().unwrap();
        assert!(final_atr > 15.0, "ATR should converge toward high volatility");
        assert!(final_atr < 20.0, "ATR shouldn't fully reach new TR immediately due to smoothing");
    }

    #[test]
    fn test_real_world_scenario() {
        // Simulate real market data with volatility changes
        let mut atr = ATR::new(14).unwrap();

        // Low volatility period
        for i in 0..10 {
            let base = 100.0;
            atr.update(OHLCData::new(base + 0.5, base - 0.5, base + (i as f64 * 0.1)));
        }

        // High volatility period
        for i in 0..10 {
            let base = 100.0 + i as f64;
            atr.update(OHLCData::new(base + 3.0, base - 3.0, base));
        }

        assert!(atr.is_ready());

        // ATR should reflect increased volatility
        let final_atr = atr.value().unwrap();
        assert!(final_atr > 1.0); // Should be higher due to increased volatility
    }
}
