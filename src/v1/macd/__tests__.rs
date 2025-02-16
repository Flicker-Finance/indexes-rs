#[cfg(test)]
mod tests {
    use crate::v1::{
        macd::{main::MACD, types::MACDResult},
        types::TradingSignal,
    };

    #[test]
    fn test_macd_initialization() {
        let macd = MACD::new(12, 26, 9);
        // Ensure that the MACD struct is created.
        assert!(macd.fast_ema.get_current_value().is_none());
        assert!(macd.slow_ema.get_current_value().is_none());
        assert!(macd.signal_ema.get_current_value().is_none());
    }

    #[test]
    fn test_macd_calculation() {
        let mut macd = MACD::new(12, 26, 9);
        // Provide a series of prices (this series may not yield precise known values,
        // but the calculation should eventually return Some(MACDResult)).
        let prices = vec![44.0, 44.5, 45.0, 44.8, 45.2, 45.5, 45.3, 45.4, 45.6, 45.7, 45.8, 46.0, 46.2];
        let mut result = None;
        for price in prices {
            result = macd.calculate(price);
        }
        // We expect the MACDResult to be available after a sufficient number of price updates.
        let res: MACDResult = result.unwrap();
        // Since exact values depend on the EMA initializations, we check that:
        // - The signal is one of the valid TradingSignal variants.
        // - The histogram, macd_line, and signal_line are finite numbers.
        assert!(res.macd_line.is_finite());
        assert!(res.signal_line.is_finite());
        assert!(res.histogram.is_finite());
        match res.signal {
            TradingSignal::Buy | TradingSignal::Sell | TradingSignal::Hold => {}
        }
    }

    #[test]
    fn test_determine_signal() {
        let macd_instance = MACD::new(12, 26, 9);
        // We test determine_signal directly:
        assert_eq!(macd_instance.determine_signal(1.0, 0.5), TradingSignal::Buy);
        assert_eq!(macd_instance.determine_signal(0.5, 1.0), TradingSignal::Sell);
        assert_eq!(macd_instance.determine_signal(0.7, 0.7), TradingSignal::Hold);
    }
}
