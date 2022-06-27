# PGFPlots

[![Test Status](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml/badge.svg)](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/pgfplots?labelColor=383f47)](https://crates.io/crates/pgfplots)
[![GitHub commits since latest release (by date)](https://img.shields.io/github/commits-since/DJDuque/pgfplots/latest?labelColor=383f47)](https://github.com/DJDuque/pgfplots/commits/main)

A Rust library to generate publication-quality figures. This crate is a PGFPlots
code generator, and provides utilities to create, customize, and compile 
high-quality plots.

## Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
pgfplots = { version = "0.4", features = ["inclusive"] }
```

Plotting a quadratic function is as simple as:

```rust
use pgfplots::axis::plot::Plot2D;

let mut plot = Plot2D::new();
plot.coordinates = (-100..100)
    .into_iter()
    .map(|i| (f64::from(i), f64::from(i*i)).into())
    .collect();

plot.show()?;
```

## [Examples](https://github.com/DJDuque/pgfplots/tree/main/examples)

A more extensive list of examples and their source code is available in the
`examples/` directory (runnable with
`cargo run --all-features --example example_name`).

|[[code]](https://github.com/DJDuque/pgfplots/blob/main/examples/fitted_line.rs)|[[code]](https://github.com/DJDuque/pgfplots/blob/main/examples/rectangle_integration.rs)|[[code]](https://github.com/DJDuque/pgfplots/blob/main/examples/snowflake.rs)|
|-|-|-|
|![](https://github.com/DJDuque/pgfplots/raw/main/examples/fitted_line.png)|![](https://github.com/DJDuque/pgfplots/raw/main/examples/rectangle_integration.png)|![](https://github.com/DJDuque/pgfplots/raw/main/examples/snowflake.png)|

## Features

- Inclusive: Allow users to process the LaTeX code that generates figures
without relying on any externally installed software, configuration, or
resource files. This is achieved by including the
[tectonic](https://crates.io/crates/tectonic) crate as a dependency.

	If you already have a LaTeX distribution installed in your system, it is
recommended to process the LaTeX code directly. The `tectonic` crate pulls in a
lot of dependencies, which significantly increase compilation and processing
 times. Plotting a quadratic function is still very simple:

	```rust
	use pgfplots::axis::plot::Plot2D;
	use std::process::{Command, Stdio};

	let mut plot = Plot2D::new();
	plot.coordinates = (-100..100)
		.into_iter()
		.map(|i| (f64::from(i), f64::from(i*i)).into())
		.collect();

	let argument = plot.standalone_string().replace('\n', "").replace('\t', "");
	Command::new("pdflatex")
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.arg("-interaction=batchmode")
		.arg("-halt-on-error")
		.arg("-jobname=figure")
		.arg(argument)
		.status()
		.expect("Error: unable to run pdflatex");
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
