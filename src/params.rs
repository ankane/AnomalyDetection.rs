use crate::detect_anoms::detect_anoms;
use crate::Error;

#[derive(Debug)]
pub enum Direction {
    Positive,
    Negative,
    Both,
}

#[derive(Debug)]
pub struct AnomalyDetectionParams {
    alpha: f32,
    max_anoms: f32,
    direction: Direction,
    verbose: bool
}

pub fn params() -> AnomalyDetectionParams {
    AnomalyDetectionParams::new()
}

#[derive(Debug)]
pub struct AnomalyDetectionResult {
    anomalies: Vec<usize>,
}

impl AnomalyDetectionResult {
    pub fn anomalies(&self) -> &[usize] {
        &self.anomalies
    }
}

impl AnomalyDetectionParams {
    pub fn new() -> Self {
        Self {
            alpha: 0.05,
            max_anoms: 0.1,
            direction: Direction::Both,
            verbose: false
        }
    }

    pub fn alpha(&mut self, value: f32) -> &mut Self {
        self.alpha = value;
        self
    }

    pub fn max_anoms(&mut self, value: f32) -> &mut Self {
        self.max_anoms = value;
        self
    }

    pub fn direction(&mut self, value: Direction) -> &mut Self {
        self.direction = value;
        self
    }

    pub fn verbose(&mut self, value: bool) -> &mut Self {
        self.verbose = value;
        self
    }

    pub fn fit(&self, series: &[f32], period: usize) -> Result<AnomalyDetectionResult, Error> {
        let (one_tail, upper_tail) = match self.direction {
            Direction::Positive => (true, true),
            Direction::Negative => (true, false),
            Direction::Both => (false, true),
        };

        Ok(AnomalyDetectionResult {
            anomalies: detect_anoms(series, period, self.max_anoms, self.alpha, one_tail, upper_tail, self.verbose)?,
        })
    }
}

impl Default for AnomalyDetectionParams {
    fn default() -> Self {
        Self::new()
    }
}
