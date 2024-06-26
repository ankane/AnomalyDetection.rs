# AnomalyDetection.rs

Time series [AnomalyDetection](https://github.com/twitter/AnomalyDetection) for Rust

Learn [how it works](https://blog.twitter.com/engineering/en_us/a/2015/introducing-practical-and-robust-anomaly-detection-in-a-time-series)

[![Build Status](https://github.com/ankane/AnomalyDetection.rs/actions/workflows/build.yml/badge.svg)](https://github.com/ankane/AnomalyDetection.rs/actions)

## Installation

Add this line to your application’s `Cargo.toml` under `[dependencies]`:

```toml
anomaly_detection = "0.3"
```

## Getting Started

Detect anomalies in a time series

```rust
use anomaly_detection::AnomalyDetector;

let series = vec![
    5.0, 9.0, 2.0, 9.0, 0.0, 6.0, 3.0, 8.0, 5.0, 18.0,
    7.0, 8.0, 8.0, 0.0, 2.0, 15.0, 0.0, 5.0, 6.0, 7.0,
    3.0, 6.0, 1.0, 4.0, 4.0, 4.0, 30.0, 7.0, 5.0, 8.0
];
let period = 7; // number of observations in a single period

let res = AnomalyDetector::fit(&series, period).unwrap();
```

Get anomalies

```rust
res.anomalies();
```

## Parameters

Set parameters

```rust
AnomalyDetector::params()
    .alpha(0.05)                    // level of statistical significance
    .max_anoms(0.1)                 // maximum number of anomalies as percent of data
    .direction(Direction::Both)     // Positive, Negative, or Both
    .verbose(false)                 // show progress
```

## Credits

This library was ported from the [AnomalyDetection](https://github.com/twitter/AnomalyDetection) R package and is available under the same license.

## References

- [Automatic Anomaly Detection in the Cloud Via Statistical Learning](https://arxiv.org/abs/1704.07706)

## History

View the [changelog](https://github.com/ankane/AnomalyDetection.rs/blob/master/CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/ankane/AnomalyDetection.rs/issues)
- Fix bugs and [submit pull requests](https://github.com/ankane/AnomalyDetection.rs/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/ankane/AnomalyDetection.rs.git
cd AnomalyDetection.rs
cargo test
```
