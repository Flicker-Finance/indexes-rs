use super::types::*;
use crate::v1::{ema::main::ExponentialMovingAverage, types::TradingSignal};

pub struct MACD {
    fast_ema: ExponentialMovingAverage,
    slow_ema: ExponentialMovingAverage,
    signal_ema: ExponentialMovingAverage,
    histogram: Vec<f64>,
}

impl MACD {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        MACD {
            fast_ema: ExponentialMovingAverage::new(fast_period),
            slow_ema: ExponentialMovingAverage::new(slow_period),
            signal_ema: ExponentialMovingAverage::new(signal_period),
            histogram: Vec::new(),
        }
    }

    pub fn calculate(&mut self, price: f64) -> Option<MACDResult> {
        let fast = self.fast_ema.add_value(price)?;
        let slow = self.slow_ema.add_value(price)?;
        let macd_line = fast - slow;
        let signal_line = self.signal_ema.add_value(macd_line)?;
        let histogram = macd_line - signal_line;

        self.histogram.push(histogram);

        Some(MACDResult {
            macd_line,
            signal_line,
            histogram,
            signal: self.determine_signal(macd_line, signal_line),
        })
    }

    pub fn determine_signal(&self, macd: f64, signal: f64) -> TradingSignal {
        if macd > signal {
            TradingSignal::Buy
        } else if macd < signal {
            TradingSignal::Sell
        } else {
            TradingSignal::Hold
        }
    }
}
