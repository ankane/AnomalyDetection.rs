//! Anomaly detection for Rust
//!
//! [View the docs](https://github.com/ankane/AnomalyDetection.rs)

mod detect_anoms;
mod detector;
mod error;
mod params;
mod result;

pub use detector::AnomalyDetector;
pub use error::Error;
pub use params::{AnomalyDetectionParams, Direction};
pub use result::AnomalyDetectionResult;

pub fn params() -> AnomalyDetectionParams {
    AnomalyDetectionParams::new()
}
