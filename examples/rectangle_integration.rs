use pgfplots::{
    axis::{plot::*, *},
    Engine, Picture,
};

fn main() {
    // Set line
    let mut line = Plot2D::new();
    line.coordinates = (0..101)
        .into_iter()
        .map(|i| (f64::from(i), f64::from(i * i)).into())
        .collect();

    // Set rectangles
    let mut rectangles = Plot2D::new();
    rectangles.coordinates = (0..101)
        .into_iter()
        .step_by(10)
        .map(|i| (f64::from(i), f64::from(i * i)).into())
        .collect();
    // Currently have to "guess" the bar width in pt units
    // Still pending the \compat key
    rectangles.add_key(PlotKey::Type2D(Type2D::YBar {
        bar_width: 19.5,
        bar_shift: 0.0,
    }));
    rectangles.add_key(PlotKey::Custom(String::from("fill=gray!20")));
    rectangles.add_key(PlotKey::Custom(String::from("draw opacity=0.5")));

    // Customise axis environment
    let mut axis = Axis::new();
    axis.set_title("Rectangle Integration");
    axis.set_x_label("$x$");
    axis.set_y_label("$y = x^2$");
    axis.plots.push(rectangles);
    axis.plots.push(line);
    axis.add_key(AxisKey::Custom(String::from("axis lines=middle")));
    axis.add_key(AxisKey::Custom(String::from("xlabel near ticks")));
    axis.add_key(AxisKey::Custom(String::from("ylabel near ticks")));

    #[cfg(feature = "tectonic")]
    Picture::from(axis).show_pdf(Engine::Tectonic).unwrap();
    #[cfg(not(feature = "tectonic"))]
    Picture::from(axis).show_pdf(Engine::PdfLatex).unwrap();
}
