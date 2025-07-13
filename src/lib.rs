/*!
# indexes_rs Library

Welcome to the `indexes_rs` library! This library provides a suite of technical indicators for financial market analysis. All indicators and related types are grouped under the `v1` and `v2` modules.

## Version 1 (v1)
The original collection of fundamental technical indicators including RSI, MACD, Bollinger Bands, and other essential indicators for technical analysis.

## Version 2 (v2)
An advanced collection of technical indicators specifically designed for cryptocurrency trading algorithms and sophisticated signal aggregation systems. This version focuses on volume-based indicators, trend strength measurement, and momentum analysis optimized for crypto market volatility.

The v2 indicators implemented in this library include:

- **OBV (On Balance Volume):** Volume-based momentum indicator that tracks cumulative volume flow.
- **MFI (Money Flow Index):** Volume-weighted RSI that combines price and volume for overbought/oversold analysis.
- **Parabolic SAR:** Trend-following indicator providing dynamic stop-loss levels and reversal signals.
- **ADX (Average Directional Index):** Measures trend strength regardless of direction, essential for trend filtering.
- **CCI (Commodity Channel Index):** Momentum oscillator for identifying cyclical trends and extreme conditions.
- **Williams %R:** Momentum indicator for detecting extreme overbought/oversold conditions.
- **Standard Deviation:** Mathematical foundation for volatility measurement and statistical analysis.

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

    pub mod types;
}

pub mod v2 {
    //! # Version 2 - Advanced Crypto Trading Indicators
    //!
    //! This module contains advanced technical indicators specifically designed for cryptocurrency
    //! trading algorithms and sophisticated signal aggregation systems. These indicators focus on
    //! volume analysis, trend strength measurement, and momentum detection optimized for the high
    //! volatility environment of cryptocurrency markets.
    //!
    //! ## Tier 1 Indicators (Essential)
    //! - **obv:** On Balance Volume - Critical volume-based momentum indicator
    //! - **mfi:** Money Flow Index - Volume-weighted RSI for comprehensive market analysis
    //!
    //! ## Tier 2 Indicators (Highly Recommended)
    //! - **parabolic_sar:** Parabolic SAR - Dynamic trend-following with 95% confidence level
    //! - **adx:** Average Directional Index - Superior trend strength filtering
    //! - **cci:** Commodity Channel Index - Effective momentum measurement for cyclical analysis
    //!
    //! ## Tier 3 Indicators (Supplementary)
    //! - **williams_r:** Williams %R - Extreme condition detection for overbought/oversold analysis
    //!
    //! ## Mathematical Foundation
    //! - **std_dev:** Standard Deviation - Essential statistical foundation for volatility analysis
    //!
    //! All indicators are designed to work together in signal aggregation systems and include
    //! comprehensive error handling, state management, and batch processing capabilities.

    /// **OBV Module**
    ///
    /// On Balance Volume (OBV) is a volume-based momentum indicator that tracks cumulative
    /// volume flow to predict price movements. Essential for crypto signal validation.
    pub mod obv {
        mod __tests__;
        pub mod main;
        pub mod types;
    }

    /// **MFI Module**
    ///
    /// Money Flow Index (MFI) combines price and volume data to create a volume-weighted
    /// version of RSI. Critical for identifying overbought/oversold conditions in crypto markets.
    pub mod mfi {
        mod __tests__;
        pub mod main;
        pub mod types;
    }

    /// **Parabolic SAR Module**
    ///
    /// Parabolic SAR (Stop and Reverse) provides dynamic trend-following signals and stop-loss
    /// levels. Achieves 95% confidence level in crypto applications according to analysis.
    pub mod parabolic_sar {
        mod __tests__;
        pub mod main;
        pub mod types;
    }

    /// **ADX Module**
    ///
    /// Average Directional Index measures trend strength regardless of direction.
    /// Superior trend strength filtering essential for crypto trading algorithms.
    pub mod adx {
        mod __tests__;
        pub mod main;
        pub mod types;
    }

    /// **CCI Module**
    ///
    /// Commodity Channel Index is a momentum oscillator that identifies cyclical trends
    /// and extreme market conditions. Effective for crypto's cyclical price patterns.
    pub mod cci {
        mod __tests__;
        pub mod main;
        pub mod types;
    }

    /// **Williams %R Module**
    ///
    /// Williams %R is a momentum indicator specialized for detecting extreme overbought
    /// and oversold conditions. Particularly effective in crypto's volatile environment.
    pub mod williams_r {
        mod __tests__;
        pub mod main;
        pub mod types;
    }

    /// **Standard Deviation Module**
    ///
    /// Standard Deviation provides the mathematical foundation for volatility measurement
    /// and statistical analysis. Essential for Bollinger Bands and risk assessment.
    pub mod std_dev {
        mod __tests__;
        pub mod main;
        pub mod types;
    }
}
