use crate::{AttributeOperation, Identifier, RegistryTagReference};
use spinel_nbt::{Nbt, NbtCompound};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnchantmentEffectBranch {
    Attribute,
    Conditional,
    DamageImmunity,
    Entity,
    Location,
    TargetedConditional,
    Value,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnchantmentEffect {
    Attribute(AttributeEffect),
    Conditional(Box<ConditionalEffect>),
    DamageImmunity(DamageImmunityEffect),
    Entity(TaggedEntityEffect),
    Location(TaggedLocationEffect),
    TargetedConditional(Box<TargetedConditionalEffect>),
    Value(TaggedValueEffect),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeEffect {
    id: Identifier,
    attribute: Identifier,
    amount: Nbt,
    operation: AttributeOperation,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DamageImmunityEffect;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConditionalEffect {
    effect: EnchantmentEffect,
    requirements: Option<NbtCompound>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetedConditionalEffect {
    enchanted: EnchantmentTarget,
    affected: Option<EnchantmentTarget>,
    effect: EnchantmentEffect,
    requirements: Option<NbtCompound>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaggedEntityEffect {
    type_key: Identifier,
    payload: NbtCompound,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaggedLocationEffect {
    type_key: Identifier,
    payload: NbtCompound,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaggedValueEffect {
    type_key: Identifier,
    payload: NbtCompound,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityEffect {
    AllOf(EntityEffectAllOf),
    ApplyMobEffect(EntityEffectApplyMobEffect),
    ChangeItemDamage(EntityEffectChangeItemDamage),
    DamageEntity(EntityEffectDamageEntity),
    Explode(EntityEffectExplode),
    Ignite(EntityEffectIgnite),
    ApplyImpulse(EntityEffectApplyImpulse),
    ApplyExhaustion(EntityEffectApplyExhaustion),
    PlaySound(EntityEffectPlaySound),
    ReplaceBlock(EntityEffectReplaceBlock),
    ReplaceDisk(EntityEffectReplaceDisk),
    RunFunction(EntityEffectRunFunction),
    SpawnParticles(EntityEffectSpawnParticles),
    SetBlockProperties(EntityEffectSetBlockProperties),
    SummonEntity(EntityEffectSummonEntity),
    Raw(TaggedEntityEffect),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectAllOf {
    effects: Vec<EntityEffect>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectApplyMobEffect {
    to_apply: RegistryTagReference,
    min_duration: Nbt,
    max_duration: Nbt,
    min_amplifier: Nbt,
    max_amplifier: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectChangeItemDamage {
    amount: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectDamageEntity {
    damage_type: Identifier,
    min_damage: Nbt,
    max_damage: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectExplode {
    attribute_to_user: bool,
    damage_type: Option<Identifier>,
    knockback_multiplier: Option<Nbt>,
    immune_blocks: Option<Nbt>,
    offset: Nbt,
    radius: Nbt,
    create_fire: bool,
    block_interaction: Nbt,
    small_particle: Nbt,
    large_particle: Nbt,
    sound: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectIgnite {
    duration: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectApplyImpulse {
    direction: Nbt,
    coordinate_scale: Nbt,
    magnitude: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectApplyExhaustion {
    amount: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectPlaySound {
    sounds: Vec<Nbt>,
    volume: Nbt,
    pitch: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectReplaceBlock {
    offset: Nbt,
    predicate: Nbt,
    block_state: Nbt,
    trigger_game_event: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectReplaceDisk {
    radius: Nbt,
    height: Nbt,
    offset: Nbt,
    predicate: Option<Nbt>,
    block_state: Nbt,
    trigger_game_event: Option<Nbt>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectRunFunction {
    function: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectSpawnParticles {
    particle: Nbt,
    horizontal_position: Nbt,
    vertical_position: Nbt,
    horizontal_velocity: Nbt,
    vertical_velocity: Nbt,
    speed: Nbt,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectSetBlockProperties {
    properties: Nbt,
    offset: Nbt,
    trigger_game_event: Option<Nbt>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityEffectSummonEntity {
    entity: Nbt,
    join_team: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnchantmentTarget {
    Attacker,
    DamagingEntity,
    Victim,
}
impl EnchantmentEffect {
    #[must_use]
    pub const fn get_branch(&self) -> EnchantmentEffectBranch {
        match self {
            Self::Attribute(_) => EnchantmentEffectBranch::Attribute,
            Self::Conditional(_) => EnchantmentEffectBranch::Conditional,
            Self::DamageImmunity(_) => EnchantmentEffectBranch::DamageImmunity,
            Self::Entity(_) => EnchantmentEffectBranch::Entity,
            Self::Location(_) => EnchantmentEffectBranch::Location,
            Self::TargetedConditional(_) => EnchantmentEffectBranch::TargetedConditional,
            Self::Value(_) => EnchantmentEffectBranch::Value,
        }
    }
    #[must_use]
    pub fn to_branch_nbt(&self) -> Nbt {
        let mut compound = match self {
            Self::Attribute(effect) => effect.to_nbt_compound(),
            Self::Conditional(effect) => effect.to_nbt_compound(),
            Self::DamageImmunity(effect) => effect.to_nbt_compound(),
            Self::Entity(effect) => effect.to_nbt_compound(),
            Self::Location(effect) => effect.to_nbt_compound(),
            Self::TargetedConditional(effect) => effect.to_nbt_compound(),
            Self::Value(effect) => effect.to_nbt_compound(),
        };
        compound.insert(
            "branch".to_owned(),
            Nbt::String(self.get_branch().nbt_name().to_owned()),
        );
        Nbt::Compound(compound)
    }
}
impl EnchantmentEffectBranch {
    #[must_use]
    pub const fn nbt_name(&self) -> &'static str {
        match self {
            Self::Attribute => "attribute",
            Self::Conditional => "conditional",
            Self::DamageImmunity => "damage_immunity",
            Self::Entity => "entity",
            Self::Location => "location",
            Self::TargetedConditional => "targeted_conditional",
            Self::Value => "value",
        }
    }
}
impl AttributeEffect {
    #[must_use]
    pub fn new(
        id: Identifier,
        attribute: Identifier,
        amount: Nbt,
        operation: AttributeOperation,
    ) -> Self {
        Self {
            id,
            attribute,
            amount,
            operation,
        }
    }
    #[must_use]
    pub const fn get_id(&self) -> &Identifier {
        &self.id
    }
    #[must_use]
    pub const fn get_attribute(&self) -> &Identifier {
        &self.attribute
    }
    #[must_use]
    pub const fn get_amount(&self) -> &Nbt {
        &self.amount
    }
    #[must_use]
    pub const fn get_operation(&self) -> AttributeOperation {
        self.operation
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("id".to_owned(), Nbt::String(self.id.to_string()));
        compound.insert(
            "attribute".to_owned(),
            Nbt::String(self.attribute.to_string()),
        );
        compound.insert("amount".to_owned(), self.amount.clone());
        compound.insert(
            "operation".to_owned(),
            Nbt::String(self.operation.nbt_name().to_owned()),
        );
        compound
    }
}
impl DamageImmunityEffect {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        NbtCompound::new()
    }
}
impl ConditionalEffect {
    #[must_use]
    pub fn new(effect: EnchantmentEffect, requirements: Option<NbtCompound>) -> Self {
        Self {
            effect,
            requirements,
        }
    }
    #[must_use]
    pub const fn get_effect(&self) -> &EnchantmentEffect {
        &self.effect
    }
    #[must_use]
    pub const fn get_requirements(&self) -> Option<&NbtCompound> {
        self.requirements.as_ref()
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("effect".to_owned(), self.effect.to_branch_nbt());
        if let Some(requirements) = &self.requirements {
            compound.insert(
                "requirements".to_owned(),
                Nbt::Compound(requirements.clone()),
            );
        }
        compound
    }
}
impl TargetedConditionalEffect {
    #[must_use]
    pub fn new(
        enchanted: EnchantmentTarget,
        affected: Option<EnchantmentTarget>,
        effect: EnchantmentEffect,
        requirements: Option<NbtCompound>,
    ) -> Self {
        Self {
            enchanted,
            affected,
            effect,
            requirements,
        }
    }
    #[must_use]
    pub const fn get_enchanted(&self) -> EnchantmentTarget {
        self.enchanted
    }
    #[must_use]
    pub const fn get_affected(&self) -> Option<EnchantmentTarget> {
        self.affected
    }
    #[must_use]
    pub const fn get_effect(&self) -> &EnchantmentEffect {
        &self.effect
    }
    #[must_use]
    pub const fn get_requirements(&self) -> Option<&NbtCompound> {
        self.requirements.as_ref()
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "enchanted".to_owned(),
            Nbt::String(self.enchanted.nbt_name().to_owned()),
        );
        if let Some(affected) = self.affected {
            compound.insert(
                "affected".to_owned(),
                Nbt::String(affected.nbt_name().to_owned()),
            );
        }
        compound.insert("effect".to_owned(), self.effect.to_branch_nbt());
        if let Some(requirements) = &self.requirements {
            compound.insert(
                "requirements".to_owned(),
                Nbt::Compound(requirements.clone()),
            );
        }
        compound
    }
}
impl TaggedEntityEffect {
    #[must_use]
    pub fn new(type_key: Identifier, payload: NbtCompound) -> Self {
        Self { type_key, payload }
    }
    #[must_use]
    pub const fn get_type_key(&self) -> &Identifier {
        &self.type_key
    }
    #[must_use]
    pub const fn get_payload(&self) -> &NbtCompound {
        &self.payload
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        tagged_payload_nbt(&self.type_key, &self.payload)
    }
}
impl TaggedLocationEffect {
    #[must_use]
    pub fn new(type_key: Identifier, payload: NbtCompound) -> Self {
        Self { type_key, payload }
    }
    #[must_use]
    pub const fn get_type_key(&self) -> &Identifier {
        &self.type_key
    }
    #[must_use]
    pub const fn get_payload(&self) -> &NbtCompound {
        &self.payload
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        tagged_payload_nbt(&self.type_key, &self.payload)
    }
}
impl TaggedValueEffect {
    #[must_use]
    pub fn new(type_key: Identifier, payload: NbtCompound) -> Self {
        Self { type_key, payload }
    }
    #[must_use]
    pub const fn get_type_key(&self) -> &Identifier {
        &self.type_key
    }
    #[must_use]
    pub const fn get_payload(&self) -> &NbtCompound {
        &self.payload
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        tagged_payload_nbt(&self.type_key, &self.payload)
    }
}
impl EntityEffect {
    #[must_use]
    pub fn get_type_key(&self) -> Identifier {
        match self {
            Self::AllOf(_) => Identifier::minecraft("all_of"),
            Self::ApplyMobEffect(_) => Identifier::minecraft("apply_mob_effect"),
            Self::ChangeItemDamage(_) => Identifier::minecraft("change_item_damage"),
            Self::DamageEntity(_) => Identifier::minecraft("damage_entity"),
            Self::Explode(_) => Identifier::minecraft("explode"),
            Self::Ignite(_) => Identifier::minecraft("ignite"),
            Self::ApplyImpulse(_) => Identifier::minecraft("apply_impulse"),
            Self::ApplyExhaustion(_) => Identifier::minecraft("apply_exhaustion"),
            Self::PlaySound(_) => Identifier::minecraft("play_sound"),
            Self::ReplaceBlock(_) => Identifier::minecraft("replace_block"),
            Self::ReplaceDisk(_) => Identifier::minecraft("replace_disk"),
            Self::RunFunction(_) => Identifier::minecraft("run_function"),
            Self::SpawnParticles(_) => Identifier::minecraft("spawn_particles"),
            Self::SetBlockProperties(_) => Identifier::minecraft("set_block_properties"),
            Self::SummonEntity(_) => Identifier::minecraft("summon_entity"),
            Self::Raw(effect) => effect.get_type_key().clone(),
        }
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = match self {
            Self::AllOf(effect) => effect.to_nbt_compound(),
            Self::ApplyMobEffect(effect) => effect.to_nbt_compound(),
            Self::ChangeItemDamage(effect) => effect.to_nbt_compound(),
            Self::DamageEntity(effect) => effect.to_nbt_compound(),
            Self::Explode(effect) => effect.to_nbt_compound(),
            Self::Ignite(effect) => effect.to_nbt_compound(),
            Self::ApplyImpulse(effect) => effect.to_nbt_compound(),
            Self::ApplyExhaustion(effect) => effect.to_nbt_compound(),
            Self::PlaySound(effect) => effect.to_nbt_compound(),
            Self::ReplaceBlock(effect) => effect.to_nbt_compound(),
            Self::ReplaceDisk(effect) => effect.to_nbt_compound(),
            Self::RunFunction(effect) => effect.to_nbt_compound(),
            Self::SpawnParticles(effect) => effect.to_nbt_compound(),
            Self::SetBlockProperties(effect) => effect.to_nbt_compound(),
            Self::SummonEntity(effect) => effect.to_nbt_compound(),
            Self::Raw(effect) => effect.to_nbt_compound(),
        };
        if !compound.contains_key("type") {
            compound.insert(
                "type".to_owned(),
                Nbt::String(self.get_type_key().to_string()),
            );
        }
        compound
    }
}
impl EntityEffectAllOf {
    #[must_use]
    pub fn new(effects: Vec<EntityEffect>) -> Self {
        Self { effects }
    }
    #[must_use]
    pub fn get_effects(&self) -> &[EntityEffect] {
        &self.effects
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "effects".to_owned(),
            Nbt::List(
                self.effects
                    .iter()
                    .map(|effect| Nbt::Compound(effect.to_nbt_compound()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        );
        compound
    }
}
impl EntityEffectApplyMobEffect {
    #[must_use]
    pub fn new(
        to_apply: RegistryTagReference,
        min_duration: Nbt,
        max_duration: Nbt,
        min_amplifier: Nbt,
        max_amplifier: Nbt,
    ) -> Self {
        Self {
            to_apply,
            min_duration,
            max_duration,
            min_amplifier,
            max_amplifier,
        }
    }
    #[must_use]
    pub const fn get_to_apply(&self) -> &RegistryTagReference {
        &self.to_apply
    }
    #[must_use]
    pub const fn get_min_duration(&self) -> &Nbt {
        &self.min_duration
    }
    #[must_use]
    pub const fn get_max_duration(&self) -> &Nbt {
        &self.max_duration
    }
    #[must_use]
    pub const fn get_min_amplifier(&self) -> &Nbt {
        &self.min_amplifier
    }
    #[must_use]
    pub const fn get_max_amplifier(&self) -> &Nbt {
        &self.max_amplifier
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("to_apply".to_owned(), self.to_apply.to_nbt());
        compound.insert("min_duration".to_owned(), self.min_duration.clone());
        compound.insert("max_duration".to_owned(), self.max_duration.clone());
        compound.insert("min_amplifier".to_owned(), self.min_amplifier.clone());
        compound.insert("max_amplifier".to_owned(), self.max_amplifier.clone());
        compound
    }
}
impl EntityEffectChangeItemDamage {
    #[must_use]
    pub fn new(amount: Nbt) -> Self {
        Self { amount }
    }
    #[must_use]
    pub const fn get_amount(&self) -> &Nbt {
        &self.amount
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        single_field_nbt("amount", self.amount.clone())
    }
}
impl EntityEffectDamageEntity {
    #[must_use]
    pub fn new(damage_type: Identifier, min_damage: Nbt, max_damage: Nbt) -> Self {
        Self {
            damage_type,
            min_damage,
            max_damage,
        }
    }
    #[must_use]
    pub const fn get_damage_type(&self) -> &Identifier {
        &self.damage_type
    }
    #[must_use]
    pub const fn get_min_damage(&self) -> &Nbt {
        &self.min_damage
    }
    #[must_use]
    pub const fn get_max_damage(&self) -> &Nbt {
        &self.max_damage
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "damage_type".to_owned(),
            Nbt::String(self.damage_type.to_string()),
        );
        compound.insert("min_damage".to_owned(), self.min_damage.clone());
        compound.insert("max_damage".to_owned(), self.max_damage.clone());
        compound
    }
}
impl EntityEffectExplode {
    #[must_use]
    pub fn new(
        attribute_to_user: bool,
        damage_type: Option<Identifier>,
        knockback_multiplier: Option<Nbt>,
        immune_blocks: Option<Nbt>,
        offset: Nbt,
        radius: Nbt,
        create_fire: bool,
        block_interaction: Nbt,
        small_particle: Nbt,
        large_particle: Nbt,
        sound: Nbt,
    ) -> Self {
        Self {
            attribute_to_user,
            damage_type,
            knockback_multiplier,
            immune_blocks,
            offset,
            radius,
            create_fire,
            block_interaction,
            small_particle,
            large_particle,
            sound,
        }
    }
    #[must_use]
    pub const fn is_attributed_to_user(&self) -> bool {
        self.attribute_to_user
    }
    #[must_use]
    pub const fn get_damage_type(&self) -> Option<&Identifier> {
        self.damage_type.as_ref()
    }
    #[must_use]
    pub const fn get_knockback_multiplier(&self) -> Option<&Nbt> {
        self.knockback_multiplier.as_ref()
    }
    #[must_use]
    pub const fn get_immune_blocks(&self) -> Option<&Nbt> {
        self.immune_blocks.as_ref()
    }
    #[must_use]
    pub const fn get_offset(&self) -> &Nbt {
        &self.offset
    }
    #[must_use]
    pub const fn get_radius(&self) -> &Nbt {
        &self.radius
    }
    #[must_use]
    pub const fn does_create_fire(&self) -> bool {
        self.create_fire
    }
    #[must_use]
    pub const fn get_block_interaction(&self) -> &Nbt {
        &self.block_interaction
    }
    #[must_use]
    pub const fn get_small_particle(&self) -> &Nbt {
        &self.small_particle
    }
    #[must_use]
    pub const fn get_large_particle(&self) -> &Nbt {
        &self.large_particle
    }
    #[must_use]
    pub const fn get_sound(&self) -> &Nbt {
        &self.sound
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        if self.attribute_to_user {
            compound.insert("attribute_to_user".to_owned(), Nbt::Byte(1));
        }
        if let Some(damage_type) = &self.damage_type {
            compound.insert(
                "damage_type".to_owned(),
                Nbt::String(damage_type.to_string()),
            );
        }
        if let Some(knockback_multiplier) = &self.knockback_multiplier {
            compound.insert(
                "knockback_multiplier".to_owned(),
                knockback_multiplier.clone(),
            );
        }
        if let Some(immune_blocks) = &self.immune_blocks {
            compound.insert("immune_blocks".to_owned(), immune_blocks.clone());
        }
        compound.insert("offset".to_owned(), self.offset.clone());
        compound.insert("radius".to_owned(), self.radius.clone());
        if self.create_fire {
            compound.insert("create_fire".to_owned(), Nbt::Byte(1));
        }
        compound.insert(
            "block_interaction".to_owned(),
            self.block_interaction.clone(),
        );
        compound.insert("small_particle".to_owned(), self.small_particle.clone());
        compound.insert("large_particle".to_owned(), self.large_particle.clone());
        compound.insert("sound".to_owned(), self.sound.clone());
        compound
    }
}
impl EntityEffectIgnite {
    #[must_use]
    pub fn new(duration: Nbt) -> Self {
        Self { duration }
    }
    #[must_use]
    pub const fn get_duration(&self) -> &Nbt {
        &self.duration
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        single_field_nbt("duration", self.duration.clone())
    }
}
impl EntityEffectApplyImpulse {
    #[must_use]
    pub fn new(direction: Nbt, coordinate_scale: Nbt, magnitude: Nbt) -> Self {
        Self {
            direction,
            coordinate_scale,
            magnitude,
        }
    }
    #[must_use]
    pub const fn get_direction(&self) -> &Nbt {
        &self.direction
    }
    #[must_use]
    pub const fn get_coordinate_scale(&self) -> &Nbt {
        &self.coordinate_scale
    }
    #[must_use]
    pub const fn get_magnitude(&self) -> &Nbt {
        &self.magnitude
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("direction".to_owned(), self.direction.clone());
        compound.insert("coordinate_scale".to_owned(), self.coordinate_scale.clone());
        compound.insert("magnitude".to_owned(), self.magnitude.clone());
        compound
    }
}
impl EntityEffectApplyExhaustion {
    #[must_use]
    pub fn new(amount: Nbt) -> Self {
        Self { amount }
    }
    #[must_use]
    pub const fn get_amount(&self) -> &Nbt {
        &self.amount
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        single_field_nbt("amount", self.amount.clone())
    }
}
impl EntityEffectPlaySound {
    #[must_use]
    pub fn new(sounds: Vec<Nbt>, volume: Nbt, pitch: Nbt) -> Self {
        Self {
            sounds,
            volume,
            pitch,
        }
    }
    #[must_use]
    pub fn get_sounds(&self) -> &[Nbt] {
        &self.sounds
    }
    #[must_use]
    pub const fn get_volume(&self) -> &Nbt {
        &self.volume
    }
    #[must_use]
    pub const fn get_pitch(&self) -> &Nbt {
        &self.pitch
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert(
            "sound".to_owned(),
            Nbt::List(self.sounds.clone().into_boxed_slice()),
        );
        compound.insert("volume".to_owned(), self.volume.clone());
        compound.insert("pitch".to_owned(), self.pitch.clone());
        compound
    }
}
impl EntityEffectReplaceBlock {
    #[must_use]
    pub fn new(offset: Nbt, predicate: Nbt, block_state: Nbt, trigger_game_event: Nbt) -> Self {
        Self {
            offset,
            predicate,
            block_state,
            trigger_game_event,
        }
    }
    #[must_use]
    pub const fn get_offset(&self) -> &Nbt {
        &self.offset
    }
    #[must_use]
    pub const fn get_predicate(&self) -> &Nbt {
        &self.predicate
    }
    #[must_use]
    pub const fn get_block_state(&self) -> &Nbt {
        &self.block_state
    }
    #[must_use]
    pub const fn get_trigger_game_event(&self) -> &Nbt {
        &self.trigger_game_event
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("offset".to_owned(), self.offset.clone());
        compound.insert("predicate".to_owned(), self.predicate.clone());
        compound.insert("block_state".to_owned(), self.block_state.clone());
        compound.insert(
            "trigger_game_event".to_owned(),
            self.trigger_game_event.clone(),
        );
        compound
    }
}
impl EntityEffectReplaceDisk {
    #[must_use]
    pub fn new(
        radius: Nbt,
        height: Nbt,
        offset: Nbt,
        predicate: Option<Nbt>,
        block_state: Nbt,
        trigger_game_event: Option<Nbt>,
    ) -> Self {
        Self {
            radius,
            height,
            offset,
            predicate,
            block_state,
            trigger_game_event,
        }
    }
    #[must_use]
    pub const fn get_radius(&self) -> &Nbt {
        &self.radius
    }
    #[must_use]
    pub const fn get_height(&self) -> &Nbt {
        &self.height
    }
    #[must_use]
    pub const fn get_offset(&self) -> &Nbt {
        &self.offset
    }
    #[must_use]
    pub const fn get_predicate(&self) -> Option<&Nbt> {
        self.predicate.as_ref()
    }
    #[must_use]
    pub const fn get_block_state(&self) -> &Nbt {
        &self.block_state
    }
    #[must_use]
    pub const fn get_trigger_game_event(&self) -> Option<&Nbt> {
        self.trigger_game_event.as_ref()
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("radius".to_owned(), self.radius.clone());
        compound.insert("height".to_owned(), self.height.clone());
        compound.insert("offset".to_owned(), self.offset.clone());
        if let Some(predicate) = &self.predicate {
            compound.insert("predicate".to_owned(), predicate.clone());
        }
        compound.insert("block_state".to_owned(), self.block_state.clone());
        if let Some(trigger_game_event) = &self.trigger_game_event {
            compound.insert("trigger_game_event".to_owned(), trigger_game_event.clone());
        }
        compound
    }
}
impl EntityEffectRunFunction {
    #[must_use]
    pub fn new(function: impl Into<String>) -> Self {
        Self {
            function: function.into(),
        }
    }
    #[must_use]
    pub fn get_function(&self) -> &str {
        &self.function
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        single_field_nbt("function", Nbt::String(self.function.clone()))
    }
}
impl EntityEffectSpawnParticles {
    #[must_use]
    pub fn new(
        particle: Nbt,
        horizontal_position: Nbt,
        vertical_position: Nbt,
        horizontal_velocity: Nbt,
        vertical_velocity: Nbt,
        speed: Nbt,
    ) -> Self {
        Self {
            particle,
            horizontal_position,
            vertical_position,
            horizontal_velocity,
            vertical_velocity,
            speed,
        }
    }
    #[must_use]
    pub const fn get_particle(&self) -> &Nbt {
        &self.particle
    }
    #[must_use]
    pub const fn get_horizontal_position(&self) -> &Nbt {
        &self.horizontal_position
    }
    #[must_use]
    pub const fn get_vertical_position(&self) -> &Nbt {
        &self.vertical_position
    }
    #[must_use]
    pub const fn get_horizontal_velocity(&self) -> &Nbt {
        &self.horizontal_velocity
    }
    #[must_use]
    pub const fn get_vertical_velocity(&self) -> &Nbt {
        &self.vertical_velocity
    }
    #[must_use]
    pub const fn get_speed(&self) -> &Nbt {
        &self.speed
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("particle".to_owned(), self.particle.clone());
        compound.insert(
            "horizontal_position".to_owned(),
            self.horizontal_position.clone(),
        );
        compound.insert(
            "vertical_position".to_owned(),
            self.vertical_position.clone(),
        );
        compound.insert(
            "horizontal_velocity".to_owned(),
            self.horizontal_velocity.clone(),
        );
        compound.insert(
            "vertical_velocity".to_owned(),
            self.vertical_velocity.clone(),
        );
        compound.insert("speed".to_owned(), self.speed.clone());
        compound
    }
}
impl EntityEffectSetBlockProperties {
    #[must_use]
    pub fn new(properties: Nbt, offset: Nbt, trigger_game_event: Option<Nbt>) -> Self {
        Self {
            properties,
            offset,
            trigger_game_event,
        }
    }
    #[must_use]
    pub const fn get_properties(&self) -> &Nbt {
        &self.properties
    }
    #[must_use]
    pub const fn get_offset(&self) -> &Nbt {
        &self.offset
    }
    #[must_use]
    pub const fn get_trigger_game_event(&self) -> Option<&Nbt> {
        self.trigger_game_event.as_ref()
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("properties".to_owned(), self.properties.clone());
        compound.insert("offset".to_owned(), self.offset.clone());
        if let Some(trigger_game_event) = &self.trigger_game_event {
            compound.insert("trigger_game_event".to_owned(), trigger_game_event.clone());
        }
        compound
    }
}
impl EntityEffectSummonEntity {
    #[must_use]
    pub fn new(entity: Nbt, join_team: bool) -> Self {
        Self { entity, join_team }
    }
    #[must_use]
    pub const fn get_entity(&self) -> &Nbt {
        &self.entity
    }
    #[must_use]
    pub const fn should_join_team(&self) -> bool {
        self.join_team
    }
    #[must_use]
    pub fn to_nbt_compound(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        compound.insert("entity".to_owned(), self.entity.clone());
        if self.join_team {
            compound.insert("join_team".to_owned(), Nbt::Byte(1));
        }
        compound
    }
}
impl EnchantmentTarget {
    #[must_use]
    pub const fn nbt_name(self) -> &'static str {
        match self {
            Self::Attacker => "attacker",
            Self::DamagingEntity => "damaging_entity",
            Self::Victim => "victim",
        }
    }
    #[must_use]
    pub fn from_nbt_name(name: &str) -> Option<Self> {
        match name.as_bytes() {
            b"attacker" => Some(Self::Attacker),
            b"damaging_entity" => Some(Self::DamagingEntity),
            b"victim" => Some(Self::Victim),
            _ => None,
        }
    }
}
fn single_field_nbt(name: &str, value: Nbt) -> NbtCompound {
    let mut compound = NbtCompound::new();
    compound.insert(name.to_owned(), value);
    compound
}
fn tagged_payload_nbt(type_key: &Identifier, payload: &NbtCompound) -> NbtCompound {
    let mut compound = payload.clone();
    compound.insert("type".to_owned(), Nbt::String(type_key.to_string()));
    compound
}
impl EntityEffect {
    #[must_use]
    pub fn from_nbt_compound(compound: &NbtCompound) -> Option<Self> {
        match identifier_field(compound, "type")?.path.as_ref() {
            "all_of" => decode_all_of_entity_effect(compound),
            "apply_mob_effect" => decode_apply_mob_effect_entity_effect(compound),
            "change_item_damage" => decode_change_item_damage_entity_effect(compound),
            "damage_entity" => decode_damage_entity_effect(compound),
            "explode" => decode_explode_entity_effect(compound),
            "ignite" => decode_ignite_entity_effect(compound),
            "apply_impulse" => decode_apply_impulse_entity_effect(compound),
            "apply_exhaustion" => decode_apply_exhaustion_entity_effect(compound),
            "play_sound" => decode_play_sound_entity_effect(compound),
            "replace_block" => decode_replace_block_entity_effect(compound),
            "replace_disk" => decode_replace_disk_entity_effect(compound),
            "run_function" => decode_run_function_entity_effect(compound),
            "spawn_particles" => decode_spawn_particles_entity_effect(compound),
            "set_block_properties" => decode_set_block_properties_entity_effect(compound),
            "summon_entity" => decode_summon_entity_entity_effect(compound),
            _ => None,
        }
    }
}
fn decode_all_of_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    let effects = match compound.get("effects")? {
        Nbt::List(effects) => effects
            .iter()
            .map(|effect| match effect {
                Nbt::Compound(effect) => EntityEffect::from_nbt_compound(effect),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?,
        _ => return None,
    };
    Some(EntityEffect::AllOf(EntityEffectAllOf::new(effects)))
}
fn decode_apply_mob_effect_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::ApplyMobEffect(
        EntityEffectApplyMobEffect::new(
            RegistryTagReference::from_nbt(compound.get("to_apply")?)?,
            required_nbt_field(compound, "min_duration")?,
            required_nbt_field(compound, "max_duration")?,
            required_nbt_field(compound, "min_amplifier")?,
            required_nbt_field(compound, "max_amplifier")?,
        ),
    ))
}
fn decode_change_item_damage_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::ChangeItemDamage(
        EntityEffectChangeItemDamage::new(required_nbt_field(compound, "amount")?),
    ))
}
fn decode_damage_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::DamageEntity(EntityEffectDamageEntity::new(
        identifier_field(compound, "damage_type")?,
        required_nbt_field(compound, "min_damage")?,
        required_nbt_field(compound, "max_damage")?,
    )))
}
fn decode_explode_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::Explode(EntityEffectExplode::new(
        bool_field_or(compound, "attribute_to_user", false)?,
        optional_identifier_field(compound, "damage_type")?,
        optional_nbt_field(compound, "knockback_multiplier"),
        optional_nbt_field(compound, "immune_blocks"),
        optional_nbt_field(compound, "offset").unwrap_or_else(zero_vector_nbt),
        required_nbt_field(compound, "radius")?,
        bool_field_or(compound, "create_fire", false)?,
        required_nbt_field(compound, "block_interaction")?,
        required_nbt_field(compound, "small_particle")?,
        required_nbt_field(compound, "large_particle")?,
        required_nbt_field(compound, "sound")?,
    )))
}
fn decode_ignite_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::Ignite(EntityEffectIgnite::new(
        required_nbt_field(compound, "duration")?,
    )))
}
fn decode_apply_impulse_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::ApplyImpulse(EntityEffectApplyImpulse::new(
        required_nbt_field(compound, "direction")?,
        required_nbt_field(compound, "coordinate_scale")?,
        required_nbt_field(compound, "magnitude")?,
    )))
}
fn decode_apply_exhaustion_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::ApplyExhaustion(
        EntityEffectApplyExhaustion::new(required_nbt_field(compound, "amount")?),
    ))
}
fn decode_play_sound_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    let sounds = match compound.get("sound")? {
        Nbt::List(sounds) => sounds.iter().cloned().collect(),
        sound => vec![sound.clone()],
    };
    Some(EntityEffect::PlaySound(EntityEffectPlaySound::new(
        sounds,
        required_nbt_field(compound, "volume")?,
        required_nbt_field(compound, "pitch")?,
    )))
}
fn decode_replace_block_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::ReplaceBlock(EntityEffectReplaceBlock::new(
        required_nbt_field(compound, "offset")?,
        required_nbt_field(compound, "predicate")?,
        required_nbt_field(compound, "block_state")?,
        required_nbt_field(compound, "trigger_game_event")?,
    )))
}
fn decode_replace_disk_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::ReplaceDisk(EntityEffectReplaceDisk::new(
        required_nbt_field(compound, "radius")?,
        required_nbt_field(compound, "height")?,
        required_nbt_field(compound, "offset")?,
        optional_nbt_field(compound, "predicate"),
        required_nbt_field(compound, "block_state")?,
        optional_nbt_field(compound, "trigger_game_event"),
    )))
}
fn decode_run_function_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    match compound.get("function")? {
        Nbt::String(function) => Some(EntityEffect::RunFunction(EntityEffectRunFunction::new(
            function.clone(),
        ))),
        _ => None,
    }
}
fn decode_spawn_particles_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::SpawnParticles(
        EntityEffectSpawnParticles::new(
            required_nbt_field(compound, "particle")?,
            required_nbt_field(compound, "horizontal_position")?,
            required_nbt_field(compound, "vertical_position")?,
            required_nbt_field(compound, "horizontal_velocity")?,
            required_nbt_field(compound, "vertical_velocity")?,
            required_nbt_field(compound, "speed")?,
        ),
    ))
}
fn decode_set_block_properties_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::SetBlockProperties(
        EntityEffectSetBlockProperties::new(
            required_nbt_field(compound, "properties")?,
            required_nbt_field(compound, "offset")?,
            optional_nbt_field(compound, "trigger_game_event"),
        ),
    ))
}
fn decode_summon_entity_entity_effect(compound: &NbtCompound) -> Option<EntityEffect> {
    Some(EntityEffect::SummonEntity(EntityEffectSummonEntity::new(
        required_nbt_field(compound, "entity")?,
        bool_field_or(compound, "join_team", false)?,
    )))
}
fn required_nbt_field(compound: &NbtCompound, name: &str) -> Option<Nbt> {
    compound.get(name).cloned()
}
fn optional_nbt_field(compound: &NbtCompound, name: &str) -> Option<Nbt> {
    compound.get(name).cloned()
}
fn identifier_field(compound: &NbtCompound, name: &str) -> Option<Identifier> {
    match compound.get(name)? {
        Nbt::String(value) => value.parse().ok(),
        _ => None,
    }
}
fn optional_identifier_field(compound: &NbtCompound, name: &str) -> Option<Option<Identifier>> {
    match compound.get(name) {
        Some(Nbt::String(value)) => Some(Some(value.parse().ok()?)),
        Some(_) => None,
        None => Some(None),
    }
}
fn bool_field_or(compound: &NbtCompound, name: &str, default_value: bool) -> Option<bool> {
    match compound.get(name) {
        Some(Nbt::Byte(value)) => Some(*value != 0),
        Some(Nbt::Int(value)) => Some(*value != 0),
        Some(_) => None,
        None => Some(default_value),
    }
}
fn zero_vector_nbt() -> Nbt {
    let mut vector = NbtCompound::new();
    vector.insert("x".to_owned(), Nbt::Double(0.0));
    vector.insert("y".to_owned(), Nbt::Double(0.0));
    vector.insert("z".to_owned(), Nbt::Double(0.0));
    Nbt::Compound(vector)
}
