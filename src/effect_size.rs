use crate::descriptive::{mean, stddev_sample, variance_sample};
use crate::utils::invalid_sample_input;

pub fn risk_difference(a: i64, b: i64, c: i64, d: i64) -> Option<f64> {
    if [a, b, c, d].iter().any(|&x| x < 0) {
        return None;
    }

    let group_1_n = (a + b) as f64;
    let group_2_n = (c + d) as f64;

    if group_1_n == 0.0 || group_2_n == 0.0 {
        return None;
    }

    let risk_1 = a as f64 / group_1_n;
    let risk_2 = c as f64 / group_2_n;

    Some(risk_1 - risk_2)
}

pub fn risk_ratio(a: i64, b: i64, c: i64, d: i64) -> Option<f64> {
    if [a, b, c, d].iter().any(|&x| x < 0) {
        return None;
    }

    let group_1_n = (a + b) as f64;
    let group_2_n = (c + d) as f64;

    if group_1_n == 0.0 || group_2_n == 0.0 {
        return None;
    }

    let risk_1 = a as f64 / group_1_n;
    let risk_2 = c as f64 / group_2_n;

    if c == 0 {
        return None;
    }

    Some(risk_1 / risk_2)
}

pub fn odds_ratio(a: i64, b: i64, c: i64, d: i64) -> Option<f64> {
    if [a, b, c, d].iter().any(|&x| x < 0) {
        return None;
    }

    let correction = if [a, b, c, d].iter().any(|&x| x == 0) {
        0.5
    } else {
        0.0
    };

    let odds_ratio_value = ((a as f64 + correction) * (d as f64 + correction))
        / ((b as f64 + correction) * (c as f64 + correction));

    Some(odds_ratio_value)
}

pub fn cohens_d(group_1: &[f64], group_2: &[f64]) -> Option<f64> {
    if invalid_sample_input(group_1) || invalid_sample_input(group_2) {
        return None;
    }

    let mean_1 = mean(group_1)?;
    let mean_2 = mean(group_2)?;
    let variance_sample_1 = variance_sample(group_1)?;
    let variance_sample_2 = variance_sample(group_2)?;
    let group_1_len = group_1.len() as f64;
    let group_2_len = group_2.len() as f64;
    let pooled_variance = ((group_1_len - 1.0) * variance_sample_1
        + (group_2_len - 1.0) * variance_sample_2)
        / (group_1_len + group_2_len - 2.0);
    let pooled_sd = pooled_variance.sqrt();
    if pooled_sd == 0.0 {
        return None;
    }

    Some((mean_1 - mean_2) / pooled_sd)
}

pub fn hedges_g(group_1: &[f64], group_2: &[f64]) -> Option<f64> {
    let cohens_d_value = cohens_d(group_1, group_2)?;
    let group_1_len = group_1.len() as f64;
    let group_2_len = group_2.len() as f64;
    let j_value = 1.0 - (3.0 / (4.0 * (group_1_len + group_2_len) - 9.0));

    Some(cohens_d_value * j_value)
}

pub fn glass_delta(treatment: &[f64], control: &[f64]) -> Option<f64> {
    if invalid_sample_input(treatment) || invalid_sample_input(control) {
        return None;
    }

    let mean_treatment = mean(treatment)?;
    let mean_control = mean(control)?;

    let sd_control = stddev_sample(control)?;
    if sd_control == 0.0 {
        return None;
    }

    Some((mean_treatment - mean_control) / sd_control)
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
    fn risk_difference_returns_none_for_negative_input() {
        assert_eq!(risk_difference(-1, 5, 3, 7), None);
    }

    #[test]
    fn risk_difference_returns_none_for_empty_group_one() {
        assert_eq!(risk_difference(0, 0, 3, 7), None);
    }

    #[test]
    fn risk_difference_returns_none_for_empty_group_two() {
        assert_eq!(risk_difference(3, 7, 0, 0), None);
    }

    #[test]
    fn risk_difference_returns_zero_for_equal_risks() {
        assert_eq!(risk_difference(5, 5, 10, 10), Some(0.0));
    }

    #[test]
    fn risk_difference_returns_positive_for_higher_risk_group_one() {
        assert_option_float_close(risk_difference(8, 2, 2, 8), 0.6);
    }

    #[test]
    fn risk_difference_returns_negative_for_lower_risk_group_one() {
        assert_option_float_close(risk_difference(2, 8, 8, 2), -0.6);
    }

    #[test]
    fn risk_difference_handles_zero_events_in_group() {
        assert_eq!(risk_difference(0, 10, 5, 5), Some(-0.5));
    }

    #[test]
    fn risk_difference_returns_fractional_result() {
        let expected = 1.0 / 3.0 - 0.5;
        assert_option_float_close(risk_difference(1, 2, 1, 1), expected);
    }

    #[test]
    fn risk_ratio_returns_none_for_negative_input() {
        assert_eq!(risk_ratio(-1, 5, 3, 7), None);
    }

    #[test]
    fn risk_ratio_returns_none_for_empty_group_one() {
        assert_eq!(risk_ratio(0, 0, 3, 7), None);
    }

    #[test]
    fn risk_ratio_returns_none_for_empty_group_two() {
        assert_eq!(risk_ratio(3, 7, 0, 0), None);
    }

    #[test]
    fn risk_ratio_returns_none_for_zero_risk_in_control() {
        assert_eq!(risk_ratio(5, 5, 0, 10), None);
    }

    #[test]
    fn risk_ratio_returns_one_for_equal_risks() {
        assert_eq!(risk_ratio(5, 5, 10, 10), Some(1.0));
    }

    #[test]
    fn risk_ratio_returns_two_for_double_risk() {
        assert_eq!(risk_ratio(6, 4, 3, 7), Some(2.0));
    }

    #[test]
    fn risk_ratio_returns_half_for_protective_effect() {
        assert_eq!(risk_ratio(2, 8, 4, 6), Some(0.5));
    }

    #[test]
    fn risk_ratio_returns_zero_for_zero_events_in_group_one() {
        assert_eq!(risk_ratio(0, 10, 5, 5), Some(0.0));
    }

    #[test]
    fn risk_ratio_returns_fractional_result() {
        let expected = (1.0 / 3.0) / 0.5;
        assert_option_float_close(risk_ratio(1, 2, 1, 1), expected);
    }

    #[test]
    fn odds_ratio_returns_none_for_negative_input() {
        assert_eq!(odds_ratio(-1, 5, 3, 7), None);
    }

    #[test]
    fn odds_ratio_returns_one_for_equal_odds() {
        assert_eq!(odds_ratio(5, 5, 5, 5), Some(1.0));
    }

    #[test]
    fn odds_ratio_returns_raw_or_without_zeros() {
        assert_option_float_close(odds_ratio(4, 6, 2, 8), 8.0 / 3.0);
    }

    #[test]
    fn odds_ratio_greater_than_one_for_higher_odds() {
        assert_eq!(odds_ratio(8, 2, 2, 8), Some(16.0));
    }

    #[test]
    fn odds_ratio_applies_haldane_for_zero_cell() {
        let expected = (0.5 * 5.5) / (10.5 * 5.5);
        assert_option_float_close(odds_ratio(0, 10, 5, 5), expected);
    }

    #[test]
    fn odds_ratio_haldane_does_not_return_zero_for_zero_numerator() {
        let result = odds_ratio(0, 10, 5, 5).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn odds_ratio_applies_haldane_for_zero_in_denominator_cell() {
        let expected = (5.5 * 5.5) / (0.5 * 5.5);
        assert_option_float_close(odds_ratio(5, 0, 5, 5), expected);
    }

    #[test]
    fn cohens_d_returns_none_for_empty_group_one() {
        assert_eq!(cohens_d(&[], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn cohens_d_returns_none_for_empty_group_two() {
        assert_eq!(cohens_d(&[1.0, 2.0, 3.0], &[]), None);
    }

    #[test]
    fn cohens_d_returns_none_for_one_element_group() {
        assert_eq!(cohens_d(&[5.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn cohens_d_returns_none_for_nan() {
        assert_eq!(cohens_d(&[1.0, f64::NAN, 3.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn cohens_d_returns_none_for_both_groups_zero_variance() {
        assert_eq!(cohens_d(&[5.0, 5.0, 5.0], &[8.0, 8.0, 8.0]), None);
    }

    #[test]
    fn cohens_d_returns_zero_for_equal_means() {
        assert_eq!(cohens_d(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), Some(0.0));
    }

    #[test]
    fn cohens_d_returns_positive_when_group_one_higher() {
        assert_option_float_close(cohens_d(&[3.0, 4.0, 5.0], &[1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn cohens_d_returns_negative_when_group_one_lower() {
        assert_option_float_close(cohens_d(&[1.0, 2.0, 3.0], &[3.0, 4.0, 5.0]), -2.0);
    }

    #[test]
    fn cohens_d_works_when_one_group_zero_variance() {
        let result = cohens_d(&[5.0, 5.0, 5.0], &[1.0, 2.0, 3.0]);
        assert!(result.is_some());
    }

    #[test]
    fn hedges_g_returns_none_for_empty_group() {
        assert_eq!(hedges_g(&[], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn hedges_g_returns_none_for_one_element_group() {
        assert_eq!(hedges_g(&[5.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn hedges_g_returns_none_for_nan() {
        assert_eq!(hedges_g(&[1.0, f64::NAN, 3.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn hedges_g_returns_none_for_both_groups_zero_variance() {
        assert_eq!(hedges_g(&[5.0, 5.0, 5.0], &[8.0, 8.0, 8.0]), None);
    }

    #[test]
    fn hedges_g_returns_zero_for_equal_means() {
        assert_eq!(hedges_g(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), Some(0.0));
    }

    #[test]
    fn hedges_g_applies_correction_factor() {
        let expected = 2.0 * 0.8;
        assert_option_float_close(hedges_g(&[3.0, 4.0, 5.0], &[1.0, 2.0, 3.0]), expected);
    }

    #[test]
    fn hedges_g_is_smaller_than_cohens_d() {
        let d = cohens_d(&[3.0, 4.0, 5.0], &[1.0, 2.0, 3.0]).unwrap();
        let g = hedges_g(&[3.0, 4.0, 5.0], &[1.0, 2.0, 3.0]).unwrap();
        assert!(g.abs() < d.abs());
    }

    #[test]
    fn hedges_g_returns_negative_when_group_one_lower() {
        let expected = -2.0 * 0.8;
        assert_option_float_close(hedges_g(&[1.0, 2.0, 3.0], &[3.0, 4.0, 5.0]), expected);
    }

    #[test]
    fn glass_delta_returns_none_for_empty_treatment() {
        assert_eq!(glass_delta(&[], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn glass_delta_returns_none_for_empty_control() {
        assert_eq!(glass_delta(&[1.0, 2.0, 3.0], &[]), None);
    }

    #[test]
    fn glass_delta_returns_none_for_one_element_group() {
        assert_eq!(glass_delta(&[5.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn glass_delta_returns_none_for_nan() {
        assert_eq!(glass_delta(&[1.0, f64::NAN, 3.0], &[1.0, 2.0, 3.0]), None);
    }

    #[test]
    fn glass_delta_returns_none_for_zero_variance_control() {
        assert_eq!(glass_delta(&[1.0, 2.0, 3.0], &[5.0, 5.0, 5.0]), None);
    }

    #[test]
    fn glass_delta_works_when_treatment_zero_variance() {
        let result = glass_delta(&[5.0, 5.0, 5.0], &[1.0, 2.0, 3.0]);
        assert!(result.is_some());
    }

    #[test]
    fn glass_delta_returns_zero_for_equal_means() {
        assert_eq!(glass_delta(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), Some(0.0));
    }

    #[test]
    fn glass_delta_returns_positive_when_treatment_higher() {
        assert_option_float_close(glass_delta(&[3.0, 4.0, 5.0], &[1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn glass_delta_returns_negative_when_treatment_lower() {
        assert_option_float_close(glass_delta(&[1.0, 2.0, 3.0], &[3.0, 4.0, 5.0]), -2.0);
    }

    #[test]
    fn glass_delta_is_order_sensitive() {
        let ab = glass_delta(&[3.0, 4.0, 5.0], &[10.0, 20.0, 60.0]);
        let ba = glass_delta(&[10.0, 20.0, 60.0], &[3.0, 4.0, 5.0]);
        assert_ne!(ab, ba);
    }
}
