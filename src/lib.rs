/*!
# indexes_rs Library

Welcome to the `indexes_rs` library! This library provides a suite of technical indicators for financial market analysis. All indicators and related types are grouped under the `v1` module, which represents version 1 of the library.

The indicators implemented in this library include:

- **ATR (Average True Range):** Measures market volatility.
- **Bollinger Bands:** Uses a simple moving average (SMA) and standard deviation to define upper and lower price bands.
- **EMA (Exponential Moving Average):** A weighted moving average that gives more importance to recent prices.
- **MA (Moving Averages):** A unified module that consolidates multiple moving averages (SMA, EMA, MACD).
- **MACD (Moving Average Convergence Divergence):** Combines fast and slow EMAs with a signal line to provide trend-following signals.
- **RSI (Relative Strength Index):** An oscillator used to identify overbought and oversold conditions.
- **SMA (Simple Moving Average):** Calculates the average of the close prices over a given period.
- **ROC (Rate of Change):** Measures the percentage change between the current and a past price.
- **Momentum:** Measures the difference between the current price and the price from a specified number of periods ago.
- **Stochastic Oscillator:** A momentum indicator comparing a particular closing price to a range of its prices over a certain period.
- **Support & Resistance:** Identifies potential support and resistance levels based on price swings.
- **Shared Types:** Common structures and enumerations used across multiple indicators.

Each module contains its own implementation (typically in a `main.rs` file) and associated tests (in a `__tests__.rs` or `_tests__` directory). For more details on each indicator, please refer to the documentation within the corresponding module.

*/

pub mod v1 {
    //! # Version 1
    //!
    //! This module groups all the technical indicators and shared types under version 1 of the
    //! `indexes_rs` library. The structure is as follows:
    //!
    //! - **atr:** Implements the Average True Range (ATR) indicator.
    //! - **bollinger:** Implements Bollinger Bands.
    //! - **ema:** Implements the Exponential Moving Average (EMA) indicator.
    //! - **ma:** Provides a unified interface for multiple moving averages (SMA, EMA, MACD).
    //! - **macd:** Implements the MACD (Moving Average Convergence Divergence) indicator.
    //! - **rsi:** Implements the Relative Strength Index (RSI) indicator.
    //! - **sma:** Implements the Simple Moving Average (SMA) indicator.
    //! - **roc:** Implements the Rate of Change (ROC) indicator.
    //! - **momentum:** Implements the Momentum indicator.
    //! - **stochastic:** Implements the Stochastic Oscillator indicator.
    //! - **support_resistance:** Implements Support & Resistance indicators.
    //! - **types:** Contains shared types (structs, enums) used throughout the library.

    pub mod atr {
        //! **ATR Module**
        //!
        //! Provides an implementation of the Average True Range (ATR) indicator.
        mod __tests__;
        pub mod main;
    }
    pub mod bollinger {
        //! **Bollinger Bands Module**
        //!
        //! Implements Bollinger Bands using a simple moving average (SMA) and standard deviation.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod ema {
        //! **EMA Module**
        //!
        //! Implements the Exponential Moving Average (EMA) indicator.
        mod __tests__;
        pub mod main;
    }
    pub mod ma {
        //! **Moving Averages Module**
        //!
        //! Provides a unified interface for multiple moving averages, including SMA, EMA, and MACD.
        mod __tests__;
        pub mod main;
    }
    pub mod macd {
        //! **MACD Module**
        //!
        //! Implements the Moving Average Convergence Divergence (MACD) indicator.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod rsi {
        //! **RSI Module**
        //!
        //! Implements the Relative Strength Index (RSI) indicator.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod sma {
        //! **SMA Module**
        //!
        //! Implements the Simple Moving Average (SMA) indicator.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod roc {
        //! **ROC Module**
        //!
        //! Implements the Rate of Change (ROC) indicator.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod momentum {
        //! **Momentum Module**
        //!
        //! Implements the Momentum indicator.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod stochastic {
        //! **Stochastic Oscillator Module**
        //!
        //! Implements the Stochastic Oscillator indicator.
        mod __tests__;
        pub mod main;
        pub mod types;
    }
    pub mod support_resistance {
        //! **Support & Resistance Module**
        //!
        //! Implements support and resistance indicators based on swing highs and lows.
        mod _tests__;
        pub mod main;
        pub mod types;
    }
    pub mod types {
        //! **Shared Types Module**
        //!
        //! Contains common types such as enums and structs used across multiple modules
        //! in the library.
    }
}
