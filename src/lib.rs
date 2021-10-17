use statrs::distribution::{ContinuousCDF, StudentsT};

#[derive(Debug)]
pub struct Error(pub(crate) String);

fn mad(data: &[f32]) -> f32 {
    let med = median(data);
    let res = data.iter().map(|v| (v - med).abs()).collect::<Vec<f32>>();
    1.4826 * median(&res)
}

fn median(data: &[f32]) -> f32 {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (sorted[(sorted.len() - 1) / 2] + sorted[sorted.len() / 2]) / 2.0
}

fn detect_anoms(data: &[f32], num_obs_per_period: usize, k: f32, alpha: f32, one_tail: bool, upper_tail: bool) -> Result<Vec<usize>, Error> {
    let num_obs = data.len();

    // Check to make sure we have at least two periods worth of data for anomaly context
    if num_obs < num_obs_per_period * 2 {
        return Err(Error("series must contain at least 2 periods".to_string()));
    }

    // Handle NAs
    if data.iter().any(|v| v.is_nan()) {
        return Err(Error("series contains NANs".to_string()));
    }

    // Decompose data. This returns a univarite remainder which will be used for anomaly detection. Optionally, we might NOT decompose.
    let data_decomp = stlrs::params().robust(true).seasonal_length(data.len() * 10 + 1).fit(&data, num_obs_per_period);

    let seasonal = data_decomp.seasonal();
    let mut data = data.to_vec();
    let med = median(&data);
    for i in 0..data.len() {
        data[i] -= seasonal[i] + med;
    }

    let max_outliers = (num_obs as f32 * k) as usize;
    let n = data.len();
    let mut r_idx = vec![];

    let mut indexes = Vec::with_capacity(data.len());
    for i in 0..data.len() {
        indexes.push(i);
    }

    // Compute test statistic until r=max_outliers values have been removed from the sample
    for i in 1..=max_outliers {
        // TODO Improve performance between loop iterations
        let ma = median(&data);
        let ares: Vec<f32>;
        if one_tail {
            if upper_tail {
                ares = data.iter().map(|v| v - ma).collect();
            } else {
                ares = data.iter().map(|v| ma - v).collect();
            }
        } else {
            ares = data.iter().map(|v| (v - ma).abs()).collect();
        }

        // Protect against constant time series
        let data_sigma = mad(&data);
        if data_sigma == 0.0 {
            break;
        }

        let (r_idx_i, r0) = ares.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap();
        let r_idx_i2 = indexes[r_idx_i];

        // Only need to take sigma of r for performance
        let r = *r0 / data_sigma;

        data.remove(r_idx_i);
        indexes.remove(r_idx_i);

        // Compute critical value
        let p;
        if one_tail {
            p = 1.0 - alpha / (n - i + 1) as f32;
        } else {
            p = 1.0 - alpha / (2.0 * (n - i + 1) as f32);
        }

        let t = StudentsT::new(0.0, 1.0, (n - i - 1) as f64).unwrap().inverse_cdf(p as f64) as f32;
        let lam = t * (n - i) as f32 / (((n - i - 1) as f32 + t.powf(2.0)) * (n - i + 1) as f32).sqrt();

        if r > lam {
            r_idx.push(r_idx_i2);
        } else {
            break;
        }
    }

    // Sort like R version
    r_idx.sort_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(r_idx)
}

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
}

pub fn params() -> AnomalyDetectionParams {
    AnomalyDetectionParams {
        alpha: 0.05,
        max_anoms: 0.1,
        direction: Direction::Both,
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

    // TODO return Result in 0.2.0
    pub fn fit(&self, series: &[f32], period: usize) -> AnomalyDetectionResult {
        let (one_tail, upper_tail) = match self.direction {
            Direction::Positive => (true, true),
            Direction::Negative => (true, false),
            Direction::Both => (false, true),
        };

        // TODO return Result
        AnomalyDetectionResult {
            anomalies: detect_anoms(series, period, self.max_anoms, self.alpha, one_tail, upper_tail).unwrap_or_else(|e| panic!("{}", e.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    fn generate_series() -> Vec<f32> {
        vec![
            5.0, 9.0, 2.0, 9.0, 0.0, 6.0, 3.0, 8.0, 5.0, 18.0,
            7.0, 8.0, 8.0, 0.0, 2.0, -5.0, 0.0, 5.0, 6.0, 7.0,
            3.0, 6.0, 1.0, 4.0, 4.0, 4.0, 30.0, 7.0, 5.0, 8.0
        ]
    }

    #[test]
    fn test_works() {
        let series = generate_series();
        let res = crate::params().max_anoms(0.2).fit(&series, 7);
        assert_eq!(&vec![9, 15, 26], res.anomalies());
    }

    #[test]
    fn test_direction_pos() {
        let series = generate_series();
        let res = crate::params()
            .max_anoms(0.2)
            .direction(Direction::Positive)
            .fit(&series, 7);
        assert_eq!(&vec![9, 26], res.anomalies());
    }

    #[test]
    fn test_direction_neg() {
        let series = generate_series();
        let res = crate::params()
            .max_anoms(0.2)
            .direction(Direction::Negative)
            .fit(&series, 7);
        assert_eq!(&vec![15], res.anomalies());
    }

    #[test]
    fn test_alpha() {
        let series = generate_series();
        let res = crate::params().max_anoms(0.2).alpha(0.5).fit(&series, 7);
        assert_eq!(&vec![1, 4, 9, 15, 26], res.anomalies());
    }

    #[test]
    #[should_panic(expected = "series contains NANs")]
    fn test_nan() {
        let mut series = vec![1.0; 30];
        series[15] = f32::NAN;
        crate::params().fit(&series, 7);
    }

    #[test]
    #[should_panic(expected = "series must contain at least 2 periods")]
    fn test_empty_data() {
        let series = Vec::new();
        crate::params().fit(&series, 7);
    }

    #[test]
    #[should_panic(expected = "direction must be pos, neg, or both")]
    fn test_direction_bad() {
        let series = generate_series();
        crate::params().direction("bad").fit(&series, 7);
    }

    #[test]
    fn test_max_anoms_zero() {
        let series = generate_series();
        let res = crate::params().max_anoms(0.0).fit(&series, 7);
        assert!(res.anomalies().is_empty());
    }
}
