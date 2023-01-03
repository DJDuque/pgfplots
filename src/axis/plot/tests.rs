use super::*;

#[test]
fn error_direction_to_string() {
    assert_eq!(ErrorDirection::None.to_string(), String::from("none"));
    assert_eq!(ErrorDirection::Plus.to_string(), String::from("plus"));
    assert_eq!(ErrorDirection::Minus.to_string(), String::from("minus"));
    assert_eq!(ErrorDirection::Both.to_string(), String::from("both"));
}

#[test]
fn error_character_to_string() {
    assert_eq!(
        ErrorCharacter::Absolute.to_string(),
        String::from("explicit")
    );
    assert_eq!(
        ErrorCharacter::Relative.to_string(),
        String::from("explicit relative")
    );
}

// This test is here only to let us know if we added an enum variant
// but we forgot to add unit tests for it
//
// If this fails, it is because you added a new variant.
// Please do the following:
// 1) Add a unit test for the new variant you added (see examples below).
// 2) AFTER doing (1), add the new variant to the match.
#[test]
fn plot_type2d_tested() {
    let type_2d = Type2D::OnlyMarks;
    match type_2d {
        Type2D::SharpPlot => (),
        Type2D::Smooth { tension: _ } => (),
        Type2D::ConstLeft => (),
        Type2D::ConstRight => (),
        Type2D::ConstMid => (),
        Type2D::JumpLeft => (),
        Type2D::JumpRight => (),
        Type2D::JumpMid => (),
        Type2D::XBar {
            bar_width: _,
            bar_shift: _,
        } => (),
        Type2D::YBar {
            bar_width: _,
            bar_shift: _,
        } => (),
        Type2D::XComb => (),
        Type2D::YComb => (),
        Type2D::OnlyMarks => (),
    }
}

#[test]
fn type_2d_to_string() {
    assert_eq!(Type2D::SharpPlot.to_string(), String::from("sharp plot"));
    assert_eq!(
        Type2D::Smooth { tension: 0.55 }.to_string(),
        String::from("smooth, tension=0.55")
    );
    assert_eq!(
        Type2D::ConstLeft.to_string(),
        String::from("const plot mark left")
    );
    assert_eq!(
        Type2D::ConstRight.to_string(),
        String::from("const plot mark right")
    );
    assert_eq!(
        Type2D::ConstMid.to_string(),
        String::from("const plot mark mid")
    );
    assert_eq!(Type2D::JumpLeft.to_string(), String::from("jump mark left"));
    assert_eq!(
        Type2D::JumpRight.to_string(),
        String::from("jump mark right")
    );
    assert_eq!(Type2D::JumpMid.to_string(), String::from("jump mark mid"));
    assert_eq!(
        Type2D::XBar {
            bar_width: 0.5,
            bar_shift: 1.0
        }
        .to_string(),
        String::from("xbar, bar width=0.5, bar shift=1")
    );
    assert_eq!(
        Type2D::XBar {
            bar_shift: 1.0,
            bar_width: 0.5
        }
        .to_string(),
        String::from("xbar, bar width=0.5, bar shift=1")
    );
    assert_eq!(
        Type2D::YBar {
            bar_width: 0.5,
            bar_shift: 1.0
        }
        .to_string(),
        String::from("ybar, bar width=0.5, bar shift=1")
    );
    assert_eq!(
        Type2D::YBar {
            bar_shift: 1.0,
            bar_width: 0.5
        }
        .to_string(),
        String::from("ybar, bar width=0.5, bar shift=1")
    );
    assert_eq!(Type2D::XComb.to_string(), String::from("xcomb"));
    assert_eq!(Type2D::YComb.to_string(), String::from("ycomb"));
    assert_eq!(Type2D::OnlyMarks.to_string(), String::from("only marks"));
}

// This test is here only to let us know if we added an enum variant
// but we forgot to add unit tests for it
//
// If this fails, it is because you added a new variant.
// Please do the following:
// 1) Add a unit test for the new variant you added (see examples below).
// 2) AFTER doing (1), add the new variant to the match.
#[test]
fn plot_keys_tested() {
    let plot_key = PlotKey::Custom(String::from(""));
    match plot_key {
        PlotKey::Custom(_) => (),
        PlotKey::Type2D(_) => (),
        PlotKey::XError(_) => (),
        PlotKey::XErrorDirection(_) => (),
        PlotKey::YError(_) => (),
        PlotKey::YErrorDirection(_) => (),
    }
}

#[test]
fn plot_key_custom_to_string() {
    assert_eq!(
        PlotKey::Custom(String::from("something/random here")).to_string(),
        String::from("something/random here")
    );
}

#[test]
fn plot_key_type_2d_to_string() {
    assert_eq!(
        PlotKey::Type2D(Type2D::SharpPlot).to_string(),
        String::from("sharp plot")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::ConstLeft).to_string(),
        String::from("const plot mark left")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::ConstRight).to_string(),
        String::from("const plot mark right")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::ConstMid).to_string(),
        String::from("const plot mark mid")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::JumpLeft).to_string(),
        String::from("jump mark left")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::JumpRight).to_string(),
        String::from("jump mark right")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::JumpMid).to_string(),
        String::from("jump mark mid")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::XComb).to_string(),
        String::from("xcomb")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::YComb).to_string(),
        String::from("ycomb")
    );
    assert_eq!(
        PlotKey::Type2D(Type2D::OnlyMarks).to_string(),
        String::from("only marks")
    );
}

#[test]
fn plot_key_x_error_to_string() {
    assert_eq!(
        PlotKey::XError(ErrorCharacter::Absolute).to_string(),
        String::from("error bars/x explicit")
    );
    assert_eq!(
        PlotKey::XError(ErrorCharacter::Relative).to_string(),
        String::from("error bars/x explicit relative")
    );
}

#[test]
fn plot_key_x_error_direction_to_string() {
    assert_eq!(
        PlotKey::XErrorDirection(ErrorDirection::None).to_string(),
        String::from("error bars/x dir=none")
    );
    assert_eq!(
        PlotKey::XErrorDirection(ErrorDirection::Plus).to_string(),
        String::from("error bars/x dir=plus")
    );
    assert_eq!(
        PlotKey::XErrorDirection(ErrorDirection::Minus).to_string(),
        String::from("error bars/x dir=minus")
    );
    assert_eq!(
        PlotKey::XErrorDirection(ErrorDirection::Both).to_string(),
        String::from("error bars/x dir=both")
    );
}

#[test]
fn plot_key_y_error_to_string() {
    assert_eq!(
        PlotKey::YError(ErrorCharacter::Absolute).to_string(),
        String::from("error bars/y explicit")
    );
    assert_eq!(
        PlotKey::YError(ErrorCharacter::Relative).to_string(),
        String::from("error bars/y explicit relative")
    );
}

#[test]
fn plot_key_y_error_direction_to_string() {
    assert_eq!(
        PlotKey::YErrorDirection(ErrorDirection::None).to_string(),
        String::from("error bars/y dir=none")
    );
    assert_eq!(
        PlotKey::YErrorDirection(ErrorDirection::Plus).to_string(),
        String::from("error bars/y dir=plus")
    );
    assert_eq!(
        PlotKey::YErrorDirection(ErrorDirection::Minus).to_string(),
        String::from("error bars/y dir=minus")
    );
    assert_eq!(
        PlotKey::YErrorDirection(ErrorDirection::Both).to_string(),
        String::from("error bars/y dir=both")
    );
}

#[test]
fn plot_2d_new() {
    let plot = Plot2D::new();
    assert!(plot.coordinates.is_empty());
    assert!(plot.keys.is_empty());
}

#[test]
fn plot_2d_add_key() {
    let mut plot = Plot2D::new();
    plot.add_key(PlotKey::Type2D(Type2D::SharpPlot));
    assert_eq!(plot.keys.len(), 1);
    assert_eq!(plot.keys[0].to_string(), String::from("sharp plot"));

    plot.add_key(PlotKey::XError(ErrorCharacter::Absolute));
    assert_eq!(plot.keys.len(), 2);
    assert_eq!(plot.keys[0].to_string(), String::from("sharp plot"));
    assert_eq!(
        plot.keys[1].to_string(),
        String::from("error bars/x explicit")
    );

    plot.add_key(PlotKey::Custom(String::from("random")));
    assert_eq!(plot.keys.len(), 3);
    assert_eq!(plot.keys[0].to_string(), String::from("sharp plot"));
    assert_eq!(
        plot.keys[1].to_string(),
        String::from("error bars/x explicit")
    );
    assert_eq!(plot.keys[2].to_string(), String::from("random"));

    plot.add_key(PlotKey::Custom(String::from("random")));
    assert_eq!(plot.keys.len(), 4);
    assert_eq!(plot.keys[0].to_string(), String::from("sharp plot"));
    assert_eq!(
        plot.keys[1].to_string(),
        String::from("error bars/x explicit")
    );
    assert_eq!(plot.keys[2].to_string(), String::from("random"));
    assert_eq!(plot.keys[3].to_string(), String::from("random"));

    plot.add_key(PlotKey::Type2D(Type2D::SharpPlot));
    assert_eq!(plot.keys.len(), 4);
    assert_eq!(
        plot.keys[0].to_string(),
        String::from("error bars/x explicit")
    );
    assert_eq!(plot.keys[1].to_string(), String::from("random"));
    assert_eq!(plot.keys[2].to_string(), String::from("random"));
    assert_eq!(plot.keys[3].to_string(), String::from("sharp plot"));

    plot.add_key(PlotKey::XError(ErrorCharacter::Relative));
    assert_eq!(plot.keys[0].to_string(), String::from("random"));
    assert_eq!(plot.keys[1].to_string(), String::from("random"));
    assert_eq!(plot.keys[2].to_string(), String::from("sharp plot"));
    assert_eq!(
        plot.keys[3].to_string(),
        String::from("error bars/x explicit relative")
    );
}

#[test]
fn plot_2d_to_string() {
    let mut plot = Plot2D::new();
    assert_eq!(plot.to_string(), "\t\\addplot[] coordinates {\n\t};");

    plot.coordinates.push((1.0, -1.0).into());
    assert_eq!(
        plot.to_string(),
        "\t\\addplot[] coordinates {\n\t\t(1,-1)\n\t};"
    );

    plot.coordinates.clear();
    plot.add_key(PlotKey::Type2D(Type2D::SharpPlot));
    assert_eq!(
        plot.to_string(),
        "\t\\addplot[\n\t\tsharp plot,\n\t] coordinates {\n\t};"
    );

    plot.add_key(PlotKey::XError(ErrorCharacter::Absolute));
    plot.add_key(PlotKey::XErrorDirection(ErrorDirection::Both));
    plot.coordinates.push((1.0, -1.0).into());
    plot.coordinates.push((2.0, -2.0).into());
    plot.coordinates.push((3.0, -3.0).into());
    assert_eq!(
        plot.to_string(),
        "\t\\addplot[\n\t\tsharp plot,\n\t\terror bars/x explicit,\n\t\terror bars/x dir=both,\n\t] coordinates {\n\t\t(1,-1)\n\t\t(2,-2)\n\t\t(3,-3)\n\t};"
    );
}
