use super::Error;
use distrs::StudentsT;
use stlrs::Stl;

fn mad(data: &[f32], med: f32) -> f32 {
    let mut res = data.iter().map(|v| (v - med).abs()).collect::<Vec<f32>>();
    res.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    1.4826 * median_sorted(&res)
}

fn median(data: &[f32]) -> f32 {
    let mut sorted = data.to_vec();
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    median_sorted(&sorted)
}

fn median_sorted(sorted: &[f32]) -> f32 {
    (sorted[(sorted.len() - 1) / 2] + sorted[sorted.len() / 2]) / 2.0
}

pub fn detect_anoms(data: &[f32], num_obs_per_period: usize, k: f32, alpha: f32, one_tail: bool, upper_tail: bool, verbose: bool) -> Result<Vec<usize>, Error> {
    let n = data.len();

    // Check to make sure we have at least two periods worth of data for anomaly context
    if n < num_obs_per_period * 2 {
        return Err(Error::Series("series must contain at least 2 periods".to_string()));
    }

    // Handle NAs
    if data.iter().any(|v| v.is_nan()) {
        return Err(Error::Series("series contains NANs".to_string()));
    }

    // Decompose data. This returns a univarite remainder which will be used for anomaly detection. Optionally, we might NOT decompose.
    let data_decomp = Stl::params().robust(true).seasonal_length(data.len() * 10 + 1).fit(data, num_obs_per_period).unwrap();
    let seasonal = data_decomp.seasonal();

    // Copy data since we need to modify it
    let mut data = data.to_vec();
    let med = median(&data);
    for i in 0..n {
        data[i] -= seasonal[i] + med;
    }

    let mut num_anoms = 0;
    let max_outliers = (n as f32 * k) as usize;
    let mut anomalies = Vec::with_capacity(max_outliers);

    // Sort data for fast median
    // Use stable sort for indexes for deterministic results
    let mut indexes = (0..n).collect::<Vec<usize>>();
    indexes.sort_by(|a, b| data[*a].partial_cmp(&data[*b]).unwrap());
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    // Compute test statistic until r=max_outliers values have been removed from the sample
    for i in 1..=max_outliers {
        if verbose {
            println!("{} / {} completed", i, max_outliers);
        }

        let ma = median_sorted(&data);
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

        let (idx, r0) = ares.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).unwrap();

        // Only need to take sigma of r for performance
        let r = *r0 / data_sigma;

        anomalies.push(indexes[idx]);
        data.remove(idx);
        indexes.remove(idx);

        // Compute critical value
        let p = if one_tail {
            1.0 - alpha / (n - i + 1) as f32
        } else {
            1.0 - alpha / (2.0 * (n - i + 1) as f32)
        };

        let t = StudentsT::ppf(p as f64, (n - i - 1) as u32) as f32;
        let lam = t * (n - i) as f32 / (((n - i - 1) as f32 + t * t) * (n - i + 1) as f32).sqrt();

        if r > lam {
            num_anoms = i;
        }
    }

    anomalies.truncate(num_anoms);

    // Sort like R version
    anomalies.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    Ok(anomalies)
}
