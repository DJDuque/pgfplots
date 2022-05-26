use crate::{axis::plot::Plot2D, Picture};
use std::fmt;

/// Plot inside an [`Axis`] environment.
pub mod plot;

/// PGFPlots options passed to the [`Axis`] environment.
///
/// The most commonly used key-value pairs are variants of the [`AxisKey`] enum.
/// The [`AxisKey::Custom`] variant is provided to add unimplemented keys and
/// will be written verbatim in the options of the [`Axis`] environment.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AxisKey {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the [`Axis`].
    Custom(String),
    /// Control the scaling of the *x* axis.
    XMode(Scale),
    /// Control the scaling of the *y* axis.
    YMode(Scale),
    /// Control the title of the axis environment.
    Title(String),
    /// Control the label of the *x* axis.
    XLabel(String),
    /// Control the label of the *y* axis.
    YLabel(String),
}

impl fmt::Display for AxisKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AxisKey::Custom(key) => write!(f, "{key}"),
            AxisKey::XMode(value) => write!(f, "xmode={value}"),
            AxisKey::YMode(value) => write!(f, "ymode={value}"),
            AxisKey::Title(value) => write!(f, "title={{{value}}}"),
            AxisKey::XLabel(value) => write!(f, "xlabel={{{value}}}"),
            AxisKey::YLabel(value) => write!(f, "ylabel={{{value}}}"),
        }
    }
}

/// Axis environment inside a [`Picture`].
///
/// An [`Axis`] is equivalent to the PGFPlots axis environment:
///
/// ```text
/// \begin{axis}[AxisKeys]
///     % plots
/// \end{axis}
/// ```
///
/// # Examples
///
/// ```no_run
/// use pgfplots::axis::Axis;
///
/// let mut axis = Axis::new();
/// axis.set_title("Picture of $\\gamma$ rays");
/// axis.set_x_label("$x$~[m]");
/// axis.set_y_label("$y$~[m]");
/// ```
#[derive(Clone, Debug, Default)]
pub struct Axis {
    keys: Vec<AxisKey>,
    pub plots: Vec<Plot2D>,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{axis}}")?;
        // If there are keys, print one per line. It makes it easier for a
        // human to find individual keys later.
        if !self.keys.is_empty() {
            writeln!(f, "[")?;
            for key in self.keys.iter() {
                writeln!(f, "\t{key},")?;
            }
            write!(f, "]")?;
        }
        writeln!(f)?;

        for plot in self.plots.iter() {
            writeln!(f, "{plot}")?;
        }

        write!(f, "\\end{{axis}}")?;

        Ok(())
    }
}

impl Axis {
    /// Creates a new, empty axis environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the title of the axis environment. This can be valid LaTeX e.g.
    /// inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_title("My plot: $y = x^2$");
    /// ```
    pub fn set_title(&mut self, title: &str) {
        self.add_key(AxisKey::Title(String::from(title)));
    }
    /// Set the label of the *x* axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_x_label("$x$~[m]");
    /// ```
    pub fn set_x_label(&mut self, label: &str) {
        self.add_key(AxisKey::XLabel(String::from(label)));
    }
    /// Set the label of the *y* axis. This can be valid LaTeX e.g. inline math.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// axis.set_y_label("$y$~[m]");
    /// ```
    pub fn set_y_label(&mut self, label: &str) {
        self.add_key(AxisKey::YLabel(String::from(label)));
    }
    /// Add a key to control the appearance of the axis. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::{Axis, AxisKey, Scale::Log};
    ///
    /// let mut axis = Axis::new();
    /// axis.add_key(AxisKey::YMode(Log));
    /// ```
    pub fn add_key(&mut self, key: AxisKey) {
        match key {
            AxisKey::Custom(_) => (),
            _ => {
                if let Some(index) = self
                    .keys
                    .iter()
                    .position(|k| std::mem::discriminant(k) == std::mem::discriminant(&key))
                {
                    self.keys.remove(index);
                }
            }
        }
        self.keys.push(key);
    }
    /// Return a [`String`] with valid LaTeX code that generates a standalone
    /// PDF with the axis in a default picture environment.
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
    /// use pgfplots::axis::Axis;
    ///
    /// let mut axis = Axis::new();
    /// assert_eq!(
    /// r#"\documentclass{standalone}
    /// \usepackage{pgfplots}
    /// \begin{document}
    /// \begin{tikzpicture}
    /// \begin{axis}
    /// \end{axis}
    /// \end{tikzpicture}
    /// \end{document}"#,
    /// axis.standalone_string());
    /// ```
    pub fn standalone_string(&self) -> String {
        let mut picture = Picture::new();
        picture.axes.push(self.clone());
        picture.standalone_string()
    }
}

/// Control the scaling of an axis.
#[derive(Clone, Copy, Debug)]
pub enum Scale {
    /// Logarithmic scaling i.e. apply the natural logarithm to each coordinate.
    Log,
    /// Linear scaling of the coordinates.
    Normal,
}
impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Log => write!(f, "log"),
            Scale::Normal => write!(f, "normal"),
        }
    }
}

#[cfg(test)]
mod tests;
