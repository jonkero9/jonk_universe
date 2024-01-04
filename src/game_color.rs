use crate::model::star_system::StarColor;
use raylib::color::Color;

#[derive(Debug)]
pub struct Gamecolors {
    pub blue: Color,
    pub white: Color,
    pub yellow: Color,
    pub orange: Color,
    pub red: Color,
    pub bg: Color,
}

impl From<StarColor> for Color {
    fn from(value: StarColor) -> Self {
        return match value {
            StarColor::Red => COLORS.red,
            StarColor::Orange => COLORS.orange,
            StarColor::Yellow => COLORS.yellow,
            StarColor::White => COLORS.white,
            StarColor::Blue => COLORS.blue,
        };
    }
}

pub static COLORS: Gamecolors = Gamecolors {
    blue: Color {
        r: 137,
        g: 180,
        b: 250,
        a: 255,
    },
    white: Color {
        r: 186,
        g: 194,
        b: 222,
        a: 255,
    },
    yellow: Color {
        r: 249,
        g: 226,
        b: 175,
        a: 255,
    },
    orange: Color {
        r: 243,
        b: 181,
        g: 139,
        a: 255,
    },
    red: Color {
        r: 243,
        g: 139,
        b: 168,
        a: 255,
    },
    bg: Color {
        r: 30,
        g: 30,
        b: 46,
        a: 255,
    },
};
