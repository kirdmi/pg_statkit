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
}
