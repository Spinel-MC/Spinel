use crate::color::AlphaColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
    pub const BLACK: Self = Self::from_rgb(0, 0, 0);

    #[must_use]
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    #[must_use]
    pub fn from_rgb_f32(red: f32, green: f32, blue: f32) -> Self {
        Self::from_rgb(
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
        )
    }

    #[must_use]
    pub const fn from_rgb_int(rgb: u32) -> Self {
        Self {
            red: ((rgb >> 16) & 0xff) as u8,
            green: ((rgb >> 8) & 0xff) as u8,
            blue: (rgb & 0xff) as u8,
        }
    }

    pub fn try_from_rgb(red: i32, green: i32, blue: i32) -> Result<Self, ColorError> {
        Ok(Self::from_rgb(
            checked_component(red, ColorComponent::Red)?,
            checked_component(green, ColorComponent::Green)?,
            checked_component(blue, ColorComponent::Blue)?,
        ))
    }

    #[must_use]
    pub const fn with_red(self, red: u8) -> Self {
        Self { red, ..self }
    }

    #[must_use]
    pub const fn with_green(self, green: u8) -> Self {
        Self { green, ..self }
    }

    #[must_use]
    pub const fn with_blue(self, blue: u8) -> Self {
        Self { blue, ..self }
    }

    #[must_use]
    pub const fn with_alpha(self, alpha: u8) -> AlphaColor {
        AlphaColor::from_argb_components(alpha, self.red, self.green, self.blue)
    }

    pub fn from_hex_string(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#')?;
        if hex.len() != 6 {
            return None;
        }
        u32::from_str_radix(hex, 16).ok().map(Self::from_rgb_int)
    }

    #[must_use]
    pub fn as_hex_string(self) -> String {
        format!("#{:06X}", self.as_rgb())
    }

    #[must_use]
    pub fn mix_with(self, colors: &[Self]) -> Self {
        let mut red = self.red as u32;
        let mut green = self.green as u32;
        let mut blue = self.blue as u32;
        let mut max = self.red.max(self.green).max(self.blue) as u32;

        for color in colors {
            red += color.red as u32;
            green += color.green as u32;
            blue += color.blue as u32;
            max += color.red.max(color.green).max(color.blue) as u32;
        }

        let count = colors.len() as f32 + 1.0;
        let average_red = red as f32 / count;
        let average_green = green as f32 / count;
        let average_blue = blue as f32 / count;
        let average_max = max as f32 / count;
        let maximum_average = average_red.max(average_green).max(average_blue);
        if maximum_average <= f32::EPSILON {
            return Self::BLACK;
        }
        let gain_factor = average_max / maximum_average;

        Self::from_rgb(
            (average_red * gain_factor).round() as u8,
            (average_green * gain_factor).round() as u8,
            (average_blue * gain_factor).round() as u8,
        )
    }

    #[must_use]
    pub const fn red(self) -> u8 {
        self.red
    }

    #[must_use]
    pub const fn green(self) -> u8 {
        self.green
    }

    #[must_use]
    pub const fn blue(self) -> u8 {
        self.blue
    }

    #[must_use]
    pub const fn as_rgb(self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | self.blue as u32
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::from_rgb_int(value)
    }
}

fn checked_component(value: i32, component: ColorComponent) -> Result<u8, ColorError> {
    u8::try_from(value).map_err(|_| ColorError { component, value })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorError {
    pub component: ColorComponent,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorComponent {
    Alpha,
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use crate::color::{AlphaColor, Color, ColorComponent, DyeColor};

    #[test]
    fn color_converts_to_and_from_rgb_int() {
        let color = Color::from_rgb_int(0x12_34_56);

        assert_eq!(color.red(), 0x12);
        assert_eq!(color.green(), 0x34);
        assert_eq!(color.blue(), 0x56);
        assert_eq!(color.as_rgb(), 0x12_34_56);
    }

    #[test]
    fn color_rejects_out_of_range_components() {
        let error = Color::try_from_rgb(0, 300, 0).unwrap_err();

        assert_eq!(error.component, ColorComponent::Green);
        assert_eq!(error.value, 300);
    }

    #[test]
    fn color_hex_and_alpha_helpers_match_minestom_shape() {
        let color = Color::from_hex_string("#123456").unwrap();

        assert_eq!(color.as_rgb(), 0x12_34_56);
        assert_eq!(color.as_hex_string(), "#123456");
        assert_eq!(color.with_red(0xaa).as_rgb(), 0xaa_34_56);
        assert_eq!(color.with_alpha(0x80).as_argb(), 0x80_12_34_56);
    }

    #[test]
    fn alpha_color_converts_argb_and_rgba() {
        let color = AlphaColor::from_argb(0x12_34_56_78);

        assert_eq!(color.alpha(), 0x12);
        assert_eq!(color.as_argb(), 0x12_34_56_78);
        assert_eq!(color.as_rgba(), 0x34_56_78_12);
        assert_eq!(AlphaColor::from_rgba_hex_string("#34567812"), Some(color));
        assert_eq!(AlphaColor::from_argb_hex_string("#12345678"), Some(color));
    }

    #[test]
    fn dye_colors_include_all_vanilla_entries() {
        assert_eq!(DyeColor::ALL.len(), 16);
        assert_eq!(DyeColor::White.color().as_rgb(), 0xf9fffe);
        assert_eq!(DyeColor::Black.text_color().as_rgb(), 0x000000);
        assert_eq!(DyeColor::Red.firework_color().as_rgb(), 0xb3312c);
        assert_eq!(DyeColor::Orange.map_color_id(), 15);
    }
}
