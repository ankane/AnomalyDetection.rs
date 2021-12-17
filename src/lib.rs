//! Anomaly detection for Rust
//!
//! [View the docs](https://github.com/ankane/AnomalyDetection.rs)

mod detect_anoms;
mod error;
mod params;
mod students_t;

pub use error::Error;
pub use params::{params, AnomalyDetectionParams, AnomalyDetectionResult, Direction};

#[cfg(test)]
mod tests {
    use crate::{Direction, Error};

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
        let res = crate::params().max_anoms(0.2).fit(&series, 7).unwrap();
        assert_eq!(&vec![9, 15, 26], res.anomalies());
    }

    #[test]
    fn test_direction_pos() {
        let series = generate_series();
        let res = crate::params()
            .max_anoms(0.2)
            .direction(Direction::Positive)
            .fit(&series, 7)
            .unwrap();
        assert_eq!(&vec![9, 26], res.anomalies());
    }

    #[test]
    fn test_direction_neg() {
        let series = generate_series();
        let res = crate::params()
            .max_anoms(0.2)
            .direction(Direction::Negative)
            .fit(&series, 7)
            .unwrap();
        assert_eq!(&vec![15], res.anomalies());
    }

    #[test]
    fn test_alpha() {
        let series = generate_series();
        let res = crate::params().max_anoms(0.2).alpha(0.5).fit(&series, 7).unwrap();
        assert_eq!(&vec![1, 4, 9, 15, 26], res.anomalies());
    }

    #[test]
    fn test_nan() {
        let mut series = vec![1.0; 30];
        series[15] = f32::NAN;
        let result = crate::params().fit(&series, 7);
        assert_eq!(
            result.unwrap_err(),
            Error::Series("series contains NANs".to_string())
        );
    }

    #[test]
    fn test_empty_data() {
        let series = Vec::new();
        let result = crate::params().fit(&series, 7);
        assert_eq!(
            result.unwrap_err(),
            Error::Series("series must contain at least 2 periods".to_string())
        );
    }

    #[test]
    fn test_max_anoms_zero() {
        let series = generate_series();
        let res = crate::params().max_anoms(0.0).fit(&series, 7).unwrap();
        assert!(res.anomalies().is_empty());
    }
}
