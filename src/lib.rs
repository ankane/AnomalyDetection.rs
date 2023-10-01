#![doc = include_str!("../README.md")]

mod detect_anoms;
mod detector;
mod error;
mod params;
mod result;

pub use detector::AnomalyDetector;
pub use error::Error;
pub use params::{AnomalyDetectionParams, Direction};
pub use result::AnomalyDetectionResult;

/// Creates a new set of parameters.
pub fn params() -> AnomalyDetectionParams {
    AnomalyDetectionParams::new()
}
