mod descriptive;
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

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

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

        assert_eq!(
            crate::statkit_stddev_sample(vec![1.0, 1.0, 3.0, 3.0]),
            Some(expected)
        );
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
