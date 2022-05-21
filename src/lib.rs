//! A Rust library that generates PGFPlots code to `\input` into LaTeX documents.

use crate::axis::Axis;
use std::fmt;

/// Axis environment inside a [`Picture`].
pub mod axis;

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
///     % contents
/// \end{tikzpicture}
/// ```
#[derive(Clone, Debug)]
pub struct Picture {
    keys: Vec<PictureKey>,
    axes: Vec<Axis>,
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\begin{{tikzpicture}}")?;
        // If there are keys, print one per line. It makes it easier for a
        // human later to find keys if they are divided by lines.
        if !self.keys.is_empty() {
            write!(f, "[\n")?;
            for key in self.keys.iter() {
                write!(f, "\t{key},\n")?;
            }
            write!(f, "]")?;
        }
        write!(f, "\n")?;

        for axis in self.axes.iter() {
            write!(f, "\t{axis}\n")?;
        }

        write!(f, "\\end{{tikzpicture}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
