# PGFPlots

[![Test Status](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml/badge.svg)](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/pgfplots?labelColor=383f47)](https://crates.io/crates/pgfplots)
[![GitHub commits since latest release (by date)](https://img.shields.io/github/commits-since/DJDuque/pgfplots/latest?labelColor=383f47)](https://github.com/DJDuque/pgfplots/commits/main)

A Rust library to generate publication-quality figures. This crate is a PGFPlots
code generator, and provides utilities to create, customize, and compile 
high-quality plots.

## Usage

Users need to have `pdflatex` available in their system with the `pgfplots`
package. In the unlikely scenario that you don't have this already, install
any LaTeX distribution manually.

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
pgfplots = "0.1"
```

Plotting a quadratic function is as simple as:

```rust
use pgfplots::axis::plot::Plot2D;

let mut plot = Plot2D::new();
plot.coordinates = (-100..100)
    .into_iter()
    .map(|i| (f64::from(i), f64::from(i*i)).into())
    .collect();

let status = plot
    .pdflatex_standalone("figure")
    .expect("failed to run pdflatex");

if status.success() {
    // There is a `figure.pdf` in current working directory with our picture
    // There are also `figure.log` and `figure.aux` that we can safely remove
}
```

## Want to contribute?

There are multiple ways to contribute:
- Install and test PGFPlots. If it doesn't work as expected please [open an
  issue](https://github.com/DJDuque/pgfplots/issues/new).
- Comment/propose a fix on some of the current [open 
issues](https://github.com/DJDuque/pgfplots/issues).
- Read through the [documentation](https://docs.rs/pgfplots). If there is 
  something confusing, or you have a suggestion for something that could be 
  improved, please let the maintainer(s) know.
- Help evaluate [open pull requests](https://github.com/DJDuque/pgfplots/pulls),
  by testing locally and reviewing what is proposed.
