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
}
