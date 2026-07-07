# pg_statkit

A PostgreSQL statistical toolkit for clinical, health, and biomedical data analysis, written in Rust.

Native descriptive and inferential statistics inside PostgreSQL, with a focus on the measures used in clinical research and epidemiology.

## Goals

- Descriptive statistics
- Effect sizes (Cohen's d, Hedges' g, odds ratio, risk ratio, risk difference, and more)
- Hypothesis testing
- Confidence intervals
- Research-ready statistical summaries

## Status

Early development.

## Usage

```sql
CREATE EXTENSION pg_statkit;

-- Central tendency
SELECT statkit_mean('{1,2,3,4,5}'::float8[]);      -- 3
SELECT statkit_median('{1,2,3,4,5}'::float8[]);    -- 3

-- Dispersion
SELECT statkit_stddev_sample('{1,1,3,3}'::float8[]);       -- 1.1547...
SELECT statkit_variance_population('{1,2,3,4}'::float8[]);  -- 1.25
SELECT statkit_iqr('{1,2,3,4,5}'::float8[]);       -- 3
SELECT statkit_mad('{1,2,3,4,100}'::float8[]);     -- 1

-- Quantiles
SELECT statkit_q1('{1,2,3,4,5}'::float8[]);        -- 1.5
SELECT statkit_percentile('{1,2,3,4,5}'::float8[], 40);  -- 2.6
```

Input is `double precision[]`. Note the `::float8[]` cast: array literals
like `ARRAY[1.0, 2.0]` default to `numeric[]` in PostgreSQL and must be
cast explicitly.

```sql
-- On a table column
SELECT statkit_mean(array_agg(value::float8)) FROM measurements;

-- Grouped, e.g. one summary statistic per patient
SELECT patient_id, statkit_median(array_agg(value::float8)) AS median_value
FROM measurements
GROUP BY patient_id;
```
