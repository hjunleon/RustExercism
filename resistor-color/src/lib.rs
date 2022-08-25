use int_enum::IntEnum;
use std::string::ToString;
use enum_iterator::{all, Sequence};
use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Sequence, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::White => write!(f, "White"),
            ResistorColor::Yellow => write!(f, "Yellow"),
        }
    }
}


pub fn color_to_value(_color: ResistorColor) -> u32 {

    _color.int_value()   // or _color as usize
    // match _color{
    //     ResistorColor::Black=>0,
    //     ResistorColor::Brown=>1,
    //     ResistorColor::Red=>2,
    //     ResistorColor::Orange=>3,
    //     ResistorColor::Yellow=>4,
    //     ResistorColor::Green=>5,
    //     ResistorColor::Blue=>6,
    //     ResistorColor::Violet=>7,
    //     ResistorColor::Grey=>8,
    //     ResistorColor::White=>9,
    // }
    // unimplemented!("convert a color into a numerical representation")
}

pub fn value_to_color_string(value: u32) -> String {
    
    let res: String = match ResistorColor::from_int(value) {
        Ok(x) => format!("{:?}", x),//x.to_string(),
        Err(_) => String::from("value out of range")
    };
    res
    // unimplemented!(
    //     "convert the value {} into a string representation of color",
    //     value
    // )
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
    // unimplemented!("return a list of all the colors ordered by resistance")
}
