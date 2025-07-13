# indexes-rs

A Rust library providing a comprehensive collection of technical analysis indicators for financial market analysis(especially crypto market analysis).

[![Crates.io](https://img.shields.io/crates/v/indexes-rs.svg)](https://crates.io/crates/indexes-rs)
[![Documentation](https://docs.rs/indexes-rs/badge.svg)](https://docs.rs/indexes-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Developed and maintained by [Flicker](https://flicker.finance)

## Features

Currently supported technical indicators:
- ATR (Average True Range)
- Bollinger Bands
- EMA (Exponential Moving Average)
- MA (Moving Average)
- MACD (Moving Average Convergence Divergence)
- Momentum
- ROC (Rate of Change)
- RSI (Relative Strength Index)
- SMA (Simple Moving Average)
- Stochastic Oscillator
- Support and Resistance Levels
- OBV (On Balance Volume)
- MFI (Money Flow Index)
- Parabolic SAR (Stop and Reverse)
- ADX (Average Directional Index)
- CCI (Commodity Channel Index)
- Williams %R
- Standard Deviation

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
indexes-rs = "1.0.1"
```

Basic example:
```rust
use indexes_rs::v1::rsi::main::RSI;

fn main() {
    let prices = vec![10.0, 12.0, 11.0, 13.0, 15.0, 14.0];
    let rsi = RSI::new(14); // 14-period RSI
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

