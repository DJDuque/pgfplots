#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
//! A Rust library to generate publication-quality figures.
//!
//! This crate is a PGFPlots code generator, and provides utilities to create,
//! customize, and compile high-quality plots. The `inclusive` feature allows
//! users to fully process figures without relying on any externally installed
//! software.
//!
//! The library's API is designed to feel natural for LaTeX and PGFPlots users,
//! but no previous experience is required to start generating
//! publication-quality plots in Rust.
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest way to generate a plot is to use a
//! [`Plot2D`]. Plotting a quadratic function is as simple as:
//!
//! ```no_run
//! use pgfplots::axis::plot::Plot2D;
//!
//! let mut plot = Plot2D::new();
//! plot.coordinates = (-100..100)
//!     .into_iter()
//!     .map(|i| (f64::from(i), f64::from(i*i)).into())
//!     .collect();
//!
//! # #[cfg(feature = "inclusive")]
//! plot.show();
//! ```
//!
//! It is possible to show multiple plots in the same axis environment by
//! creating an [`Axis`] and adding plots to it. An [`Axis`] and its individual
//! [`Plot2D`]s are customized by [`AxisKey`]s and [`PlotKey`]s respectively.

// Only imported for documentation. If you notice that this is no longer the
// case, please change it.
#[allow(unused_imports)]
use crate::axis::{
    plot::{Plot2D, PlotKey},
    AxisKey,
};

use crate::axis::Axis;
use std::fmt;

#[cfg(feature = "inclusive")]
use std::io::Write;

/// Axis environment inside a [`Picture`].
pub mod axis;

/// The error type returned when showing a figure fails.
#[cfg(feature = "inclusive")]
#[derive(Clone, Copy, Debug)]
pub enum ShowPdfError {
    /// Compilation of LaTeX source failed internally
    Compile,
    /// Creating or writing to temporary file failed
    Write,
    /// Persisting the temporary file failed
    Persist,
    /// Opening the file failed
    Open,
}
#[cfg(feature = "inclusive")]
impl fmt::Display for ShowPdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ShowPdfError::Compile => write!(f, "tectonic compilation error"),
            ShowPdfError::Write => write!(f, "creating or writing to temporary file failed"),
            ShowPdfError::Persist => write!(f, "persisting temporary file failed"),
            ShowPdfError::Open => write!(f, "opening file error"),
        }
    }
}
#[cfg(feature = "inclusive")]
impl From<tectonic::errors::Error> for ShowPdfError {
    fn from(_: tectonic::errors::Error) -> Self {
        Self::Compile
    }
}
#[cfg(feature = "inclusive")]
impl From<std::io::Error> for ShowPdfError {
    fn from(_: std::io::Error) -> Self {
        Self::Write
    }
}
#[cfg(feature = "inclusive")]
impl From<tempfile::PersistError> for ShowPdfError {
    fn from(_: tempfile::PersistError) -> Self {
        Self::Persist
    }
}
#[cfg(feature = "inclusive")]
impl From<opener::OpenError> for ShowPdfError {
    fn from(_: opener::OpenError) -> Self {
        Self::Open
    }
}

/// Ti*k*Z options passed to the [`Picture`] environment.
///
/// The most commonly used key-value pairs are variants of the [`PictureKey`]
/// enum. The [`PictureKey::Custom`] variant is provided to add unimplemented
/// keys and will be written verbatim in the options of the [`Picture`]
/// environment.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PictureKey {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the [`Picture`].
    Custom(String),
}

impl fmt::Display for PictureKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PictureKey::Custom(key) => write!(f, "{key}"),
        }
    }
}

/// Picture environment.
///
/// Creating a [`Picture`] is equivalent to the Ti*k*Z graphics environment:
///
/// ```text
/// \begin{tikzpicture}[PictureKeys]
///     % axis environments
/// \end{tikzpicture}
/// ```
///
/// You will rarely interact with a [`Picture`]. It is only useful to generate
/// complex layouts with multiple axis environments.
#[derive(Clone, Debug, Default)]
pub struct Picture {
    keys: Vec<PictureKey>,
    pub axes: Vec<Axis>,
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{tikzpicture}}")?;
        // If there are keys, print one per line. It makes it easier for a
        // human later to find keys if they are divided by lines.
        if !self.keys.is_empty() {
            writeln!(f, "[")?;
            for key in self.keys.iter() {
                writeln!(f, "\t{key},")?;
            }
            write!(f, "]")?;
        }
        writeln!(f)?;

        for axis in self.axes.iter() {
            writeln!(f, "{axis}")?;
        }

        write!(f, "\\end{{tikzpicture}}")?;

        Ok(())
    }
}

impl Picture {
    /// Create a new, empty picture environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::Picture;
    ///
    /// let mut picture = Picture::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Add a key to control the appearance of the picture. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::{Picture, PictureKey};
    ///
    /// let mut picture = Picture::new();
    /// picture.add_key(PictureKey::Custom(String::from("baseline")));
    /// ```
    pub fn add_key(&mut self, key: PictureKey) {
        match key {
            PictureKey::Custom(_) => (),
            // If/whenever another variant is added, handle it the same way as
            // Axis::add_key and Plot2D::add_key
        }
        self.keys.push(key);
    }
    /// Return a [`String`] with valid LaTeX code that generates a standalone
    /// PDF with the picture environment.
    ///
    /// # Note
    ///
    /// Passing this string directly to e.g. `pdflatex` will fail to generate a
    /// PDF document. It is usually necessary to [`str::replace`] all the
    /// occurrences of `\n` and `\t` with white space before sending this string
    /// as an argument to a LaTeX compiler.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::Picture;
    ///
    /// let mut picture = Picture::new();
    /// assert_eq!(
    /// r#"\documentclass{standalone}
    /// \usepackage{pgfplots}
    /// \begin{document}
    /// \begin{tikzpicture}
    /// \end{tikzpicture}
    /// \end{document}"#,
    /// picture.standalone_string());
    /// ```
    pub fn standalone_string(&self) -> String {
        String::from("\\documentclass{standalone}\n")
            + "\\usepackage{pgfplots}\n"
            + "\\begin{document}\n"
            + &self.to_string()
            + "\n\\end{document}"
    }
    /// Show the picture as a standalone PDF. This will create a file in the
    /// location returned by [`std::env::temp_dir()`] and open it with the
    /// default PDF viewer in your system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pgfplots::Picture;
    ///
    /// let mut picture = Picture::new();
    /// picture.show();
    /// ```
    #[cfg(feature = "inclusive")]
    pub fn show(&self) -> Result<(), ShowPdfError> {
        let pdf_data = tectonic::latex_to_pdf(self.standalone_string())?;

        let mut file = tempfile::NamedTempFile::new()?;
        file.write_all(&pdf_data)?;
        let (_file, path) = file.keep()?;

        opener::open(&path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
