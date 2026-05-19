use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DyeColor {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

impl DyeColor {
    pub const ALL: [Self; 16] = [
        Self::White,
        Self::Orange,
        Self::Magenta,
        Self::LightBlue,
        Self::Yellow,
        Self::Lime,
        Self::Pink,
        Self::Gray,
        Self::LightGray,
        Self::Cyan,
        Self::Purple,
        Self::Blue,
        Self::Brown,
        Self::Green,
        Self::Red,
        Self::Black,
    ];

    #[must_use]
    pub const fn color(self) -> Color {
        self.data().texture_diffuse_color
    }

    #[must_use]
    pub const fn text_color(self) -> Color {
        self.data().text_color
    }

    #[must_use]
    pub const fn firework_color(self) -> Color {
        self.data().firework_color
    }

    #[must_use]
    pub const fn map_color_id(self) -> i32 {
        self.data().map_color_id
    }

    #[must_use]
    pub const fn red(self) -> u8 {
        self.color().red()
    }

    #[must_use]
    pub const fn green(self) -> u8 {
        self.color().green()
    }

    #[must_use]
    pub const fn blue(self) -> u8 {
        self.color().blue()
    }

    const fn data(self) -> DyeColorData {
        match self {
            Self::White => DyeColorData::new(0xf9fffe, 0xffffff, 0xf0f0f0, 8),
            Self::Orange => DyeColorData::new(0xf9801d, 0xff681f, 0xeb8844, 15),
            Self::Magenta => DyeColorData::new(0xc74ebd, 0xff00ff, 0xc354cd, 16),
            Self::LightBlue => DyeColorData::new(0x3ab3da, 0x9ac0cd, 0x6689d3, 17),
            Self::Yellow => DyeColorData::new(0xfed83d, 0xffff00, 0xdecf2a, 18),
            Self::Lime => DyeColorData::new(0x80c71f, 0xbfff00, 0x41cd34, 19),
            Self::Pink => DyeColorData::new(0xf38baa, 0xff69b4, 0xd88198, 20),
            Self::Gray => DyeColorData::new(0x474f52, 0x808080, 0x434343, 21),
            Self::LightGray => DyeColorData::new(0x9d9d97, 0xd3d3d3, 0xababab, 22),
            Self::Cyan => DyeColorData::new(0x169c9c, 0x00ffff, 0x287697, 23),
            Self::Purple => DyeColorData::new(0x8932b8, 0xa020f0, 0x7b2fbe, 24),
            Self::Blue => DyeColorData::new(0x3c44aa, 0x0000ff, 0x253192, 25),
            Self::Brown => DyeColorData::new(0x835432, 0x8b4513, 0x51301a, 26),
            Self::Green => DyeColorData::new(0x5e7c16, 0x00ff00, 0x3b511a, 27),
            Self::Red => DyeColorData::new(0xb02e26, 0xff0000, 0xb3312c, 28),
            Self::Black => DyeColorData::new(0x1d1d21, 0x000000, 0x1e1b1b, 29),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DyeColorData {
    texture_diffuse_color: Color,
    text_color: Color,
    firework_color: Color,
    map_color_id: i32,
}

impl DyeColorData {
    const fn new(
        texture_diffuse_color: u32,
        text_color: u32,
        firework_color: u32,
        map_color_id: i32,
    ) -> Self {
        Self {
            texture_diffuse_color: Color::from_rgb_int(texture_diffuse_color),
            text_color: Color::from_rgb_int(text_color),
            firework_color: Color::from_rgb_int(firework_color),
            map_color_id,
        }
    }
}
