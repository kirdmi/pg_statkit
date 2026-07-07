// helpers

use core::option::Option;

fn sum_squared_deviations(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
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

fn contains_non_finite(values: &[f64]) -> bool {
    values.iter().any(|value| !value.is_finite())
}

fn invalid_input(values: &[f64]) -> bool {
    values.is_empty() || contains_non_finite(values)
}

fn invalid_sample_input(values: &[f64]) -> bool {
    values.len() < 2 || contains_non_finite(values)
}

fn median_of_sorted(sorted: &[f64]) -> Option<f64> {
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

// API

pub fn mean(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
        return None;
    }

    let sum: f64 = values.iter().sum();
    Some(sum / values.len() as f64)
}

pub fn sum(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
        return None;
    }

    let sum: f64 = values.iter().sum();
    Some(sum)
}

pub fn min(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
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
    if invalid_input(values) {
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
    if invalid_input(values) {
        return None;
    }

    let maximum = max(values)?;
    let minimum = min(values)?;

    Some(maximum - minimum)
}

pub fn median(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
        return None;
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    median_of_sorted(&sorted)
}

pub fn variance_population(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
        return None;
    }

    let len: f64 = values.len() as f64;
    let ssd = sum_squared_deviations(values)?;

    Some(ssd / len)
}

pub fn variance_sample(values: &[f64]) -> Option<f64> {
    if invalid_sample_input(values) {
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

pub fn quartiles(values: &[f64]) -> Option<(f64, f64, f64)> {
    if invalid_sample_input(values) {
        return None;
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len();

    let mid = n / 2;

    let (lower, upper) = if n % 2 == 0 {
        (&sorted[..mid], &sorted[mid..])
    } else {
        (&sorted[..mid], &sorted[mid + 1..])
    };

    let q2 = median_of_sorted(&sorted)?;

    let q1 = median_of_sorted(lower)?;

    let q3 = median_of_sorted(upper)?;

    Some((q1, q2, q3))
}

pub fn iqr(values: &[f64]) -> Option<f64> {
    let (q1, _, q3) = quartiles(values)?;

    Some(q3 - q1)
}

pub fn mad(values: &[f64]) -> Option<f64> {
    if invalid_input(values) {
        return None;
    }

    let median = median(values)?;

    let mut absolute_deviations = Vec::with_capacity(values.len());

    for &value in values.iter() {
        absolute_deviations.push((value - median).abs());
    }

    absolute_deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());

    median_of_sorted(&absolute_deviations)
}

pub fn percentile(values: &[f64], p: f64) -> Option<f64> {
    if invalid_input(values) {
        return None;
    }

    if !p.is_finite() {
        return None;
    }

    if p < 0.0 || p > 100.0 {
        return None;
    }

    let mut sorted = values.to_vec();

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len();
    let position = (p / 100.0) * (n as f64 - 1.0);

    let lower_index = position.floor() as usize;
    let upper_index = position.ceil() as usize;

    let weight = position - lower_index as f64;

    let lower = sorted[lower_index];
    let upper = sorted[upper_index];

    Some(lower + weight * (upper - lower))
}

pub fn cv_population(values: &[f64]) -> Option<f64> {
    let mean_value = mean(values)?;

    if mean_value == 0.0 {
        return None;
    }

    Some(stddev_population(values)? / mean_value)
}

pub fn cv_sample(values: &[f64]) -> Option<f64> {
    let mean_value = mean(values)?;

    if mean_value == 0.0 {
        return None;
    }

    Some(stddev_sample(values)? / mean_value)
}

pub fn sem(values: &[f64]) -> Option<f64> {
    let deviation = stddev_sample(values)?;
    let n = values.len() as f64;

    Some(deviation / n.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_option_float_close(actual: Option<f64>, expected: f64) {
        let epsilon = 1e-10;
        let actual = actual.expect("expected Some(f64), got None");

        assert!(
            (actual - expected).abs() < epsilon,
            "actual: {}, expected: {}, diff: {}",
            actual,
            expected,
            (actual - expected).abs()
        );
    }

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

        assert_option_float_close(stddev_sample(&[1.0, 1.0, 3.0, 3.0]), expected);
    }

    #[test]
    fn variance_population_returns_fractional_variance() {
        assert_option_float_close(variance_population(&[1.0, 2.0, 3.0]), 2.0 / 3.0);
    }

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
    fn mean_returns_none_for_nan() {
        assert_eq!(mean(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn median_returns_none_for_nan() {
        assert_eq!(median(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn variance_population_returns_none_for_infinity() {
        assert_eq!(variance_population(&[1.0, f64::INFINITY, 3.0]), None);
    }

    #[test]
    fn variance_sample_returns_none_for_nan() {
        assert_eq!(variance_sample(&[1.0, f64::NAN, 3.0]), None);
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
    fn quartiles_returns_none_for_empty_input() {
        assert_eq!(quartiles(&[]), None);
    }

    #[test]
    fn quartiles_returns_none_for_one_element_input() {
        assert_eq!(quartiles(&[5.0]), None);
    }

    #[test]
    fn quartiles_returns_result_for_two_element_input() {
        assert_eq!(quartiles(&[1.0, 2.0]), Some((1.0, 1.5, 2.0)));
    }

    #[test]
    fn quartiles_returns_result_for_three_element_input() {
        assert_eq!(quartiles(&[1.0, 2.0, 3.0]), Some((1.0, 2.0, 3.0)));
    }

    #[test]
    fn quartiles_returns_result_for_four_element_input() {
        assert_eq!(quartiles(&[1.0, 2.0, 3.0, 4.0]), Some((1.5, 2.5, 3.5)));
    }

    #[test]
    fn quartiles_returns_result_for_five_element_input() {
        assert_eq!(quartiles(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some((1.5, 3.0, 4.5)));
    }

    #[test]
    fn quartiles_returns_result_for_tens_input() {
        assert_eq!(
            quartiles(&[10.0, 10.0, 10.0, 10.0]),
            Some((10.0, 10.0, 10.0))
        );
    }

    #[test]
    fn quartiles_returns_result_for_three_element_input_with_nan() {
        assert_eq!(quartiles(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn iqr_returns_result_for_two_element_input() {
        assert_eq!(iqr(&[1.0, 2.0]), Some(1.0));
    }

    #[test]
    fn iqr_returns_result_for_three_element_input() {
        assert_eq!(iqr(&[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn iqr_returns_result_for_four_element_input() {
        assert_eq!(iqr(&[1.0, 2.0, 3.0, 4.0]), Some(2.0));
    }

    #[test]
    fn iqr_returns_result_for_five_element_input() {
        assert_eq!(iqr(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
    }

    #[test]
    fn mad_returns_none_for_empty_input() {
        assert_eq!(mad(&[]), None);
    }

    #[test]
    fn mad_returns_zero_for_one_input() {
        assert_eq!(mad(&[5.0]), Some(0.0));
    }

    #[test]
    fn mad_returns_zero_for_same_elements() {
        assert_eq!(mad(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn mad_returns_result_for_odd_input() {
        assert_eq!(mad(&[1.0, 2.0, 3.0, 4.0, 100.0]), Some(1.0));
    }

    #[test]
    fn mad_returns_result_for_even_input() {
        assert_eq!(mad(&[1.0, 2.0, 3.0, 4.0]), Some(1.0));
    }

    #[test]
    fn mad_returns_none_for_nan() {
        assert_eq!(mad(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn mad_returns_none_for_infinity() {
        assert_eq!(mad(&[1.0, f64::INFINITY, 3.0]), None);
    }

    #[test]
    fn percentile_returns_none_for_empty_input() {
        assert_eq!(percentile(&[], 50.0), None);
    }

    #[test]
    fn percentile_returns_none_for_invalid_p() {
        assert_eq!(percentile(&[1.0, 2.0, 3.0], -1.0), None);
        assert_eq!(percentile(&[1.0, 2.0, 3.0], 101.0), None);
        assert_eq!(percentile(&[1.0, 2.0, 3.0], f64::NAN), None);
    }

    #[test]
    fn percentile_returns_none_for_non_finite_values() {
        assert_eq!(percentile(&[1.0, f64::NAN, 3.0], 50.0), None);
    }

    #[test]
    fn percentile_returns_min_for_zero_percentile() {
        assert_eq!(percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 0.0), Some(1.0));
    }

    #[test]
    fn percentile_returns_median_for_fifty_percentile() {
        assert_eq!(percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 50.0), Some(3.0));
    }

    #[test]
    fn percentile_returns_max_for_hundred_percentile() {
        assert_eq!(percentile(&[1.0, 2.0, 3.0, 4.0, 5.0], 100.0), Some(5.0));
    }

    #[test]
    fn percentile_interpolates_between_values() {
        assert_eq!(percentile(&[10.0, 20.0], 25.0), Some(12.5));
    }

    #[test]
    fn cv_population_returns_none_for_empty_input() {
        assert_eq!(cv_population(&[]), None);
    }

    #[test]
    fn cv_population_returns_none_for_nan() {
        assert_eq!(cv_population(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn cv_population_returns_none_for_zero_mean() {
        assert_eq!(cv_population(&[-1.0, 1.0]), None);
    }

    #[test]
    fn cv_population_returns_zero_for_one_input() {
        assert_eq!(cv_population(&[5.0]), Some(0.0));
    }

    #[test]
    fn cv_population_returns_zero_for_same_elements() {
        assert_eq!(cv_population(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn cv_population_returns_cv_for_positive_mean() {
        assert_eq!(cv_population(&[1.0, 1.0, 3.0, 3.0]), Some(0.5));
    }

    #[test]
    fn cv_population_returns_negative_cv_for_negative_mean() {
        assert_eq!(cv_population(&[-1.0, -1.0, -3.0, -3.0]), Some(-0.5));
    }

    #[test]
    fn cv_population_returns_fractional_cv() {
        let expected = (2.0_f64 / 3.0).sqrt() / 2.0;
        assert_option_float_close(cv_population(&[1.0, 2.0, 3.0]), expected);
    }

    #[test]
    fn cv_sample_returns_none_for_empty_input() {
        assert_eq!(cv_sample(&[]), None);
    }

    #[test]
    fn cv_sample_returns_none_for_one_input() {
        assert_eq!(cv_sample(&[5.0]), None);
    }

    #[test]
    fn cv_sample_returns_none_for_nan() {
        assert_eq!(cv_sample(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn cv_sample_returns_none_for_zero_mean() {
        assert_eq!(cv_sample(&[-1.0, 1.0]), None);
    }

    #[test]
    fn cv_sample_returns_zero_for_same_elements() {
        assert_eq!(cv_sample(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn cv_sample_returns_cv_for_positive_mean() {
        let expected = (4.0_f64 / 3.0).sqrt() / 2.0;
        assert_option_float_close(cv_sample(&[1.0, 1.0, 3.0, 3.0]), expected);
    }

    #[test]
    fn cv_sample_returns_negative_cv_for_negative_mean() {
        let expected = (4.0_f64 / 3.0).sqrt() / -2.0;
        assert_option_float_close(cv_sample(&[-1.0, -1.0, -3.0, -3.0]), expected);
    }

    #[test]
    fn sem_returns_none_for_empty_input() {
        assert_eq!(sem(&[]), None);
    }

    #[test]
    fn sem_returns_none_for_one_input() {
        assert_eq!(sem(&[5.0]), None);
    }

    #[test]
    fn sem_returns_none_for_nan() {
        assert_eq!(sem(&[1.0, f64::NAN, 3.0]), None);
    }

    #[test]
    fn sem_returns_zero_for_same_elements() {
        assert_eq!(sem(&[10.0, 10.0, 10.0]), Some(0.0));
    }

    #[test]
    fn sem_returns_sem_for_known_values() {
        let expected = (4.0_f64 / 3.0).sqrt() / (4.0_f64).sqrt();
        assert_option_float_close(sem(&[1.0, 1.0, 3.0, 3.0]), expected);
    }
}
