use std::f64::consts::PI;

// Hill, G. W. (1970).
// Algorithm 396: Student's t-Quantiles.
// Communications of the ACM, 13(10), 619-620.
pub fn inverse_cdf(p: f64, n: u64) -> f64 {
    assert!(p >= 0.0 && p <= 1.0);
    assert!(n >= 1);

    // distribution is symmetric
    let (sign, p) = if p < 0.5 {
        (-1.0, 1.0 - p)
    } else {
        (1.0, p)
    };

    // two-tail to one-tail
    let p = 2.0 * (1.0 - p);

    if n == 2 {
        return sign * (2.0 / (p * (2.0 - p)) - 2.0).sqrt();
    }

    let half_pi = PI / 2.0;

    if n == 1 {
        let p = p * half_pi;
        return sign * p.cos() / p.sin();
    }

    let ni = n;
    let n = n as f64;

    let a = 1.0 / (n - 0.5);
    let b = 48.0 / (a * a);
    let mut c = ((20700.0 * a / b - 98.0) * a - 16.0) * a + 96.36;
    let d = ((94.5 / (b + c) - 3.0) / b + 1.0) * (a * half_pi).sqrt() * n;
    let mut x = d * p;
    let mut y = x.powf(2.0 / n);
    if y > 0.05 + a {
        x = normdev(p * 0.5);
        y = x * x;
        if ni < 5 {
            c += 0.3 * (n - 4.5) * (x + 0.6);
        }
        c = (((0.05 * d * x - 5.0) * x - 7.0) * x - 2.0) * x + b + c;
        y = (((((0.4 * y + 6.3) * y + 36.0) * y + 94.5) / c - y - 3.0) / b + 1.0) * x;
        y = a * y * y;
        y = if y > 0.002 { y.exp() - 1.0 } else { 0.5 * y * y + y };
    } else {
        y = ((1.0 / (((n + 6.0) / (n * y) - 0.089 * d - 0.822) * (n + 2.0) * 3.0) + 0.5 / (n + 4.0)) * y - 1.0) * (n + 1.0) / (n + 2.0) + 1.0 / y;
    }
    sign * (n * y).sqrt()
}

// always negative since below 0.5
fn normdev(p: f64) -> f64 {
    assert!(p <= 0.5);

    let p = 1.0 - p;
    -2.0_f64.sqrt() * inverse_erf(2.0 * p - 1.0)
}

// Winitzki, S. (2008).
// A handy approximation for the error function and its inverse.
// https://drive.google.com/file/d/0B2Mt7luZYBrwZlctV3A3eF82VGM/view?resourcekey=0-UQpPhwZgzP0sF4LHBDlLtg
// from https://sites.google.com/site/winitzki
fn inverse_erf(x: f64) -> f64 {
    assert!(x >= 0.0 && x <= 1.0);

    let a = 0.147;
    let ln = (1.0 - x * x).ln();
    let f1 = 2.0 / (PI * a);
    let f2 = ln / 2.0;
    let f3 = 1.0 / a * ln;
    (-f1 - f2 + ((f1 + f2).powf(2.0) - f3).sqrt()).sqrt()
}

#[cfg(test)]
mod tests {
    use super::inverse_cdf;
    use std::f64::{NEG_INFINITY, INFINITY};

    fn assert_in_delta(act: f64, exp: f64) {
        if exp.is_finite() {
            assert!((exp - act).abs() < 0.0002, "{} != {}", act, exp);
        } else {
            assert_eq!(act, exp);
        }
    }

    #[test]
    fn test_one() {
        let expected = [NEG_INFINITY, -3.07768, -1.37638, -0.72654, -0.32492, 0.0, 0.32492, 0.72654, 1.37638, 3.07768, INFINITY];
        for i in 0..=10 {
            assert_in_delta(inverse_cdf(0.1 * i as f64, 1), expected[i]);
        }
    }

    #[test]
    fn test_two() {
        let expected = [NEG_INFINITY, -1.88562, -1.06066, -0.61721, -0.28868, 0.0, 0.28868, 0.61721, 1.06066, 1.88562, INFINITY];
        for i in 0..=10 {
            assert_in_delta(inverse_cdf(0.1 * i as f64, 2), expected[i]);
        }
    }

    #[test]
    fn test_thirty() {
        let expected = [NEG_INFINITY, -1.31042, -0.85377, -0.53002, -0.25561, 0.0, 0.25561, 0.53002, 0.85377, 1.31042, INFINITY];
        for i in 0..=10 {
            assert_in_delta(inverse_cdf(0.1 * i as f64, 30), expected[i]);
        }
    }
}
