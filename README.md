# pg_statkit

![CI](https://github.com/kirdmi/pg_statkit/actions/workflows/rust.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

A PostgreSQL statistical toolkit for clinical, health, and biomedical data analysis, written in Rust.

Native descriptive statistics and effect sizes inside PostgreSQL, with a focus on the measures used in clinical research and epidemiology. Built with [pgrx](https://github.com/pgcentralfoundation/pgrx).

## Why

PostgreSQL ships with basic descriptive statistics (`avg`, `stddev`, `percentile_cont`), but has no native support for effect sizes such as Cohen's d, odds ratio, and risk ratio, the working vocabulary of clinical research. Getting them usually means exporting data to R or Python.

`pg_statkit` provides these directly in SQL, for workflows where the data already lives in PostgreSQL and you'd rather not move it out for a single statistic. For full statistical analysis with confidence intervals and assumption checks, mature tools like R's [`effectsize`](https://easystats.github.io/effectsize/) remain the right choice. This extension covers the common case of computing a measure in place.

## Features

**Descriptive statistics**
- Central tendency: mean, median, sum
- Dispersion: variance and standard deviation (population and sample), range, IQR, MAD
- Quantiles: quartiles, arbitrary percentiles
- Coefficient of variation (population and sample), standard error of the mean

**Effect sizes**
- 2×2 contingency measures: risk difference, risk ratio, odds ratio
- Standardized mean differences: Cohen's d, Hedges' g, Glass's Δ

## Planned

- Hypothesis testing (t-tests, chi-square, non-parametric)
- Confidence intervals
- Research-ready statistical summaries

## Installation

Supports PostgreSQL 13 to 17.

### Prebuilt binaries (Linux x86_64)

Download the archive for your PostgreSQL version from the [latest release](https://github.com/kirdmi/pg_statkit/releases/latest), then copy the files into the directories reported by `pg_config`:

```sh
tar xzf pg_statkit-pg16-linux-x86_64.tar.gz
cp pg_statkit.so "$(pg_config --pkglibdir)/"
cp pg_statkit*.sql pg_statkit.control "$(pg_config --sharedir)/extension/"
```

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) and [cargo-pgrx](https://github.com/pgcentralfoundation/pgrx).

```sh
cargo install --locked cargo-pgrx --version 0.18.1
cargo pgrx install
```

### Enable the extension

```sql
CREATE EXTENSION pg_statkit;
```

## Usage

### Descriptive statistics

Input is `double precision[]`. Note the `::float8[]` cast: array literals like `ARRAY[1.0, 2.0]` default to `numeric[]` in PostgreSQL and must be cast explicitly.

```sql
SELECT statkit_mean('{1,2,3,4,5}'::float8[]);              -- 3
SELECT statkit_median('{1,2,3,4,5}'::float8[]);            -- 3
SELECT statkit_stddev_sample('{1,1,3,3}'::float8[]);       -- 1.1547...
SELECT statkit_iqr('{1,2,3,4,5}'::float8[]);               -- 3
SELECT statkit_percentile('{1,2,3,4,5}'::float8[], 40);    -- 2.6
SELECT statkit_cv_sample('{1,1,3,3}'::float8[]);           -- 0.5773...
SELECT statkit_sem('{1,1,3,3}'::float8[]);                 -- 0.5773...
```

### Effect sizes

**2×2 contingency tables** take four integer counts (`bigint`), laid out as:

```
                outcome    no outcome
group 1            a            b
group 2            c            d
```

```sql
SELECT statkit_risk_difference(8, 2, 2, 8);   -- 0.6
SELECT statkit_risk_ratio(6, 4, 3, 7);        -- 2
SELECT statkit_odds_ratio(8, 2, 2, 8);        -- 16

-- Named arguments make the cell layout explicit and order-safe:
SELECT statkit_odds_ratio(a => 8, b => 2, c => 2, d => 8);
```

`statkit_odds_ratio` applies the Haldane-Anscombe correction (adds 0.5 to all cells) when any cell is zero, so the returned value in that case is a corrected estimate rather than the raw odds ratio.

**Standardized mean differences** take two `double precision[]` samples:

```sql
SELECT statkit_cohens_d('{3,4,5}'::float8[], '{1,2,3}'::float8[]);   -- 2
SELECT statkit_hedges_g('{3,4,5}'::float8[], '{1,2,3}'::float8[]);   -- 1.6

-- Glass's Δ uses the control group's SD as the denominator,
-- so argument order is significant:
SELECT statkit_glass_delta(
    treatment => '{3,4,5}'::float8[],
    control   => '{1,2,3}'::float8[]
);
```

### On table columns

Functions take arrays, so aggregate a column with `array_agg`. Cast to `float8` to be safe when the column is `numeric`:

```sql
-- One statistic over a whole column
SELECT statkit_mean(array_agg(value::float8)) FROM measurements;

-- Grouped, e.g. one summary per patient
SELECT patient_id, statkit_median(array_agg(value::float8)) AS median_value
FROM measurements
GROUP BY patient_id;

-- Effect size between two groups
SELECT statkit_cohens_d(
    array_agg(value::float8) FILTER (WHERE arm = 'treatment'),
    array_agg(value::float8) FILTER (WHERE arm = 'control')
)
FROM trial;
```

Functions return `NULL` when a statistic is undefined for the input (empty or too-small samples, non-finite values, zero denominators).

## License

MIT
