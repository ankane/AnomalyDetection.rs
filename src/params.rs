use super::detect_anoms::detect_anoms;
use super::result::AnomalyDetectionResult;
use super::Error;

#[derive(Clone, Debug)]
pub enum Direction {
    Positive,
    Negative,
    Both,
}

#[derive(Clone, Debug)]
pub struct AnomalyDetectionParams {
    alpha: f32,
    max_anoms: f32,
    direction: Direction,
    verbose: bool,
}

impl AnomalyDetectionParams {
    /// Creates a new set of parameters.
    pub fn new() -> Self {
        Self {
            alpha: 0.05,
            max_anoms: 0.1,
            direction: Direction::Both,
            verbose: false,
        }
    }

    /// Sets the level of statistical significance.
    pub fn alpha(&mut self, value: f32) -> &mut Self {
        self.alpha = value;
        self
    }

    /// Sets the maximum number of anomalies as percent of data.
    pub fn max_anoms(&mut self, value: f32) -> &mut Self {
        self.max_anoms = value;
        self
    }

    /// Sets the direction.
    pub fn direction(&mut self, value: Direction) -> &mut Self {
        self.direction = value;
        self
    }

    /// Sets whether to show progress.
    pub fn verbose(&mut self, value: bool) -> &mut Self {
        self.verbose = value;
        self
    }

    /// Detects anomalies in a time series.
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
