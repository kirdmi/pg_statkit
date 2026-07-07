mod descriptive;
mod effect_size;
mod utils;
use pgrx::prelude::*;

::pgrx::pg_module_magic!(name, version);

#[pg_extern]
fn statkit_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pg_extern]
fn statkit_mean(values: Vec<f64>) -> Option<f64> {
    descriptive::mean(&values)
}

#[pg_extern]
fn statkit_sum(values: Vec<f64>) -> Option<f64> {
    descriptive::sum(&values)
}

#[pg_extern]
fn statkit_min(values: Vec<f64>) -> Option<f64> {
    descriptive::min(&values)
}

#[pg_extern]
fn statkit_max(values: Vec<f64>) -> Option<f64> {
    descriptive::max(&values)
}

#[pg_extern]
fn statkit_range(values: Vec<f64>) -> Option<f64> {
    descriptive::range(&values)
}

#[pg_extern]
fn statkit_median(values: Vec<f64>) -> Option<f64> {
    descriptive::median(&values)
}

#[pg_extern]
fn statkit_variance_population(values: Vec<f64>) -> Option<f64> {
    descriptive::variance_population(&values)
}

#[pg_extern]
fn statkit_variance_sample(values: Vec<f64>) -> Option<f64> {
    descriptive::variance_sample(&values)
}

#[pg_extern]
fn statkit_stddev_population(values: Vec<f64>) -> Option<f64> {
    descriptive::stddev_population(&values)
}

#[pg_extern]
fn statkit_stddev_sample(values: Vec<f64>) -> Option<f64> {
    descriptive::stddev_sample(&values)
}

#[pg_extern]
fn statkit_q1(values: Vec<f64>) -> Option<f64> {
    let (q1, _, _) = descriptive::quartiles(&values)?;
    Some(q1)
}

#[pg_extern]
fn statkit_q2(values: Vec<f64>) -> Option<f64> {
    let (_, q2, _) = descriptive::quartiles(&values)?;
    Some(q2)
}

#[pg_extern]
fn statkit_q3(values: Vec<f64>) -> Option<f64> {
    let (_, _, q3) = descriptive::quartiles(&values)?;
    Some(q3)
}

#[pg_extern]
fn statkit_iqr(values: Vec<f64>) -> Option<f64> {
    descriptive::iqr(&values)
}

#[pg_extern]
fn statkit_mad(values: Vec<f64>) -> Option<f64> {
    descriptive::mad(&values)
}

#[pg_extern]
fn statkit_percentile(values: Vec<f64>, p: f64) -> Option<f64> {
    descriptive::percentile(&values, p)
}

#[pg_extern]
fn statkit_cv_population(values: Vec<f64>) -> Option<f64> {
    descriptive::cv_population(&values)
}

#[pg_extern]
fn statkit_cv_sample(values: Vec<f64>) -> Option<f64> {
    descriptive::cv_sample(&values)
}

#[pg_extern]
fn statkit_sem(values: Vec<f64>) -> Option<f64> {
    descriptive::sem(&values)
}

#[pg_extern]
fn statkit_risk_difference(a: i64, b: i64, c: i64, d: i64) -> Option<f64> {
    effect_size::risk_difference(a, b, c, d)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
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

    #[pg_test]
    fn test_statkit_version() {
        assert_eq!(crate::statkit_version(), env!("CARGO_PKG_VERSION"));
    }

    #[pg_test]
    fn test_statkit_mean() {
        assert_eq!(crate::statkit_mean(vec![1.0, 2.0, 3.0]), Some(2.0));
    }

    #[pg_test]
    fn test_statkit_mean_empty() {
        assert_eq!(crate::statkit_mean(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_sum() {
        assert_eq!(crate::statkit_sum(vec![1.0, 2.0, 3.0]), Some(6.0));
    }

    #[pg_test]
    fn test_statkit_sum_empty() {
        assert_eq!(crate::statkit_sum(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_min() {
        assert_eq!(crate::statkit_min(vec![1.0, 2.0, 3.0]), Some(1.0));
    }

    #[pg_test]
    fn test_statkit_min_empty() {
        assert_eq!(crate::statkit_min(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_max() {
        assert_eq!(crate::statkit_max(vec![1.0, 2.0, 3.0]), Some(3.0));
    }

    #[pg_test]
    fn test_statkit_max_empty() {
        assert_eq!(crate::statkit_max(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_range() {
        assert_eq!(crate::statkit_range(vec![1.0, 2.0, 3.0]), Some(2.0));
    }

    #[pg_test]
    fn test_statkit_range_empty() {
        assert_eq!(crate::statkit_range(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_median_odd() {
        assert_eq!(crate::statkit_median(vec![1.0, 2.0, 3.0]), Some(2.0));
    }

    #[pg_test]
    fn test_statkit_median_even() {
        assert_eq!(crate::statkit_median(vec![1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }

    #[pg_test]
    fn test_statkit_median_empty() {
        assert_eq!(crate::statkit_median(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_variance_population_empty() {
        assert_eq!(crate::statkit_variance_population(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_variance_population_zero_for_one_input() {
        assert_eq!(crate::statkit_variance_population(vec![5.0]), Some(0.0));
    }

    #[pg_test]
    fn test_statkit_variance_population_variance_for_list_of_different_elements() {
        assert_eq!(
            crate::statkit_variance_population(vec![1.0, 2.0, 3.0, 4.0]),
            Some(1.25)
        );
    }

    #[pg_test]
    fn test_statkit_variance_population_variance_for_list_of_same_elements() {
        assert_eq!(
            crate::statkit_variance_population(vec![10.0, 10.0, 10.0]),
            Some(0.0)
        );
    }

    #[pg_test]
    fn test_statkit_variance_sample_empty() {
        assert_eq!(crate::statkit_variance_sample(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_variance_sample_zero_for_one_input() {
        assert_eq!(crate::statkit_variance_sample(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_variance_sample_variance_for_list_of_different_elements() {
        assert_eq!(
            crate::statkit_variance_sample(vec![1.0, 2.0, 3.0, 4.0, 5.0]),
            Some(2.5)
        );
    }

    #[pg_test]
    fn test_statkit_variance_sample_variance_for_list_of_same_elements() {
        assert_eq!(
            crate::statkit_variance_sample(vec![10.0, 10.0, 10.0]),
            Some(0.0)
        );
    }

    #[pg_test]
    fn test_statkit_stddev_population_returns_none_for_empty_input() {
        assert_eq!(crate::statkit_stddev_population(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_stddev_population_returns_zero_for_one_input() {
        assert_eq!(crate::statkit_stddev_population(vec![5.0]), Some(0.0));
    }

    #[pg_test]
    fn test_statkit_stddev_population_returns_zero_for_same_elements() {
        assert_eq!(
            crate::statkit_stddev_population(vec![10.0, 10.0, 10.0]),
            Some(0.0)
        );
    }

    #[pg_test]
    fn test_statkit_stddev_population_returns_stddev_for_different_elements() {
        assert_eq!(
            crate::statkit_stddev_population(vec![1.0, 1.0, 3.0, 3.0]),
            Some(1.0)
        );
    }

    #[pg_test]
    fn test_statkit_stddev_sample_returns_none_for_empty_input() {
        assert_eq!(crate::statkit_stddev_sample(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_stddev_sample_returns_none_for_one_input() {
        assert_eq!(crate::statkit_stddev_sample(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_stddev_sample_returns_zero_for_same_elements() {
        assert_eq!(
            crate::statkit_stddev_sample(vec![10.0, 10.0, 10.0]),
            Some(0.0)
        );
    }

    #[pg_test]
    fn test_statkit_stddev_sample_returns_stddev_for_different_elements() {
        let expected = (4.0_f64 / 3.0).sqrt();

        assert_option_float_close(
            crate::statkit_stddev_sample(vec![1.0, 1.0, 3.0, 3.0]),
            expected,
        );
    }

    #[pg_test]
    fn test_statkit_q1_empty() {
        assert_eq!(crate::statkit_q1(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_q2_empty() {
        assert_eq!(crate::statkit_q2(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_q3_empty() {
        assert_eq!(crate::statkit_q3(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_q1_one_element() {
        assert_eq!(crate::statkit_q1(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_q2_one_element() {
        assert_eq!(crate::statkit_q2(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_q3_one_element() {
        assert_eq!(crate::statkit_q3(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_quartiles_even() {
        let values = vec![1.0, 2.0, 3.0, 4.0];

        assert_eq!(crate::statkit_q1(values.clone()), Some(1.5));
        assert_eq!(crate::statkit_q2(values.clone()), Some(2.5));
        assert_eq!(crate::statkit_q3(values), Some(3.5));
    }

    #[pg_test]
    fn test_statkit_quartiles_odd() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        assert_eq!(crate::statkit_q1(values.clone()), Some(1.5));
        assert_eq!(crate::statkit_q2(values.clone()), Some(3.0));
        assert_eq!(crate::statkit_q3(values), Some(4.5));
    }

    #[pg_test]
    fn test_statkit_quartiles_nan() {
        let values = vec![1.0, f64::NAN, 3.0];

        assert_eq!(crate::statkit_q1(values.clone()), None);
        assert_eq!(crate::statkit_q2(values.clone()), None);
        assert_eq!(crate::statkit_q3(values), None);
    }

    #[pg_test]
    fn test_statkit_iqr_empty() {
        assert_eq!(crate::statkit_iqr(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_iqr_valid_input() {
        assert_eq!(crate::statkit_iqr(vec![1.0, 2.0, 3.0, 4.0, 5.0]), Some(3.0));
    }

    #[pg_test]
    fn test_statkit_iqr_nan() {
        assert_eq!(crate::statkit_iqr(vec![1.0, f64::NAN, 3.0]), None);
    }

    #[pg_test]
    fn test_statkit_mad_empty() {
        assert_eq!(crate::statkit_mad(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_mad_valid_input() {
        assert_eq!(
            crate::statkit_mad(vec![1.0, 2.0, 3.0, 4.0, 100.0]),
            Some(1.0)
        );
    }

    #[pg_test]
    fn test_statkit_mad_nan() {
        assert_eq!(crate::statkit_mad(vec![1.0, f64::NAN, 3.0]), None);
    }

    #[pg_test]
    fn test_statkit_cv_population() {
        assert_eq!(
            crate::statkit_cv_population(vec![1.0, 1.0, 3.0, 3.0]),
            Some(0.5)
        );
    }

    #[pg_test]
    fn test_statkit_cv_population_zero_mean() {
        assert_eq!(crate::statkit_cv_population(vec![-1.0, 1.0]), None);
    }

    #[pg_test]
    fn test_statkit_cv_population_empty() {
        assert_eq!(crate::statkit_cv_population(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_cv_sample() {
        let expected = (4.0_f64 / 3.0).sqrt() / 2.0;
        assert_option_float_close(crate::statkit_cv_sample(vec![1.0, 1.0, 3.0, 3.0]), expected);
    }

    #[pg_test]
    fn test_statkit_cv_sample_zero_mean() {
        assert_eq!(crate::statkit_cv_sample(vec![-1.0, 1.0]), None);
    }

    #[pg_test]
    fn test_statkit_cv_sample_one_element() {
        assert_eq!(crate::statkit_cv_sample(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_sem() {
        let expected = (4.0_f64 / 3.0).sqrt() / (4.0_f64).sqrt();
        assert_option_float_close(crate::statkit_sem(vec![1.0, 1.0, 3.0, 3.0]), expected);
    }

    #[pg_test]
    fn test_statkit_sem_one_element() {
        assert_eq!(crate::statkit_sem(vec![5.0]), None);
    }

    #[pg_test]
    fn test_statkit_sem_empty() {
        assert_eq!(crate::statkit_sem(vec![]), None);
    }

    #[pg_test]
    fn test_statkit_risk_difference() {
        assert_option_float_close(crate::statkit_risk_difference(8, 2, 2, 8), 0.6);
    }

    #[pg_test]
    fn test_statkit_risk_difference_empty_group() {
        assert_eq!(crate::statkit_risk_difference(0, 0, 3, 7), None);
    }

    #[pg_test]
    fn test_statkit_risk_difference_negative_input() {
        assert_eq!(crate::statkit_risk_difference(-1, 5, 3, 7), None);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
