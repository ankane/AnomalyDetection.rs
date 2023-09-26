use super::{AnomalyDetectionParams, AnomalyDetectionResult, Error};

pub struct AnomalyDetector;

impl AnomalyDetector {
    /// Detects anomalies in a time series.
    pub fn fit(series: &[f32], period: usize) -> Result<AnomalyDetectionResult, Error> {
        AnomalyDetectionParams::new().fit(series, period)
    }

    /// Creates a new set of parameters.
    pub fn params() -> AnomalyDetectionParams {
        AnomalyDetectionParams::new()
    }
}
