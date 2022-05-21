use crate::axis::plot::coordinate::Coordinate2D;
use std::fmt;

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
#[derive(Clone, Debug)]
pub struct Plot2D {
    keys: Vec<PlotKey>,
    coordinates: Vec<Coordinate2D>,
}

impl fmt::Display for Plot2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\addplot[")?;
        // If there are keys, print them one per line. It makes it easier for a
        // human to find individual keys later.
        if !self.keys.is_empty() {
            write!(f, "\n")?;
            for key in self.keys.iter() {
                write!(f, "\t{key},\n")?;
            }
        }
        write!(f, "] coordinates {{\n")?;

        for coordinate in self.coordinates.iter() {
            write!(f, "\t{coordinate}\n")?;
        }

        write!(f, "}};")?;

        Ok(())
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
