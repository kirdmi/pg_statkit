use pgrx::prelude::*;

::pgrx::pg_module_magic!(name, version);

#[pg_extern]
fn statkit_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pg_extern]
fn statkit_mean(values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let sum: f64 = values.iter().sum();
    Some(sum / values.len() as f64)
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
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
