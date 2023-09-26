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

#[cfg(test)]
mod tests {
    use crate::{AnomalyDetector, Direction, Error};

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
        let res = AnomalyDetector::params().max_anoms(0.2).fit(&series, 7).unwrap();
        assert_eq!(&vec![9, 15, 26], res.anomalies());
    }

    #[test]
    fn test_direction_pos() {
        let series = generate_series();
        let res = AnomalyDetector::params()
            .max_anoms(0.2)
            .direction(Direction::Positive)
            .fit(&series, 7)
            .unwrap();
        assert_eq!(&vec![9, 26], res.anomalies());
    }

    #[test]
    fn test_direction_neg() {
        let series = generate_series();
        let res = AnomalyDetector::params()
            .max_anoms(0.2)
            .direction(Direction::Negative)
            .fit(&series, 7)
            .unwrap();
        assert_eq!(&vec![15], res.anomalies());
    }

    #[test]
    fn test_alpha() {
        let series = generate_series();
        let res = AnomalyDetector::params().max_anoms(0.2).alpha(0.5).fit(&series, 7).unwrap();
        assert_eq!(&vec![1, 4, 9, 15, 26], res.anomalies());
    }

    #[test]
    fn test_nan() {
        let mut series = vec![1.0; 30];
        series[15] = f32::NAN;
        let result = AnomalyDetector::fit(&series, 7);
        assert_eq!(
            result.unwrap_err(),
            Error::Series("series contains NANs".to_string())
        );
    }

    #[test]
    fn test_empty_data() {
        let series = Vec::new();
        let result = AnomalyDetector::fit(&series, 7);
        assert_eq!(
            result.unwrap_err(),
            Error::Series("series must contain at least 2 periods".to_string())
        );
    }

    #[test]
    fn test_max_anoms_zero() {
        let series = generate_series();
        let res = AnomalyDetector::params().max_anoms(0.0).fit(&series, 7).unwrap();
        assert!(res.anomalies().is_empty());
    }
}
