use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if let Ok(resistor_color) = ResistorColor::from_int(value) {
        format!("{:?}", resistor_color)
    } else {
        String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut items: Vec<ResistorColor> = all::<ResistorColor>().collect();
    items.sort();
    items
}
