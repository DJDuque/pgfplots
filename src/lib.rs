#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
//! A Rust library to generate publication-quality figures.
//!
//! This crate is a PGFPlots code generator, and provides utilities to create,
//! customize, and compile high-quality plots. The `tectonic` feature allows
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
//! # use pgfplots::ShowPdfError;
//! # fn main() -> Result<(), ShowPdfError> {
//! use pgfplots::{axis::plot::Plot2D, Engine, Picture};
//!
//! let mut plot = Plot2D::new();
//! plot.coordinates = (-100..100)
//!     .into_iter()
//!     .map(|i| (f64::from(i), f64::from(i*i)).into())
//!     .collect();
//!
//! // The `Engine::PdfLatex` variant requires a working LaTeX installation with
//! // the `pgfplots` package installed.
//! // The `Engine::Tectonic` variant (enabled by the `tectonic` feature) allows
//! // users to fully process figures without relying on any externally
//! // installed software.
//! Picture::from(plot).show_pdf(Engine::PdfLatex)?;
//! # Ok(())
//! # }
//! ```
//!
//! It is possible to show multiple plots in the same axis environment by
//! creating an [`Axis`] and adding plots to it. An [`Axis`] and its individual
//! [`Plot2D`]s are customized by [`AxisKey`]s and [`PlotKey`]s respectively.

// Only imported for documentation. If you notice that this is no longer the
// case, please change it.
#[allow(unused_imports)]
use crate::axis::{plot::PlotKey, AxisKey};

use crate::axis::{plot::Plot2D, Axis};
use rand::distributions::{Alphanumeric, DistString};
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use tempfile::NamedTempFile;
use thiserror::Error;

/// Axis environment inside a [`Picture`].
pub mod axis;

/// Engine to compile a [`Picture`] into a PDF.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Engine {
    /// `Pdflatex` engine (requires `pdflatex` to be installed).
    PdfLatex,
    #[cfg(feature = "tectonic")]
    /// `Tectonic` engine (does not require any external software).
    Tectonic,
}

/// The error type returned when a [`Picture`] fails to compile into a PDF.
#[derive(Debug, Error)]
pub enum CompileError {
    /// I/O error.
    #[error("io error")]
    IoError(#[from] std::io::Error),
    /// Compilation was executed but returned a non-zero exit code.
    #[error("compilation failed with status {status}")]
    BadExitCode { status: ExitStatus },
    #[cfg(feature = "tectonic")]
    /// Tectonic error.
    #[error("tectonic error")]
    TectonicError(#[from] tectonic::errors::Error),
}

/// The error type returned when showing a [`Picture`] fails.
#[derive(Debug, Error)]
pub enum ShowPdfError {
    /// Compilation error.
    #[error("compilation error")]
    BadCompilation(#[from] CompileError),
    /// Opening the PDF failed.
    #[error("opening the pdf failed")]
    OpenerError(#[from] opener::OpenError),
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

impl From<Axis> for Picture {
    fn from(axis: Axis) -> Self {
        Self {
            keys: Vec::new(),
            axes: vec![axis],
        }
    }
}
impl From<Plot2D> for Picture {
    fn from(plot: Plot2D) -> Self {
        Picture::from(Axis::from(plot))
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
    /// let picture = Picture::new();
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
    /// let picture = Picture::new();
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
    /// Compile the picture environment into a standalone PDF document. This
    /// will create the file `jobname.pdf` in the specified `working_dir`
    /// (additional files will be created in the same directory e.g. `.log` and
    /// `.aux` files). Return a [`Result`] with the path to the generated PDF
    /// file or a [`CompileError`].
    ///
    /// # Examples
    ///
    // Example is `no_run` because `std::env::temp_dir` causes the test to fail
    // running GitHub Actions.
    /// ```no_run
    /// # use pgfplots::CompileError;
    /// # fn main() -> Result<(), CompileError> {
    /// use pgfplots::{Engine, Picture};
    ///
    /// let picture = Picture::new();
    /// let pdf_path = picture.to_pdf(std::env::temp_dir(), "jobname", Engine::PdfLatex)?;
    ///
    /// assert_eq!(pdf_path, std::env::temp_dir().join("jobname.pdf"));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_pdf<P, S>(
        &self,
        working_dir: P,
        jobname: S,
        engine: Engine,
    ) -> Result<PathBuf, CompileError>
    where
        P: AsRef<Path>,
        // str instead of OsStr because of Tectonic's `tex_input_file`
        S: AsRef<str>,
    {
        // Copy the tex code to a temporary file instead of passing it directly
        // to the engine via e.g. stdin. This avoids the "Argument list too
        // long" error when there are e.g. too many points in a plot.
        let mut tex_file = NamedTempFile::new()?;
        tex_file.write_all(self.standalone_string().as_bytes())?;

        match engine {
            Engine::PdfLatex => {
                let status = Command::new("pdflatex")
                    .current_dir(working_dir.as_ref())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .arg("-interaction=batchmode")
                    .arg("-halt-on-error")
                    .arg(String::from("-jobname=") + jobname.as_ref())
                    .arg(tex_file.path())
                    .status()?;

                if !status.success() {
                    return Err(CompileError::BadExitCode { status });
                }
            }
            #[cfg(feature = "tectonic")]
            // Modified from `tectonic::latex_to_pdf` to generate the files
            // instead of just returning the bytes.
            Engine::Tectonic => {
                let mut status = tectonic::status::NoopStatusBackend::default();

                let auto_create_config_file = false;
                let config = tectonic::ctry!(tectonic::config::PersistentConfig::open(auto_create_config_file);
                       "failed to open the default configuration file");

                let only_cached = false;
                let bundle = tectonic::ctry!(config.default_bundle(only_cached, &mut status);
                       "failed to load the default resource bundle");

                let format_cache_path = tectonic::ctry!(config.format_cache_path();
                                  "failed to set up the format cache");

                let mut sb = tectonic::driver::ProcessingSessionBuilder::default();
                sb.bundle(bundle)
                    .primary_input_path(tex_file.path())
                    .tex_input_name(jobname.as_ref())
                    .format_name("latex")
                    .format_cache_path(format_cache_path)
                    // Just to keep the behaviour consistent with `pdflatex`
                    .keep_logs(true)
                    .keep_intermediates(true)
                    .print_stdout(false)
                    .output_format(tectonic::driver::OutputFormat::Pdf)
                    .output_dir(working_dir.as_ref());

                let mut sess = tectonic::ctry!(sb.create(&mut status); "failed to initialize the LaTeX processing session");
                tectonic::ctry!(sess.run(&mut status); "the LaTeX engine failed");
            }
        }
        Ok(working_dir
            .as_ref()
            .join(String::from(jobname.as_ref()) + ".pdf"))
    }
    /// Show the picture environment in a standalone PDF document. This will
    /// create a file in the location returned by [`std::env::temp_dir`] and
    /// open it with the default PDF viewer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use pgfplots::ShowPdfError;
    /// # fn main() -> Result<(), ShowPdfError> {
    /// use pgfplots::{Engine, Picture};
    ///
    /// let picture = Picture::new();
    /// picture.show_pdf(Engine::PdfLatex)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn show_pdf(&self, engine: Engine) -> Result<(), ShowPdfError> {
        // Return a random string that can be used as a `jobname` to compile a
        // [`Picture`] in `std::env::temp_dir()`. This should not overwrite
        // any existing files.
        fn random_jobname() -> String {
            loop {
                let mut jobname = "pgfplots_".to_string();
                Alphanumeric.append_string(&mut rand::thread_rng(), &mut jobname, 8);
                let pdf_path = std::env::temp_dir().join(jobname.clone() + ".pdf");
                let log_path = std::env::temp_dir().join(jobname.clone() + ".log");
                let aux_path = std::env::temp_dir().join(jobname.clone() + ".aux");
                if !pdf_path.exists() && !log_path.exists() && !aux_path.exists() {
                    return jobname;
                }
            }
        }

        let jobname = random_jobname();
        let pdf_path = self.to_pdf(std::env::temp_dir(), &jobname, engine)?;
        opener::open(pdf_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
