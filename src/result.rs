/// An anomaly detection result.
#[derive(Clone, Debug)]
pub struct AnomalyDetectionResult {
    pub(crate) anomalies: Vec<usize>,
}

impl AnomalyDetectionResult {
    /// Returns the anomalies.
    pub fn anomalies(&self) -> &[usize] {
        &self.anomalies
    }
}
