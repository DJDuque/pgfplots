use core::fmt;

#[derive(Clone, Copy, Debug, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum PredefinedColor {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Black,
    Gray,
    White,
    DarkGray,
    LightGray,
    Brown,
    Lime,
    Olive,
    Orange,
    Pink,
    Purple,
    Teal,
    Violet,
}

#[derive(Clone, Debug)]
pub struct Color {
    value: String,
}

impl Color {
    pub fn from_mix<I, C>(weighted_colors: I) -> Self
    where
        C: Into<Self>,
        I: IntoIterator<Item = (C, u8)>,
    {
        let mut result = vec![String::from("rgb,255:")];
        for (color, weight) in weighted_colors.into_iter() {
            let color: Color = color.into();

            result.push(color.to_string());
            result.push(String::from(","));
            result.push(weight.to_string());
            result.push(String::from(";"));
        }
        // Remove the last weight
        result.pop();
        Self {
            value: result.join(""),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<PredefinedColor> for Color {
    fn from(value: PredefinedColor) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
