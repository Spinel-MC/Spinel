use crate::data_components::DataComponentValue;
use crate::data_components::nbt_reader::{
    bool_field_or, compound_from_nbt, i32_list_field_or_empty, string_field,
};
use spinel_nbt::{Nbt, NbtCompound};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FireworkExplosion {
    shape: FireworkExplosionShape,
    colors: Vec<i32>,
    fade_colors: Vec<i32>,
    has_trail: bool,
    has_twinkle: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FireworkExplosionShape {
    SmallBall,
    LargeBall,
    Star,
    Creeper,
    Burst,
}

impl FireworkExplosion {
    #[must_use]
    pub fn new(
        shape: FireworkExplosionShape,
        colors: Vec<i32>,
        fade_colors: Vec<i32>,
        has_trail: bool,
        has_twinkle: bool,
    ) -> Self {
        Self {
            shape,
            colors,
            fade_colors,
            has_trail,
            has_twinkle,
        }
    }

    #[must_use]
    pub const fn shape(&self) -> FireworkExplosionShape {
        self.shape
    }

    #[must_use]
    pub fn colors(&self) -> &[i32] {
        &self.colors
    }

    #[must_use]
    pub fn fade_colors(&self) -> &[i32] {
        &self.fade_colors
    }

    #[must_use]
    pub const fn has_trail(&self) -> bool {
        self.has_trail
    }

    #[must_use]
    pub const fn has_twinkle(&self) -> bool {
        self.has_twinkle
    }
}

impl FireworkExplosionShape {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::SmallBall => "small_ball",
            Self::LargeBall => "large_ball",
            Self::Star => "star",
            Self::Creeper => "creeper",
            Self::Burst => "burst",
        }
    }

    #[must_use]
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::SmallBall => 0,
            Self::LargeBall => 1,
            Self::Star => 2,
            Self::Creeper => 3,
            Self::Burst => 4,
        }
    }

    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name {
            "small_ball" => Some(Self::SmallBall),
            "large_ball" => Some(Self::LargeBall),
            "star" => Some(Self::Star),
            "creeper" => Some(Self::Creeper),
            "burst" => Some(Self::Burst),
            _ => None,
        }
    }
}

impl DataComponentValue for FireworkExplosion {
    fn to_component_nbt(&self) -> Nbt {
        let mut compound = NbtCompound::new();
        compound.insert(
            "shape".to_string(),
            Nbt::String(self.shape.nbt_name().to_string()),
        );
        if !self.colors.is_empty() {
            compound.insert(
                "colors".to_string(),
                Nbt::List(
                    self.colors
                        .iter()
                        .copied()
                        .map(Nbt::Int)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if !self.fade_colors.is_empty() {
            compound.insert(
                "fade_colors".to_string(),
                Nbt::List(
                    self.fade_colors
                        .iter()
                        .copied()
                        .map(Nbt::Int)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            );
        }
        if self.has_trail {
            compound.insert("has_trail".to_string(), Nbt::Byte(1));
        }
        if self.has_twinkle {
            compound.insert("has_twinkle".to_string(), Nbt::Byte(1));
        }
        Nbt::Compound(compound)
    }

    fn from_component_nbt(component_nbt: &Nbt) -> Option<Self> {
        let compound = compound_from_nbt(component_nbt)?;
        Some(Self {
            shape: FireworkExplosionShape::from_nbt_name(&string_field(compound, "shape")?)?,
            colors: i32_list_field_or_empty(compound, "colors")?,
            fade_colors: i32_list_field_or_empty(compound, "fade_colors")?,
            has_trail: bool_field_or(compound, "has_trail", false)?,
            has_twinkle: bool_field_or(compound, "has_twinkle", false)?,
        })
    }
}
