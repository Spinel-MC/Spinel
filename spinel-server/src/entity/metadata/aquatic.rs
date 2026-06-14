use spinel_registry::TropicalFishPattern;
use spinel_utils::color::DyeColor;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PufferfishState {
    #[default]
    Unpuffed,
    SemiPuffed,
    FullyPuffed,
}

impl PufferfishState {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Unpuffed => 0,
            Self::SemiPuffed => 1,
            Self::FullyPuffed => 2,
        }
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Unpuffed),
            1 => Some(Self::SemiPuffed),
            2 => Some(Self::FullyPuffed),
            _ => None,
        }
    }

    pub const fn bounding_box_size(self) -> f64 {
        match self {
            Self::Unpuffed => 0.35,
            Self::SemiPuffed => 0.5,
            Self::FullyPuffed => 0.7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TropicalFishVariant {
    pattern: TropicalFishPattern,
    base_color: DyeColor,
    pattern_color: DyeColor,
}

impl TropicalFishVariant {
    pub const DEFAULT: Self = Self::new(TropicalFishPattern::Kob, DyeColor::White, DyeColor::White);

    pub const fn new(
        pattern: TropicalFishPattern,
        base_color: DyeColor,
        pattern_color: DyeColor,
    ) -> Self {
        Self {
            pattern,
            base_color,
            pattern_color,
        }
    }

    pub const fn pattern(self) -> TropicalFishPattern {
        self.pattern
    }

    pub const fn base_color(self) -> DyeColor {
        self.base_color
    }

    pub const fn pattern_color(self) -> DyeColor {
        self.pattern_color
    }

    pub fn from_packed_id(packed_id: i32) -> Option<Self> {
        let pattern_color = dye_color_from_protocol_id((packed_id >> 24) & 0xff)?;
        let base_color = dye_color_from_protocol_id((packed_id >> 16) & 0xff)?;
        let pattern = TropicalFishPattern::from_id(packed_id & 0xffff)?;
        Some(Self::new(pattern, base_color, pattern_color))
    }

    pub fn packed_id(self) -> i32 {
        (dye_color_protocol_id(self.pattern_color) << 24)
            | (dye_color_protocol_id(self.base_color) << 16)
            | self.pattern.id()
    }

    pub const fn with_pattern(self, pattern: TropicalFishPattern) -> Self {
        Self { pattern, ..self }
    }

    pub const fn with_base_color(self, base_color: DyeColor) -> Self {
        Self { base_color, ..self }
    }

    pub const fn with_pattern_color(self, pattern_color: DyeColor) -> Self {
        Self {
            pattern_color,
            ..self
        }
    }
}

impl Default for TropicalFishVariant {
    fn default() -> Self {
        Self::DEFAULT
    }
}

fn dye_color_from_protocol_id(protocol_id: i32) -> Option<DyeColor> {
    DyeColor::ALL.get(protocol_id as usize).copied()
}

fn dye_color_protocol_id(color: DyeColor) -> i32 {
    DyeColor::ALL
        .iter()
        .position(|candidate| candidate == &color)
        .unwrap_or(0) as i32
}
