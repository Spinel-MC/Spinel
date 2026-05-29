use crate::data_components::DataComponentValue;
use spinel_nbt::Nbt;
use spinel_utils::color::{Color, DyeColor};

impl DataComponentValue for Color {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::Int(self.as_rgb() as i32)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::Int(rgb) => Some(Self::from_rgb_int(*rgb as u32)),
            _ => None,
        }
    }
}

impl DataComponentValue for DyeColor {
    fn to_component_nbt(&self) -> Nbt {
        Nbt::String(dye_color_nbt_name(*self).to_string())
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        match component_nbt {
            Nbt::String(name) => dye_color_from_nbt_name(name),
            _ => None,
        }
    }
}

#[must_use]
pub const fn dye_color_nbt_name(dye_color: DyeColor) -> &'static str {
    match dye_color {
        DyeColor::White => "white",
        DyeColor::Orange => "orange",
        DyeColor::Magenta => "magenta",
        DyeColor::LightBlue => "light_blue",
        DyeColor::Yellow => "yellow",
        DyeColor::Lime => "lime",
        DyeColor::Pink => "pink",
        DyeColor::Gray => "gray",
        DyeColor::LightGray => "light_gray",
        DyeColor::Cyan => "cyan",
        DyeColor::Purple => "purple",
        DyeColor::Blue => "blue",
        DyeColor::Brown => "brown",
        DyeColor::Green => "green",
        DyeColor::Red => "red",
        DyeColor::Black => "black",
    }
}

#[must_use]
pub const fn dye_color_protocol_id(dye_color: DyeColor) -> i32 {
    match dye_color {
        DyeColor::White => 0,
        DyeColor::Orange => 1,
        DyeColor::Magenta => 2,
        DyeColor::LightBlue => 3,
        DyeColor::Yellow => 4,
        DyeColor::Lime => 5,
        DyeColor::Pink => 6,
        DyeColor::Gray => 7,
        DyeColor::LightGray => 8,
        DyeColor::Cyan => 9,
        DyeColor::Purple => 10,
        DyeColor::Blue => 11,
        DyeColor::Brown => 12,
        DyeColor::Green => 13,
        DyeColor::Red => 14,
        DyeColor::Black => 15,
    }
}

#[must_use]
pub fn dye_color_from_nbt_name(name: &str) -> Option<DyeColor> {
    match name {
        "white" => Some(DyeColor::White),
        "orange" => Some(DyeColor::Orange),
        "magenta" => Some(DyeColor::Magenta),
        "light_blue" => Some(DyeColor::LightBlue),
        "yellow" => Some(DyeColor::Yellow),
        "lime" => Some(DyeColor::Lime),
        "pink" => Some(DyeColor::Pink),
        "gray" => Some(DyeColor::Gray),
        "light_gray" => Some(DyeColor::LightGray),
        "cyan" => Some(DyeColor::Cyan),
        "purple" => Some(DyeColor::Purple),
        "blue" => Some(DyeColor::Blue),
        "brown" => Some(DyeColor::Brown),
        "green" => Some(DyeColor::Green),
        "red" => Some(DyeColor::Red),
        "black" => Some(DyeColor::Black),
        _ => None,
    }
}
