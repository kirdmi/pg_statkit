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
}
