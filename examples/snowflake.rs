use pgfplots::axis::{
    plot::{Plot2D, PlotKey},
    Axis, AxisKey,
};

fn main() {
    let mut vertices = {
        let mut current: Vec<(f64, f64)> = vec![
            (0.0, 1.0),
            ((3.0f64).sqrt() / 2.0, -0.5),
            (-(3.0f64).sqrt() / 2.0, -0.5),
        ];
        for _ in 0..5 {
            current = snowflake_iter(&current[..]);
        }
        current
    };
    vertices.push(vertices[0]);

    let mut plot = Plot2D::new();
    plot.coordinates = vertices.into_iter().map(|v| v.into()).collect();
    plot.add_key(PlotKey::Custom(String::from("fill=gray!20")));

    let mut axis = Axis::new();
    axis.set_title("Kloch Snowflake");
    axis.plots.push(plot);
    axis.add_key(AxisKey::Custom(String::from("hide axis")));

    #[cfg(feature = "inclusive")]
    axis.show().unwrap();
}

// Stolen from plotters crate example
fn snowflake_iter(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut ret = vec![];
    for i in 0..points.len() {
        let (start, end) = (points[i], points[(i + 1) % points.len()]);
        let t = ((end.0 - start.0) / 3.0, (end.1 - start.1) / 3.0);
        let s = (
            t.0 * 0.5 - t.1 * (0.75f64).sqrt(),
            t.1 * 0.5 + (0.75f64).sqrt() * t.0,
        );
        ret.push(start);
        ret.push((start.0 + t.0, start.1 + t.1));
        ret.push((start.0 + t.0 + s.0, start.1 + t.1 + s.1));
        ret.push((start.0 + t.0 * 2.0, start.1 + t.1 * 2.0));
    }
    ret
}
