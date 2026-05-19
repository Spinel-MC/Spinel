use crate::color::{Color, ColorComponent, ColorError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlphaColor {
    alpha: u8,
    color: Color,
}

impl AlphaColor {
    pub const WHITE: Self = Self::from_argb_components(255, 255, 255, 255);
    pub const BLACK: Self = Self::from_argb_components(255, 0, 0, 0);
    pub const TRANSPARENT: Self = Self::from_argb_components(0, 0, 0, 0);

    #[must_use]
    pub const fn from_argb_components(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self {
            alpha,
            color: Color::from_rgb(red, green, blue),
        }
    }

    #[must_use]
    pub fn from_argb_f32(alpha: f32, red: f32, green: f32, blue: f32) -> Self {
        Self::from_argb_components(
            (alpha * 255.0) as u8,
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
        )
    }

    #[must_use]
    pub const fn from_argb(argb: u32) -> Self {
        Self::from_argb_components(
            ((argb >> 24) & 0xff) as u8,
            ((argb >> 16) & 0xff) as u8,
            ((argb >> 8) & 0xff) as u8,
            (argb & 0xff) as u8,
        )
    }

    pub fn try_from_argb(alpha: i32, red: i32, green: i32, blue: i32) -> Result<Self, ColorError> {
        Ok(Self::from_argb_components(
            checked_alpha(alpha)?,
            Color::try_from_rgb(red, green, blue)?.red(),
            Color::try_from_rgb(red, green, blue)?.green(),
            Color::try_from_rgb(red, green, blue)?.blue(),
        ))
    }

    #[must_use]
    pub const fn from_color(alpha: u8, color: Color) -> Self {
        Self { alpha, color }
    }

    #[must_use]
    pub const fn color(self) -> Color {
        self.color
    }

    #[must_use]
    pub const fn alpha(self) -> u8 {
        self.alpha
    }

    #[must_use]
    pub const fn red(self) -> u8 {
        self.color.red()
    }

    #[must_use]
    pub const fn green(self) -> u8 {
        self.color.green()
    }

    #[must_use]
    pub const fn blue(self) -> u8 {
        self.color.blue()
    }

    #[must_use]
    pub const fn with_alpha(self, alpha: u8) -> Self {
        Self { alpha, ..self }
    }

    #[must_use]
    pub const fn with_red(self, red: u8) -> Self {
        Self {
            color: self.color.with_red(red),
            ..self
        }
    }

    #[must_use]
    pub const fn with_green(self, green: u8) -> Self {
        Self {
            color: self.color.with_green(green),
            ..self
        }
    }

    #[must_use]
    pub const fn with_blue(self, blue: u8) -> Self {
        Self {
            color: self.color.with_blue(blue),
            ..self
        }
    }

    #[must_use]
    pub const fn as_argb(self) -> u32 {
        ((self.alpha as u32) << 24) | self.color.as_rgb()
    }

    #[must_use]
    pub const fn as_rgba(self) -> u32 {
        (self.color.as_rgb() << 8) | self.alpha as u32
    }

    pub fn from_argb_hex_string(hex: &str) -> Option<Self> {
        parse_hex_u32(hex).map(Self::from_argb)
    }

    pub fn from_rgba_hex_string(hex: &str) -> Option<Self> {
        parse_hex_u32(hex).map(|rgba| Self::from_argb(rgba.rotate_right(8)))
    }
}

fn checked_alpha(value: i32) -> Result<u8, ColorError> {
    u8::try_from(value).map_err(|_| ColorError {
        component: ColorComponent::Alpha,
        value,
    })
}

fn parse_hex_u32(hex: &str) -> Option<u32> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 8 {
        return None;
    }
    u32::from_str_radix(hex, 16).ok()
}
