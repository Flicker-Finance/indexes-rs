use serde::Serialize;

use crate::v1::{
    ema::main::ExponentialMovingAverage,
    macd::{main::MACD, types::MACDResult},
    sma::main::SimpleMovingAverage,
};

pub struct MovingAverages {
    pub sma: SMAPeriods,
    pub ema: EMAPeriods,
    pub macd: MACD,
}

pub struct SMAPeriods {
    pub short: SimpleMovingAverage,  // 20 periods
    pub medium: SimpleMovingAverage, // 50 periods
    pub long: SimpleMovingAverage,   // 200 periods
}

pub struct EMAPeriods {
    pub short: ExponentialMovingAverage,  // 20 periods
    pub medium: ExponentialMovingAverage, // 50 periods
    pub long: ExponentialMovingAverage,   // 200 periods
}

#[derive(Debug)]
pub struct MovingAverageResults {
    pub sma: SMAValues,
    pub ema: EMAValues,
    pub macd: Option<MACDResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SMAValues {
    pub short: Option<f64>,
    pub medium: Option<f64>,
    pub long: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EMAValues {
    pub short: Option<f64>,
    pub medium: Option<f64>,
    pub long: Option<f64>,
}

impl MovingAverages {
    pub fn new() -> Self {
        MovingAverages {
            sma: SMAPeriods::new(),
            ema: EMAPeriods::new(),
            macd: MACD::new(12, 26, 9),
        }
    }

    pub fn calculate(&mut self, price: f64) -> MovingAverageResults {
        self.sma.update(price);
        self.ema.update(price);

        MovingAverageResults {
            sma: self.sma.get_values(),
            ema: self.ema.get_values(),
            macd: self.macd.calculate(price),
        }
    }
}

impl SMAPeriods {
    fn new() -> Self {
        SMAPeriods {
            short: SimpleMovingAverage::new(20),
            medium: SimpleMovingAverage::new(50),
            long: SimpleMovingAverage::new(200),
        }
    }

    fn update(&mut self, price: f64) {
        self.short.add_value(price);
        self.medium.add_value(price);
        self.long.add_value(price);
    }

    fn get_values(&self) -> SMAValues {
        SMAValues {
            short: self.short.calculate(),
            medium: self.medium.calculate(),
            long: self.long.calculate(),
        }
    }
}

impl EMAPeriods {
    fn new() -> Self {
        EMAPeriods {
            short: ExponentialMovingAverage::new(20),
            medium: ExponentialMovingAverage::new(50),
            long: ExponentialMovingAverage::new(200),
        }
    }

    fn update(&mut self, price: f64) {
        self.short.add_value(price);
        self.medium.add_value(price);
        self.long.add_value(price);
    }

    fn get_values(&self) -> EMAValues {
        EMAValues {
            short: self.short.get_current_value(),
            medium: self.medium.get_current_value(),
            long: self.long.get_current_value(),
        }
    }
}
