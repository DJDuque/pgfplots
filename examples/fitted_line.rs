use pgfplots::{
    axis::{plot::*, *},
    Engine, Picture,
};
use std::f64::consts::PI;

fn main() {
    // Set the straight dotted line
    let mut line = Plot2D::new();
    line.coordinates = (0..11)
        .into_iter()
        .map(|i| (f64::from(i), 2.0 * PI * f64::from(i)).into())
        .collect();
    line.add_key(PlotKey::Custom(String::from("dashed")));

    // Set the points with error bars
    let mut points = Plot2D::new();
    points
        .coordinates
        .push((1.0, 8.0, Some(0.2), Some(0.9)).into());
    points
        .coordinates
        .push((3.0, 16.0, Some(0.4), Some(1.4)).into());
    points
        .coordinates
        .push((5.0, 33.0, Some(0.2), Some(3.4)).into());
    points
        .coordinates
        .push((7.0, 41.0, Some(0.2), Some(3.4)).into());
    points
        .coordinates
        .push((9.0, 58.0, Some(0.5), Some(1.4)).into());
    points.add_key(PlotKey::Type2D(Type2D::OnlyMarks));
    points.add_key(PlotKey::XError(ErrorCharacter::Absolute));
    points.add_key(PlotKey::XErrorDirection(ErrorDirection::Both));
    points.add_key(PlotKey::YError(ErrorCharacter::Absolute));
    points.add_key(PlotKey::YErrorDirection(ErrorDirection::Both));
    points.add_key(PlotKey::Custom(String::from("mark size=1pt")));

    // Customize axis environment
    let mut axis = Axis::new();
    axis.set_title("Slope is $2\\pi$");
    axis.set_x_label("Radius~[m]");
    axis.set_y_label("Circumference~[m]");
    axis.plots.push(line);
    axis.plots.push(points);
    axis.add_key(AxisKey::Custom(String::from("legend entries={fit,data}")));
    axis.add_key(AxisKey::Custom(String::from("legend pos=north west")));

    #[cfg(feature = "tectonic")]
    Picture::from(axis).show_pdf(Engine::Tectonic).unwrap();
    #[cfg(not(feature = "tectonic"))]
    Picture::from(axis).show_pdf(Engine::PdfLatex).unwrap();
}
