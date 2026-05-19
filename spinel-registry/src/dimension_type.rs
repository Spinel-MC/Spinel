use crate::RegistryCodec;
use crate::nbt_builder::{bool_tag, put_optional};
use spinel_nbt::{Nbt, NbtCompound};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DimensionType {
    pub has_fixed_time: bool,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub coordinate_scale: f64,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: Cow<'static, str>,
    pub ambient_light: f32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub monster_spawn_block_light_limit: i32,
    pub skybox: DimensionSkybox,
    pub cardinal_light: CardinalLightType,
}

impl DimensionType {
    #[must_use]
    pub fn builder() -> DimensionTypeBuilder {
        DimensionTypeBuilder::new()
    }
}
impl RegistryCodec for DimensionType {
    fn registry_nbt(&self) -> NbtCompound {
        let mut dimension_type_nbt = NbtCompound::new();
        dimension_type_nbt.insert("has_fixed_time".to_string(), bool_tag(self.has_fixed_time));
        dimension_type_nbt.insert("has_skylight".to_string(), bool_tag(self.has_skylight));
        dimension_type_nbt.insert("has_ceiling".to_string(), bool_tag(self.has_ceiling));
        dimension_type_nbt.insert(
            "coordinate_scale".to_string(),
            Nbt::Double(self.coordinate_scale),
        );
        dimension_type_nbt.insert("min_y".to_string(), Nbt::Int(self.min_y));
        dimension_type_nbt.insert("height".to_string(), Nbt::Int(self.height));
        dimension_type_nbt.insert("logical_height".to_string(), Nbt::Int(self.logical_height));
        dimension_type_nbt.insert(
            "infiniburn".to_string(),
            Nbt::String(self.infiniburn.to_string()),
        );
        dimension_type_nbt.insert("ambient_light".to_string(), Nbt::Float(self.ambient_light));
        dimension_type_nbt.insert(
            "monster_spawn_light_level".to_string(),
            self.monster_spawn_light_level.nbt(),
        );
        dimension_type_nbt.insert(
            "monster_spawn_block_light_limit".to_string(),
            Nbt::Int(self.monster_spawn_block_light_limit),
        );
        put_optional(&mut dimension_type_nbt, "skybox", self.skybox.nbt());
        put_optional(
            &mut dimension_type_nbt,
            "cardinal_light",
            self.cardinal_light.nbt(),
        );
        dimension_type_nbt
    }
}

pub struct DimensionTypeBuilder {
    dimension_type: DimensionType,
}

impl DimensionTypeBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dimension_type: DimensionType::default(),
        }
    }

    #[must_use]
    pub fn fixed_time(mut self, has_fixed_time: bool) -> Self {
        self.dimension_type.has_fixed_time = has_fixed_time;
        self
    }

    #[must_use]
    pub fn skylight(mut self, has_skylight: bool) -> Self {
        self.dimension_type.has_skylight = has_skylight;
        self
    }

    #[must_use]
    pub fn ceiling(mut self, has_ceiling: bool) -> Self {
        self.dimension_type.has_ceiling = has_ceiling;
        self
    }

    #[must_use]
    pub fn vertical_bounds(mut self, min_y: i32, height: i32, logical_height: i32) -> Self {
        self.dimension_type.min_y = min_y;
        self.dimension_type.height = height;
        self.dimension_type.logical_height = logical_height;
        self
    }

    #[must_use]
    pub fn build(self) -> DimensionType {
        self.dimension_type
    }
}

impl Default for DimensionTypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DimensionType {
    fn default() -> Self {
        Self {
            has_fixed_time: false,
            has_skylight: true,
            has_ceiling: false,
            coordinate_scale: 1.0,
            min_y: -64,
            height: 384,
            logical_height: 384,
            infiniburn: Cow::Borrowed("#minecraft:infiniburn_overworld"),
            ambient_light: 0.0,
            monster_spawn_light_level: MonsterSpawnLightLevel::Uniform {
                min_inclusive: 0,
                max_inclusive: 7,
            },
            monster_spawn_block_light_limit: 0,
            skybox: DimensionSkybox::Overworld,
            cardinal_light: CardinalLightType::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterSpawnLightLevel {
    Fixed(i32),
    Uniform {
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

impl MonsterSpawnLightLevel {
    fn nbt(self) -> Nbt {
        match self {
            Self::Fixed(light_level) => Nbt::Int(light_level),
            Self::Uniform {
                min_inclusive,
                max_inclusive,
            } => uniform_light_level_nbt(min_inclusive, max_inclusive),
        }
    }
}

fn uniform_light_level_nbt(min_inclusive: i32, max_inclusive: i32) -> Nbt {
    let mut light_level_nbt = NbtCompound::new();
    light_level_nbt.insert(
        "type".to_string(),
        Nbt::String("minecraft:uniform".to_string()),
    );
    light_level_nbt.insert("min_inclusive".to_string(), Nbt::Int(min_inclusive));
    light_level_nbt.insert("max_inclusive".to_string(), Nbt::Int(max_inclusive));
    Nbt::Compound(light_level_nbt)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionSkybox {
    None,
    Overworld,
    End,
}

impl DimensionSkybox {
    fn nbt(self) -> Option<Nbt> {
        match self {
            Self::None => None,
            Self::Overworld => Some(Nbt::String("minecraft:overworld".to_string())),
            Self::End => Some(Nbt::String("minecraft:end".to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalLightType {
    Default,
    Nether,
}

impl CardinalLightType {
    fn nbt(self) -> Option<Nbt> {
        match self {
            Self::Default => None,
            Self::Nether => Some(Nbt::String("minecraft:nether".to_string())),
        }
    }
}
