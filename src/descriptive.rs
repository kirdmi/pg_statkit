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

pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

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

fn sum_squared_deviations(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mean_value = mean(values)?;

    let mut sum_squared_deviations: f64 = 0.0;

    for &value in values.iter() {
        let deviation: f64 = value - mean_value;
        sum_squared_deviations += deviation * deviation;
    }

    Some(sum_squared_deviations)
}

pub fn variance_population(values: &[f64]) -> Option<f64> {
    let len: f64 = values.len() as f64;
    let ssd = sum_squared_deviations(values)?;

    Some(ssd / len)
}

pub fn variance_sample(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }

    let len: f64 = values.len() as f64;

    let ssd = sum_squared_deviations(values)?;

    Some(ssd / (len - 1.0))
}

pub fn stddev_population(values: &[f64]) -> Option<f64> {
    let variance = variance_population(values)?;

    Some(variance.sqrt())
}

pub fn stddev_sample(values: &[f64]) -> Option<f64> {
    let variance = variance_sample(values)?;

    Some(variance.sqrt())
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

    #[test]
    fn median_returns_none_for_empty_input() {
        assert_eq!(median(&[]), None);
    }

    #[test]
    fn median_returns_median_odd() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
    }

    #[test]
    fn median_returns_median_even() {
        assert_eq!(median(&[3.0, 1.0, 2.0, 4.0]), Some(2.5));
    }

    #[test]
    fn variance_population_returns_none_for_empty_input() {
        assert_eq!(variance_population(&[]), None);
    }

    #[test]
    fn variance_population_returns_zero_for_one_input() {
        assert_eq!(variance_population(&[5.0]), Some(0.0));
    }

    #[test]
    fn variance_population_returns_variance_for_list_of_different_elements() {
        assert_eq!(variance_population(&[1.0, 2.0, 3.0, 4.0]), Some(1.25));
    }

    #[test]
    fn variance_population_returns_variance_for_list_of_same_elements() {
        assert_eq!(variance_population(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn variance_sample_returns_none_for_empty_input() {
        assert_eq!(variance_sample(&[]), None);
    }

    #[test]
    fn variance_sample_returns_none_for_one_input() {
        assert_eq!(variance_sample(&[5.0]), None);
    }

    #[test]
    fn variance_sample_returns_variance_for_list_of_different_elements() {
        assert_eq!(variance_sample(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(2.5));
    }

    #[test]
    fn variance_sample_returns_variance_for_list_of_same_elements() {
        assert_eq!(variance_sample(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn stddev_population_returns_none_for_empty_input() {
        assert_eq!(stddev_population(&[]), None);
    }

    #[test]
    fn stddev_population_returns_zero_for_one_input() {
        assert_eq!(stddev_population(&[5.0]), Some(0.0));
    }

    #[test]
    fn stddev_population_returns_zero_for_same_elements() {
        assert_eq!(stddev_population(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn stddev_population_returns_stddev_for_different_elements() {
        assert_eq!(stddev_population(&[1.0, 1.0, 3.0, 3.0]), Some(1.0));
    }

    #[test]
    fn stddev_sample_returns_none_for_empty_input() {
        assert_eq!(stddev_sample(&[]), None);
    }

    #[test]
    fn stddev_sample_returns_none_for_one_input() {
        assert_eq!(stddev_sample(&[5.0]), None);
    }

    #[test]
    fn stddev_sample_returns_zero_for_same_elements() {
        assert_eq!(stddev_sample(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn stddev_sample_returns_stddev_for_different_elements() {
        let expected = (4.0_f64 / 3.0).sqrt();

        assert_eq!(stddev_sample(&[1.0, 1.0, 3.0, 3.0]), Some(expected));
    }
}
