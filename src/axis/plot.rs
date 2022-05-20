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
pub enum PlotKey {
    /// Custom key-value pairs that have not been implemented. These will be
    /// appended verbatim to the options of the `\addplot[...]` command.
    Custom(String),
}

impl fmt::Display for PlotKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlotKey::Custom(key) => write!(f, "{key}"),
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

        // Need to implement Display for each coordinate
        todo!();

        write!(f, "}};")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
