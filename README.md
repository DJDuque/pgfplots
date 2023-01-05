# PGFPlots

[![Test Status](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml/badge.svg)](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/pgfplots?labelColor=383f47)](https://crates.io/crates/pgfplots)

A Rust library to generate publication-quality figures. This crate is a PGFPlots
code generator, and provides utilities to create, customize, and compile 
high-quality plots.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
pgfplots = "0.4"
```

Plotting a quadratic function is as simple as:

```rust
use pgfplots::{axis::plot::Plot2D, Engine, Picture};

let mut plot = Plot2D::new();
plot.coordinates = (-100..100)
    .into_iter()
    .map(|i| (f64::from(i), f64::from(i*i)).into())
    .collect();

Picture::from(plot).show_pdf(Engine::PdfLatex)?;
```

## [Examples](https://github.com/DJDuque/pgfplots/tree/main/examples)

A more extensive list of examples and their source code is available in the
`examples/` directory (runnable with
`cargo run --all-features --example example_name`).

|[[code]](https://github.com/DJDuque/pgfplots/blob/main/examples/fitted_line.rs)|[[code]](https://github.com/DJDuque/pgfplots/blob/main/examples/rectangle_integration.rs)|[[code]](https://github.com/DJDuque/pgfplots/blob/main/examples/snowflake.rs)|
|-|-|-|
|![](https://github.com/DJDuque/pgfplots/raw/main/examples/fitted_line.png)|![](https://github.com/DJDuque/pgfplots/raw/main/examples/rectangle_integration.png)|![](https://github.com/DJDuque/pgfplots/raw/main/examples/snowflake.png)|

## Features

- Tectonic: Allow users to process the LaTeX code that generates figures
without relying on any externally installed software, configuration, or
resource files. This is achieved by including the
[tectonic](https://crates.io/crates/tectonic) crate as a dependency.

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
