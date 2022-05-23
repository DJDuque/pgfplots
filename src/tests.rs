use super::*;
use crate::axis::plot::Plot2D;

#[test]
fn picture_key_custom_to_string() {
    assert_eq!(
        PictureKey::Custom(String::from("something/random here")).to_string(),
        String::from("something/random here")
    );
}

#[test]
fn picture_new() {
    let picture = Picture::new();
    assert!(picture.axes.is_empty());
    assert!(picture.keys.is_empty());
}

#[test]
fn picture_add_key() {
    let mut picture = Picture::new();
    picture.add_key(PictureKey::Custom(String::from("random")));
    assert_eq!(picture.keys.len(), 1);
    assert_eq!(picture.keys[0].to_string(), String::from("random"));

    picture.add_key(PictureKey::Custom(String::from("random")));
    assert_eq!(picture.keys.len(), 2);
    assert_eq!(picture.keys[0].to_string(), String::from("random"));
    assert_eq!(picture.keys[1].to_string(), String::from("random"));
}

#[test]
fn picture_to_string() {
    let mut picture = Picture::new();
    assert_eq!(
        picture.to_string(),
        "\\begin{tikzpicture}\n\\end{tikzpicture}"
    );

    picture.add_key(PictureKey::Custom(String::from("baseline")));
    assert_eq!(
        picture.to_string(),
        "\\begin{tikzpicture}[\n\tbaseline,\n]\n\\end{tikzpicture}"
    );

    picture.keys.clear();
    let mut axis = Axis::new();
    picture.axes.push(axis.clone());
    assert_eq!(
        picture.to_string(),
        "\\begin{tikzpicture}\n\\begin{axis}\n\\end{axis}\n\\end{tikzpicture}"
    );

    picture.add_key(PictureKey::Custom(String::from("baseline")));
    picture.add_key(PictureKey::Custom(String::from("scale=2")));
    axis.plots.push(Plot2D::new());
    picture.axes.push(axis.clone());
    assert_eq!(picture.to_string(), "\\begin{tikzpicture}[\n\tbaseline,\n\tscale=2,\n]\n\\begin{axis}\n\\end{axis}\n\\begin{axis}\n\t\\addplot[] coordinates {\n\t};\n\\end{axis}\n\\end{tikzpicture}");
}
