use super::*;
use crate::axis::plot::{PlotKey, *};

#[test]
fn scale_to_string() {
    assert_eq!(Scale::Log.to_string(), String::from("log"));
    assert_eq!(Scale::Normal.to_string(), String::from("normal"));
}

#[test]
fn axis_key_custom_to_string() {
    assert_eq!(
        AxisKey::Custom(String::from("something/random here")).to_string(),
        String::from("something/random here")
    );
}

#[test]
fn axis_key_x_mode_to_string() {
    assert_eq!(
        AxisKey::XMode(Scale::Log).to_string(),
        String::from("xmode=log")
    );
    assert_eq!(
        AxisKey::XMode(Scale::Normal).to_string(),
        String::from("xmode=normal")
    );
}

#[test]
fn axis_key_y_mode_to_string() {
    assert_eq!(
        AxisKey::YMode(Scale::Log).to_string(),
        String::from("ymode=log")
    );
    assert_eq!(
        AxisKey::YMode(Scale::Normal).to_string(),
        String::from("ymode=normal")
    );
}

#[test]
fn axis_new() {
    let axis = Axis::new();
    assert!(axis.plots.is_empty());
    assert!(axis.keys.is_empty());
}

#[test]
fn axis_add_key() {
    let mut axis = Axis::new();
    axis.add_key(AxisKey::YMode(Scale::Log));
    assert_eq!(axis.keys.len(), 1);
    assert_eq!(axis.keys[0].to_string(), String::from("ymode=log"));

    axis.add_key(AxisKey::YMode(Scale::Log));
    assert_eq!(axis.keys.len(), 1);
    assert_eq!(axis.keys[0].to_string(), String::from("ymode=log"));

    axis.add_key(AxisKey::XMode(Scale::Log));
    assert_eq!(axis.keys.len(), 2);
    assert_eq!(axis.keys[0].to_string(), String::from("ymode=log"));
    assert_eq!(axis.keys[1].to_string(), String::from("xmode=log"));

    axis.add_key(AxisKey::Custom(String::from("random")));
    assert_eq!(axis.keys.len(), 3);
    assert_eq!(axis.keys[0].to_string(), String::from("ymode=log"));
    assert_eq!(axis.keys[1].to_string(), String::from("xmode=log"));
    assert_eq!(axis.keys[2].to_string(), String::from("random"));

    axis.add_key(AxisKey::Custom(String::from("random")));
    assert_eq!(axis.keys.len(), 4);
    assert_eq!(axis.keys[0].to_string(), String::from("ymode=log"));
    assert_eq!(axis.keys[1].to_string(), String::from("xmode=log"));
    assert_eq!(axis.keys[2].to_string(), String::from("random"));
    assert_eq!(axis.keys[3].to_string(), String::from("random"));

    axis.add_key(AxisKey::YMode(Scale::Log));
    assert_eq!(axis.keys.len(), 4);
    assert_eq!(axis.keys[0].to_string(), String::from("xmode=log"));
    assert_eq!(axis.keys[1].to_string(), String::from("random"));
    assert_eq!(axis.keys[2].to_string(), String::from("random"));
    assert_eq!(axis.keys[3].to_string(), String::from("ymode=log"));

    axis.add_key(AxisKey::XMode(Scale::Log));
    assert_eq!(axis.keys.len(), 4);
    assert_eq!(axis.keys[0].to_string(), String::from("random"));
    assert_eq!(axis.keys[1].to_string(), String::from("random"));
    assert_eq!(axis.keys[2].to_string(), String::from("ymode=log"));
    assert_eq!(axis.keys[3].to_string(), String::from("xmode=log"));
}

#[test]
fn axis_to_string() {
    let mut axis = Axis::new();
    assert_eq!(axis.to_string(), "\\begin{axis}\n\\end{axis}");

    axis.add_key(AxisKey::YMode(Scale::Log));
    assert_eq!(
        axis.to_string(),
        "\\begin{axis}[\n\tymode=log,\n]\n\\end{axis}"
    );

    axis.keys.clear();
    let mut plot = Plot2D::new();
    axis.plots.push(plot.clone());
    assert_eq!(
        axis.to_string(),
        "\\begin{axis}\n\t\\addplot[] coordinates {\n\t};\n\\end{axis}"
    );

    axis.add_key(AxisKey::YMode(Scale::Log));
    axis.add_key(AxisKey::XMode(Scale::Log));
    plot.coordinates.push((1.0, -1.0, None, Some(5.0)).into());
    plot.coordinates.push((1.0, -1.0, None, None).into());
    plot.add_key(PlotKey::XError(ErrorCharacter::Absolute));
    plot.add_key(PlotKey::XErrorDirection(ErrorDirection::Both));
    axis.plots.push(plot);
    assert_eq!(axis.to_string(), "\\begin{axis}[\n\tymode=log,\n\txmode=log,\n]\n\t\\addplot[] coordinates {\n\t};\n\t\\addplot[\n\t\terror bars/x explicit,\n\t\terror bars/x dir=both,\n\t] coordinates {\n\t\t(1,-1)\t+- (0,5)\n\t\t(1,-1)\n\t};\n\\end{axis}");
}
