#[derive(Clone, Copy, Debug)]
pub struct Coordinate2D {
    x: f64,
    y: f64,
    err_x: Option<f64>,
    err_y: Option<f64>,
}

#[cfg(test)]
mod tests;
