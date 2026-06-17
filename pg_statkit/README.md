# pg_statkit

A PostgreSQL statistical toolkit for product analytics, A/B testing, and applied research.

## Goals

- Descriptive statistics
- Confidence intervals
- Hypothesis testing
- Effect sizes
- A/B testing utilities
- Research-ready statistical reports

## Status

Early private development

Structure:

pg_statkit/
├── Cargo.toml
├── README.md
├── ROADMAP.md
├── LICENSE
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── sql/
├── tests/
└── src/
    ├── lib.rs
    ├── descriptive.rs
    ├── intervals.rs
    ├── hypothesis.rs
    ├── effect_size.rs
    ├── product.rs
    ├── report.rs
    └── utils.rs.
