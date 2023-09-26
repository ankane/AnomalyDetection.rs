#[derive(Clone, Debug)]
pub struct AnomalyDetectionResult {
    pub(crate) anomalies: Vec<usize>,
}

impl AnomalyDetectionResult {
    pub fn anomalies(&self) -> &[usize] {
        &self.anomalies
    }
}
