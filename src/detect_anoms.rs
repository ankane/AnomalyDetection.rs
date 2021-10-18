use crate::Error;
use statrs::distribution::{ContinuousCDF, StudentsT};

fn mad(data: &[f32], med: f32) -> f32 {
    let mut res = data.iter().map(|v| (v - med).abs()).collect::<Vec<f32>>();
    res.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    1.4826 * median(&res)
}

fn median(sorted: &[f32]) -> f32 {
    (sorted[(sorted.len() - 1) / 2] + sorted[sorted.len() / 2]) / 2.0
}

fn sort_with_index(input: &[f32]) -> (Vec<f32>, Vec<usize>) {
    let n = input.len();
    let mut combined = Vec::with_capacity(n);
    for i in 0..n {
        combined.push((i, input[i]));
    }
    combined.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    let data = combined.iter().map(|(_, v)| *v).collect::<Vec<f32>>();
    let indexes = combined.iter().map(|(k, _)| *k).collect::<Vec<usize>>();
    (data, indexes)
}

pub fn detect_anoms(data: &[f32], num_obs_per_period: usize, k: f32, alpha: f32, one_tail: bool, upper_tail: bool) -> Result<Vec<usize>, Error> {
    let num_obs = data.len();

    // Check to make sure we have at least two periods worth of data for anomaly context
    if num_obs < num_obs_per_period * 2 {
        return Err(Error::Series("series must contain at least 2 periods".to_string()));
    }

    // Handle NAs
    if data.iter().any(|v| v.is_nan()) {
        return Err(Error::Series("series contains NANs".to_string()));
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
    let mut anomalies = Vec::new();

    let (mut data, mut indexes) = sort_with_index(&data);

    // Compute test statistic until r=max_outliers values have been removed from the sample
    for i in 1..=max_outliers {
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
        let data_sigma = mad(&data, ma);
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
        let p = if one_tail {
            1.0 - alpha / (n - i + 1) as f32
        } else {
            1.0 - alpha / (2.0 * (n - i + 1) as f32)
        };

        let t = StudentsT::new(0.0, 1.0, (n - i - 1) as f64).unwrap().inverse_cdf(p as f64) as f32;
        let lam = t * (n - i) as f32 / (((n - i - 1) as f32 + t.powf(2.0)) * (n - i + 1) as f32).sqrt();

        if r > lam {
            anomalies.push(r_idx_i2);
        } else {
            break;
        }
    }

    // Sort like R version
    anomalies.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(anomalies)
}
