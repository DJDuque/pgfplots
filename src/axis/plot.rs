use crate::axis::plot::coordinate::Coordinate2D;
use std::fmt;

// Only imported for documentation. If you notice that this is no longer the
// case, please change it.
#[allow(unused_imports)]
use crate::Picture;

/// Coordinates inside a plot.
pub mod coordinate;

/// PGFPlots options passed to a plot.
///
/// The most commonly used key-value pairs are variants of the [`PlotKey`] enum.
/// The [`PlotKey::Custom`] variant is provided to add unimplemented keys and
/// will be written verbatim in the options of the `\addplot[...]` command.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PlotKey {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the `\addplot[...]` command.
    Custom(String),
    /// Control the type of two dimensional plots.
    Type2D(Type2D),
    /// Control the character (absolute or relative) of the error bars of the
    /// *x* coordinates. Note that error bars won't be drawn unless
    /// [`PlotKey::XErrorDirection`] is also set.
    XError(ErrorCharacter),
    /// Control the direction of the error bars of the *x* coordinates.
    /// Note that error bars won't be drawn unless [`PlotKey::XError`] is also
    /// set.
    XErrorDirection(ErrorDirection),
    /// Control the character (absolute or relative) of the error bars of the
    /// *y* coordinates. Note that error bars won't be drawn unless
    /// [`PlotKey::YErrorDirection`] is also set.
    YError(ErrorCharacter),
    /// Control the direction of the error bars of the *y* coordinates.
    /// Note that error bars won't be drawn unless [`PlotKey::YError`] is also
    /// set.
    YErrorDirection(ErrorDirection),
}

impl fmt::Display for PlotKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlotKey::Custom(key) => write!(f, "{key}"),
            PlotKey::Type2D(value) => write!(f, "{value}"),
            PlotKey::XError(value) => write!(f, "error bars/x {value}"),
            PlotKey::XErrorDirection(value) => write!(f, "error bars/x dir={value}"),
            PlotKey::YError(value) => write!(f, "error bars/y {value}"),
            PlotKey::YErrorDirection(value) => write!(f, "error bars/y dir={value}"),
        }
    }
}

/// Two-dimensional plot inside an [`Axis`].
///
/// Adding a [`Plot2D`] to an [`Axis`] environment is equivalent to:
///
/// ```text
/// \addplot[PlotKeys]
///     % coordinates;
/// ```
///
/// # Examples
///
/// ```no_run
/// use pgfplots::axis::plot::Plot2D;
///
/// let mut plot = Plot2D::new();
/// plot.coordinates = (-100..100)
///     .into_iter()
///     .map(|i| (f64::from(i), f64::from(i*i)).into())
///     .collect();
///
/// # #[cfg(feature = "inclusive")]
/// plot.show();
/// ```
#[derive(Clone, Debug, Default)]
pub struct Plot2D {
    keys: Vec<PlotKey>,
    pub coordinates: Vec<Coordinate2D>,
}

impl fmt::Display for Plot2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t\\addplot[")?;
        // If there are keys, print them one per line. It makes it easier for a
        // human to find individual keys later.
        if !self.keys.is_empty() {
            writeln!(f)?;
            for key in self.keys.iter() {
                writeln!(f, "\t\t{key},")?;
            }
            write!(f, "\t")?;
        }
        writeln!(f, "] coordinates {{")?;

        for coordinate in self.coordinates.iter() {
            writeln!(f, "\t\t{coordinate}")?;
        }

        write!(f, "\t}};")?;

        Ok(())
    }
}

impl Plot2D {
    /// Creates a new, empty two-dimensional plot.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::plot::Plot2D;
    ///
    /// let mut plot = Plot2D::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
    /// Add a key to control the appearance of the plot. This will overwrite
    /// any previous mutually exclusive key.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::plot::{Plot2D, PlotKey, Type2D::SharpPlot};
    ///
    /// let mut plot = Plot2D::new();
    /// plot.add_key(PlotKey::Type2D(SharpPlot));
    /// ```
    pub fn add_key(&mut self, key: PlotKey) {
        match key {
            PlotKey::Custom(_) => (),
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
}

/// Control the type of two dimensional plots.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Type2D {
    /// Coordinates are simply connected by straight lines.
    SharpPlot,
    /// Interpolate smoothly between successive points. The `tension` controls
    /// how "smooth" a plot is; recommended initial value is `Type2D::Smooth{
    /// tension: 0.55 }`. A higher value results in more "round" curves.
    Smooth { tension: f64 },
    /// Coordinates are connected with horizontal and vertical lines. Marks are
    /// placed to the left of each horizontal line.
    ConstLeft,
    /// Coordinates are connected with horizontal and vertical lines. Marks are
    /// placed to the right of each horizontal line.
    ConstRight,
    /// Coordinates are connected with horizontal and vertical lines. Marks are
    /// placed to the middle of each horizontal line.
    ConstMid,
    /// Variant of [`Type2D::ConstLeft`] which does not draw vertical lines.
    JumpLeft,
    /// Variant of [`Type2D::ConstRight`] which does not draw vertical lines.
    JumpRight,
    /// Variant of [`Type2D::ConstMid`] which does not draw vertical lines.
    JumpMid,
    /// Draw horizontal bars between the *y = 0* line and each coordinate. The
    /// `bar_width` field controls the width of the horizontal bars, and
    /// `bar_shift` controls the vertical shift. Unless you are plotting
    /// multiple bars in the same [`Axis`], you most likely want
    /// `bar_shift: 0.0`.
    ///
    /// # Note
    ///
    /// By default, `bar_width` and `bar_shift` are assumed to be in `pt` units.
    /// If you want them to be interpreted as axis units (this is most likely
    /// what you want), you need to add the plot to an [`Axis`], add the
    /// [`Axis`] to a [`Picture`], and set `compat=1.7` or higher on the
    /// [`Picture`].
    XBar { bar_width: f64, bar_shift: f64 },
    /// Draw vertical bars between the *x = 0* line and each coordinate. The
    /// `bar_width` field controls the width of the vertical bars, and
    /// `bar_shift` controls the horizontal shift. Unless you are plotting
    /// multiple bars in the same [`Axis`], you most likely want
    /// `bar_shift: 0.0`.
    ///
    /// # Note
    ///
    /// By default, `bar_width` and `bar_shift` are assumed to be in `pt` units.
    /// If you want them to be interpreted as axis units (this is most likely
    /// what you want), you need to add the plot to an [`Axis`], add the
    /// [`Axis`] to a [`Picture`], and set `compat=1.7` or higher on the
    /// [`Picture`].
    YBar { bar_width: f64, bar_shift: f64 },
    /// Similar to [`Type2D::XBar`] except that it draws a single horizontal
    /// lines instead of rectangles.
    XComb,
    /// Similar to [`Type2D::YBar`] except that it draws a single vertical
    /// lines instead of rectangles.
    YComb,
    /// Draw only markers.
    OnlyMarks,
}
impl fmt::Display for Type2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type2D::SharpPlot => write!(f, "sharp plot"),
            Type2D::Smooth { tension } => write!(f, "smooth, tension={tension}"),
            Type2D::ConstLeft => write!(f, "const plot mark left"),
            Type2D::ConstRight => write!(f, "const plot mark right"),
            Type2D::ConstMid => write!(f, "const plot mark mid"),
            Type2D::JumpLeft => write!(f, "jump mark left"),
            Type2D::JumpRight => write!(f, "jump mark right"),
            Type2D::JumpMid => write!(f, "jump mark mid"),
            Type2D::XBar {
                bar_width,
                bar_shift,
            } => write!(f, "xbar, bar width={bar_width}, bar shift={bar_shift}"),
            Type2D::YBar {
                bar_width,
                bar_shift,
            } => write!(f, "ybar, bar width={bar_width}, bar shift={bar_shift}"),
            Type2D::XComb => write!(f, "xcomb"),
            Type2D::YComb => write!(f, "ycomb"),
            Type2D::OnlyMarks => write!(f, "only marks"),
        }
    }
}

/// Control the character of error bars.
#[derive(Clone, Copy, Debug)]
pub enum ErrorCharacter {
    /// The value of an error (if any) is absolute.
    Absolute,
    /// The value of an error (if any) is relative to the value of the
    /// coordinate.
    Relative,
}
impl fmt::Display for ErrorCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCharacter::Absolute => write!(f, "explicit"),
            ErrorCharacter::Relative => write!(f, "explicit relative"),
        }
    }
}

/// Control the direction of error bars.
#[derive(Clone, Copy, Debug)]
pub enum ErrorDirection {
    /// Draws no error bars.
    None,
    /// Draws only upper bounds in the direction of interest.
    Plus,
    /// Draws only lower bounds in the direction of interest.
    Minus,
    /// Draws upper and lower bounds in the direction of interest.
    Both,
}
impl fmt::Display for ErrorDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorDirection::None => write!(f, "none"),
            ErrorDirection::Plus => write!(f, "plus"),
            ErrorDirection::Minus => write!(f, "minus"),
            ErrorDirection::Both => write!(f, "both"),
        }
    }
}

#[cfg(test)]
mod tests;
