use crate::detect_anoms::detect_anoms;

pub enum Direction {
    Positive,
    Negative,
    Both,
}

// TODO remove in 0.2.0
impl Into<String> for Direction {
    fn into(self) -> String {
        match self {
            Direction::Positive => "pos".to_string(),
            Direction::Negative => "neg".to_string(),
            _ => "both".to_string(),
        }
    }
}

pub struct AnomalyDetectionParams {
    alpha: f32,
    max_anoms: f32,
    direction: Direction,
    verbose: bool
}

pub fn params() -> AnomalyDetectionParams {
    AnomalyDetectionParams {
        alpha: 0.05,
        max_anoms: 0.1,
        direction: Direction::Both,
        verbose: false
    }
}

pub struct AnomalyDetectionResult {
    anomalies: Vec<usize>,
}

impl AnomalyDetectionResult {
    pub fn anomalies(&self) -> &Vec<usize> {
        &self.anomalies
    }
}

impl AnomalyDetectionParams {
    pub fn alpha(&mut self, value: f32) -> &mut Self {
        self.alpha = value;
        self
    }

    pub fn max_anoms(&mut self, value: f32) -> &mut Self {
        self.max_anoms = value;
        self
    }

    // TODO only support enum in 0.2.0
    pub fn direction<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let direction = value.into();
        if direction == "pos".to_string() {
            self.direction = Direction::Positive;
        } else if direction == "neg".to_string() {
            self.direction = Direction::Negative;
        } else if direction == "both".to_string() {
            self.direction = Direction::Both;
        } else {
            panic!("direction must be pos, neg, or both")
        }
        self
    }

    pub fn verbose(&mut self, value: bool) -> &mut Self {
        self.verbose = value;
        self
    }

    // TODO return Result in 0.2.0
    pub fn fit(&self, series: &[f32], period: usize) -> AnomalyDetectionResult {
        let (one_tail, upper_tail) = match self.direction {
            Direction::Positive => (true, true),
            Direction::Negative => (true, false),
            Direction::Both => (false, true),
        };

        AnomalyDetectionResult {
            anomalies: detect_anoms(series, period, self.max_anoms, self.alpha, one_tail, upper_tail, self.verbose).unwrap_or_else(|e| panic!("{}", e)),
        }
    }
}
