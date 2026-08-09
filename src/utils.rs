pub(crate) fn contains_non_finite(values: &[f64]) -> bool {
    values.iter().any(|value| !value.is_finite())
}

pub(crate) fn invalid_input(values: &[f64]) -> bool {
    values.is_empty() || contains_non_finite(values)
}

pub(crate) fn invalid_sample_input(values: &[f64]) -> bool {
    values.len() < 2 || contains_non_finite(values)
}

pub(crate) fn median_of_sorted(sorted: &[f64]) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }

    let n = sorted.len();
    let mid = n / 2;

    let median = if n % 2 == 0 {
        let left = sorted[mid - 1];
        let right = sorted[mid];

        (left + right) / 2.0
    } else {
        sorted[mid]
    };

    Some(median)
}

pub(crate) struct WelfordAccumulator {
    count: u64,
    mean: f64,
    m2: f64,
}

impl WelfordAccumulator {
    pub(crate) fn new() -> Self {
        WelfordAccumulator {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    pub(crate) fn push(&mut self, x: f64) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / (self.count as f64);
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }

    #[allow(dead_code)]
    pub(crate) fn mean(&self) -> f64 {
        self.mean
    }

    pub(crate) fn variance_population(&self) -> f64 {
        self.m2 / (self.count as f64)
    }

    pub(crate) fn variance_sample(&self) -> f64 {
        self.m2 / ((self.count as f64) - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_non_finite_returns_false_for_empty_input() {
        assert_eq!(contains_non_finite(&[]), false);
    }

    #[test]
    fn contains_non_finite_returns_false_for_finite_values() {
        assert!(!contains_non_finite(&[1.0, 2.0, 3.0]));
    }

    #[test]
    fn contains_non_finite_returns_true_for_nan() {
        assert_eq!(contains_non_finite(&[1.0, f64::NAN, 3.0]), true);
    }

    #[test]
    fn contains_non_finite_returns_true_for_infinity() {
        assert_eq!(contains_non_finite(&[1.0, f64::INFINITY, 3.0]), true);
    }

    #[test]
    fn invalid_input_returns_true_for_empty_input() {
        assert!(invalid_input(&[]));
    }

    #[test]
    fn invalid_input_returns_true_for_nan() {
        assert_eq!(invalid_input(&[1.0, f64::NAN]), true);
    }

    #[test]
    fn invalid_input_returns_false_for_valid_values() {
        assert_eq!(invalid_input(&[1.0, 2.0]), false);
    }

    #[test]
    fn invalid_sample_input_returns_true_for_one_input() {
        assert_eq!(invalid_sample_input(&[1.0]), true);
    }

    #[test]
    fn invalid_sample_input_returns_false_for_two_valid_values() {
        assert_eq!(invalid_sample_input(&[1.0, 2.0]), false);
    }

    #[test]
    fn test_welford_empty() {
        let acc = WelfordAccumulator::new();
        assert_eq!(acc.count, 0);
        assert_eq!(acc.mean(), 0.0);
    }

    #[test]
    fn test_welford_single_value() {
        let mut acc = WelfordAccumulator::new();
        acc.push(5.0);
        assert_eq!(acc.count, 1);
        assert_eq!(acc.mean(), 5.0);
        assert_eq!(acc.variance_population(), 0.0);
    }

    #[test]
    fn test_welford_identical_values() {
        let mut acc = WelfordAccumulator::new();
        for _ in 0..3 {
            acc.push(10.0);
        }
        assert_eq!(acc.mean(), 10.0);
        assert_eq!(acc.variance_population(), 0.0);
    }

    #[test]
    fn test_welford_known_values() {
        let mut acc = WelfordAccumulator::new();
        for v in [1.0, 2.0, 3.0, 4.0] {
            acc.push(v);
        }
        assert_eq!(acc.mean(), 2.5);
        assert_eq!(acc.variance_population(), 1.25);
    }

    #[test]
    fn test_welford_variance_sample() {
        let mut acc = WelfordAccumulator::new();
        for v in [1.0, 2.0, 3.0, 4.0, 5.0] {
            acc.push(v);
        }
        assert_eq!(acc.variance_sample(), 2.5);
    }

    #[test]
    fn test_welford_numerical_stability() {
        let base = 1e9;
        let mut acc = WelfordAccumulator::new();
        for v in [base + 1.0, base + 2.0, base + 3.0, base + 4.0] {
            acc.push(v);
        }
        let expected = 1.25;
        assert!((acc.variance_population() - expected).abs() < 1e-6);
    }
}
