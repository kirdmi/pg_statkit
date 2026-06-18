use core::option::Option::{self, None};

pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let sum: f64 = values.iter().sum();
    Some(sum / values.len() as f64)
}

pub fn sum(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let sum: f64 = values.iter().sum();
    Some(sum)
}

pub fn min(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut current_min = values[0];

    for value in values.iter().skip(1) {
        if *value < current_min {
            current_min = *value;
        }
    }

    Some(current_min)
}

pub fn max(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut current_max = values[0];

    for value in values.iter().skip(1) {
        if *value > current_max {
            current_max = *value;
        }
    }

    Some(current_max)
}

pub fn range(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let maximum = max(values)?;
    let minimum = min(values)?;

    Some(maximum - minimum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_returns_none_for_empty_input() {
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn mean_returns_average() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn sum_returns_none_for_empty_input() {
        assert_eq!(sum(&[]), None);
    }

    #[test]
    fn sum_returns_sum() {
        assert_eq!(sum(&[1.0, 2.0, 3.0]), Some(6.0));
    }

    #[test]
    fn min_returns_none_for_empty_input() {
        assert_eq!(min(&[]), None);
    }

    #[test]
    fn min_returns_minimum() {
        assert_eq!(min(&[3.0, 1.0, 2.0]), Some(1.0));
    }

    #[test]
    fn max_returns_none_for_empty_input() {
        assert_eq!(max(&[]), None);
    }

    #[test]
    fn max_returns_maximum() {
        assert_eq!(max(&[3.0, 1.0, 2.0]), Some(3.0));
    }

    #[test]
    fn range_returns_none_for_empty_input() {
        assert_eq!(range(&[]), None);
    }

    #[test]
    fn range_returns_range() {
        assert_eq!(range(&[3.0, 1.0, 2.0]), Some(2.0));
    }
}
