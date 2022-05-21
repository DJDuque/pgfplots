use std::fmt;

// Only imported for documentation. If you notice this is no longer the case,
// please change it.
#[allow(unused_imports)]
use crate::axis::plot::{Plot2D, PlotKey};

/// Coordinate in a two-dimensional plot.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
    /// By default, error bars are not drawn (even if it is a [`Some`]). These
    /// are only drawn if both [`PlotKey::XError`] and
    /// [`PlotKey::XErrorDirection`] are set in the [`Plot2D`].
    pub error_x: Option<f64>,
    /// By default, error bars are not drawn (even if it is a [`Some`]). These
    /// are only drawn if both [`PlotKey::YError`] and
    /// [`PlotKey::YErrorDirection`] are set in the [`Plot2D`].
    pub error_y: Option<f64>,
    // What to do when `point meta=explicit` in plot?
    // Should we add an Option<point_meta> here?
    // Is `point meta` skipped same as error when it is not set?
}

impl fmt::Display for Coordinate2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)?;

        if self.error_x.is_some() || self.error_y.is_some() {
            let error_x = self.error_x.unwrap_or(0.0);
            let error_y = self.error_y.unwrap_or(0.0);
            write!(f, "\t+- ({error_x},{error_y})")?;
        }

        Ok(())
    }
}

impl From<(f64, f64)> for Coordinate2D {
    /// Conversion from an `(x,y)` tuple into a two-dimensional coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::plot::coordinate::Coordinate2D;
    ///
    /// let point: Coordinate2D = (1.0, -1.0).into();
    ///
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, -1.0);
    /// assert!(point.error_x.is_none());
    /// assert!(point.error_y.is_none());
    /// ```
    fn from(coordinate: (f64, f64)) -> Self {
        Coordinate2D {
            x: coordinate.0,
            y: coordinate.1,
            error_x: None,
            error_y: None,
        }
    }
}

impl From<(f64, f64, Option<f64>, Option<f64>)> for Coordinate2D {
    /// Conversion from an `(x,y,error_x,error_y)` tuple into a two-dimensional
    /// coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use pgfplots::axis::plot::coordinate::Coordinate2D;
    ///
    /// let point: Coordinate2D = (1.0, -1.0, None, Some(3.0)).into();
    ///
    /// assert_eq!(point.x, 1.0);
    /// assert_eq!(point.y, -1.0);
    /// assert!(point.error_x.is_none());
    /// assert_eq!(point.error_y.unwrap(), 3.0);
    /// ```
    fn from(coordinate: (f64, f64, Option<f64>, Option<f64>)) -> Self {
        Coordinate2D {
            x: coordinate.0,
            y: coordinate.1,
            error_x: coordinate.2,
            error_y: coordinate.3,
        }
    }
}

#[cfg(test)]
mod tests;
