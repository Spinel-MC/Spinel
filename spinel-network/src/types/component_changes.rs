use crate::data_type::DataType;
use crate::types::game_profile::{
    GameProfile as NetworkGameProfile, GameProfileProperty as NetworkGameProfileProperty,
};
use crate::types::position::Position;
use crate::types::resolvable_profile::{
    PartialGameProfile, PlayerSkinPatch, ResolvableProfile as NetworkResolvableProfile,
    ResolvableProfileIdentity,
};
use crate::types::slot::Slot;
use crate::types::sound::SoundEvent;
use crate::types::var_int::VarIntWrapper;
use crate::wrappers::{JsonTextComponent, NbtTextComponent};
use spinel_nbt::Nbt;
use spinel_registry::block_entity_type::BlockEntityType;
use spinel_registry::data_components::vanilla_components::{
    ATTACK_RANGE, ATTRIBUTE_MODIFIERS, AXOLOTL_VARIANT, BANNER_PATTERNS, BASE_COLOR, BEES,
    BLOCK_ENTITY_DATA, BLOCK_STATE, BLOCKS_ATTACKS, BREAK_SOUND, BUCKET_ENTITY_DATA,
    BUNDLE_CONTENTS, CAN_BREAK, CAN_PLACE_ON, CAT_COLLAR, CAT_VARIANT, CHARGED_PROJECTILES,
    CHICKEN_VARIANT, CONSUMABLE, COW_VARIANT, CREATIVE_SLOT_LOCK, CUSTOM_DATA, CUSTOM_MODEL_DATA,
    CUSTOM_NAME, DAMAGE, DAMAGE_RESISTANT, DAMAGE_TYPE, DEATH_PROTECTION, DEBUG_STICK_STATE,
    DYED_COLOR, ENCHANTABLE, ENCHANTMENT_GLINT_OVERRIDE, ENCHANTMENTS, ENTITY_DATA, EQUIPPABLE,
    FIREWORK_EXPLOSION, FIREWORKS, FOOD, FOX_VARIANT, FROG_VARIANT, GLIDER, HORSE_VARIANT,
    INSTRUMENT, ITEM_MODEL, ITEM_NAME, JUKEBOX_PLAYABLE, KINETIC_WEAPON, LLAMA_VARIANT,
    LODESTONE_TRACKER, LORE, MAP_COLOR, MAP_ID, MAP_POST_PROCESSING, MAX_DAMAGE, MAX_STACK_SIZE,
    MINIMUM_ATTACK_CHARGE, MOOSHROOM_VARIANT, NOTE_BLOCK_SOUND, OMINOUS_BOTTLE_AMPLIFIER,
    PAINTING_VARIANT, PARROT_VARIANT, PIERCING_WEAPON, PIG_VARIANT, POT_DECORATIONS,
    POTION_CONTENTS, POTION_DURATION_SCALE, PROFILE, PROVIDES_BANNER_PATTERNS,
    PROVIDES_TRIM_MATERIAL, RABBIT_VARIANT, RARITY, RECIPES, REPAIR_COST, REPAIRABLE, SALMON_SIZE,
    SHEEP_COLOR, SHULKER_COLOR, STORED_ENCHANTMENTS, SUSPICIOUS_STEW_EFFECTS, SWING_ANIMATION,
    TOOL, TOOLTIP_DISPLAY, TOOLTIP_STYLE, TRIM, TROPICAL_FISH_BASE_COLOR, TROPICAL_FISH_PATTERN,
    TROPICAL_FISH_PATTERN_COLOR, UNBREAKABLE, USE_COOLDOWN, USE_EFFECTS, USE_REMAINDER,
    VILLAGER_VARIANT, WEAPON, WOLF_COLLAR, WOLF_SOUND_VARIANT, WOLF_VARIANT, WRITABLE_BOOK_CONTENT,
    WRITTEN_BOOK_CONTENT, ZOMBIE_NAUTILUS_VARIANT,
};
use spinel_registry::vanilla_world_blocks::Block;
use spinel_registry::{
    ArmorTrim, AttackRange, AttributeList, AttributeModifierDisplay, BANNER_PATTERN_REGISTRY,
    BannerPatterns, Bee, BlockPredicates, BlocksAttacks, BuiltinSoundEvent, CAT_VARIANT_REGISTRY,
    CHICKEN_VARIANT_REGISTRY, COW_VARIANT_REGISTRY, Consumable, CustomModelData,
    CustomPotionEffect, DAMAGE_TYPE_REGISTRY, DamageResistant, DataComponentDescriptor,
    DataComponentMap, DataComponentValue, DeathProtection, DebugStickState, ENCHANTMENT_REGISTRY,
    EnchantmentList, EntityType, Equippable, FROG_VARIANT_REGISTRY, FireworkExplosion,
    FireworkList, Food, InstrumentComponent, ItemBlockState, ItemRarity, ItemStack,
    JUKEBOX_SONG_REGISTRY, KineticWeapon, KineticWeaponCondition, LodestoneTracker,
    MapPostProcessing, Material, PAINTING_VARIANT_REGISTRY, PIG_VARIANT_REGISTRY, PiercingWeapon,
    PotDecorations, PotionContents, PotionEffectSettings, PropertyValuePredicate, Registries,
    RegistryTagReference, ResolvableProfile as ComponentResolvableProfile, SuspiciousStewEffects,
    SwingAnimation, TRIM_MATERIAL_REGISTRY, TRIM_PATTERN_REGISTRY, Tool, TooltipDisplay,
    TypedCustomData, UseCooldown, UseEffects, WOLF_SOUND_VARIANT_REGISTRY, WOLF_VARIANT_REGISTRY,
    Weapon, WorldPosition, WritableBookContent, WrittenBookContent,
    ZOMBIE_NAUTILUS_VARIANT_REGISTRY, dye_color_protocol_id,
};
use spinel_utils::color::{Color, DyeColor};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Cursor, Read, Write};
use std::sync::LazyLock;
use uuid::Uuid;

static VANILLA_REGISTRIES: LazyLock<Registries> = LazyLock::new(Registries::new_vanilla);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComponentChanges {
    pub added: Vec<ComponentEntry>,
    pub removed: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentEntry {
    pub type_id: i32,
    pub data: Vec<u8>,
}

impl DataType for ComponentChanges {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.added.len() as i32).encode(w)?;
        VarIntWrapper(self.removed.len() as i32).encode(w)?;

        for entry in &self.added {
            VarIntWrapper(entry.type_id).encode(w)?;
            w.write_all(&entry.data)?;
        }

        for type_id in &self.removed {
            VarIntWrapper(*type_id).encode(w)?;
        }

        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let (added_count, removed_count) = decode_component_counts(r)?;

        let mut added = Vec::with_capacity(added_count);
        for _ in 0..added_count {
            let type_id = VarIntWrapper::decode(r)?.0;
            validate_added_component(type_id)?;
            added.push(ComponentEntry {
                type_id,
                data: decode_component_payload(type_id, r)?,
            });
        }

        let mut removed = Vec::with_capacity(removed_count);
        for _ in 0..removed_count {
            let type_id = VarIntWrapper::decode(r)?.0;
            validate_removed_component(type_id)?;
            removed.push(type_id);
        }

        Ok(ComponentChanges { added, removed })
    }
}

impl ComponentChanges {
    pub(crate) fn encode_length_prefixed<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.added.len() as i32).encode(writer)?;
        VarIntWrapper(self.removed.len() as i32).encode(writer)?;
        for component in &self.added {
            VarIntWrapper(component.type_id).encode(writer)?;
            VarIntWrapper(component.data.len() as i32).encode(writer)?;
            writer.write_all(&component.data)?;
        }
        for component_id in &self.removed {
            VarIntWrapper(*component_id).encode(writer)?;
        }
        Ok(())
    }

    pub(crate) fn decode_length_prefixed<R: Read>(reader: &mut R) -> io::Result<Self> {
        let (added_count, removed_count) = decode_component_counts(reader)?;
        let mut added = Vec::with_capacity(added_count);
        for _ in 0..added_count {
            let type_id = VarIntWrapper::decode(reader)?.0;
            validate_added_component(type_id)?;
            let payload_length = VarIntWrapper::decode(reader)?.0;
            if payload_length < 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Negative component payload length: {payload_length}"),
                ));
            }
            let mut encoded_payload = vec![0; payload_length as usize];
            reader.read_exact(&mut encoded_payload)?;
            let mut payload_reader = Cursor::new(encoded_payload);
            let data = decode_component_payload(type_id, &mut payload_reader)?;
            if payload_reader.position() != payload_reader.get_ref().len() as u64 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Component payload was not fully consumed: {type_id}"),
                ));
            }
            added.push(ComponentEntry { type_id, data });
        }
        let mut removed = Vec::with_capacity(removed_count);
        for _ in 0..removed_count {
            let type_id = VarIntWrapper::decode(reader)?.0;
            validate_removed_component(type_id)?;
            removed.push(type_id);
        }
        Ok(Self { added, removed })
    }

    pub fn custom_data(&self) -> io::Result<Option<spinel_nbt::NbtCompound>> {
        self.decode_component(CUSTOM_DATA.id())
    }

    pub fn custom_name(&self) -> io::Result<Option<TextComponent>> {
        self.decode_component::<NbtTextComponent>(CUSTOM_NAME.id())
            .map(|component| component.map(TextComponent::from))
    }

    fn decode_component<T: DataType>(&self, component_id: i32) -> io::Result<Option<T>> {
        self.added
            .iter()
            .find(|component| component.type_id == component_id)
            .map(|component| {
                let mut component_payload = component.data.as_slice();
                let decoded_component = T::decode(&mut component_payload)?;
                if !component_payload.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Component payload was not fully consumed: {component_id}"),
                    ));
                }
                Ok(decoded_component)
            })
            .transpose()
    }
}

fn decode_component_counts<R: Read>(reader: &mut R) -> io::Result<(usize, usize)> {
    let added_count = VarIntWrapper::decode(reader)?.0;
    let removed_count = VarIntWrapper::decode(reader)?.0;
    if added_count < 0 || removed_count < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Data component counts must not be negative",
        ));
    }
    let total_count = added_count as usize + removed_count as usize;
    if total_count > 256 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Data component map too large: {total_count}"),
        ));
    }
    Ok((added_count as usize, removed_count as usize))
}

fn validate_added_component(type_id: i32) -> io::Result<()> {
    let component_descriptor = DataComponentDescriptor::from_id(type_id).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown data component id: {type_id}"),
        )
    })?;
    if !component_descriptor.is_synced() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Data component has no network codec: {type_id}"),
        ));
    }
    Ok(())
}

fn validate_removed_component(type_id: i32) -> io::Result<()> {
    DataComponentDescriptor::from_id(type_id)
        .map(|_| ())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown data component id: {type_id}"),
            )
        })
}

struct CapturingReader<'a, R: Read> {
    inner: &'a mut R,
    bytes: Vec<u8>,
}

impl<'a, R: Read> CapturingReader<'a, R> {
    fn new(inner: &'a mut R) -> Self {
        Self {
            inner,
            bytes: Vec::new(),
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl<R: Read> Read for CapturingReader<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let read_count = self.inner.read(buffer)?;
        self.bytes.extend_from_slice(&buffer[..read_count]);
        Ok(read_count)
    }
}

fn capture_component_payload<R: Read, F>(reader: &mut R, decode: F) -> io::Result<Vec<u8>>
where
    F: FnOnce(&mut CapturingReader<'_, R>) -> io::Result<()>,
{
    let mut capturing_reader = CapturingReader::new(reader);
    decode(&mut capturing_reader)?;
    Ok(capturing_reader.into_bytes())
}

fn decode_data_type_payload<T: DataType, R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        T::decode(capturing_reader).map(|_| ())
    })
}

fn decode_list_payload<R: Read, F>(
    reader: &mut R,
    maximum_length: i32,
    mut decode_entry: F,
) -> io::Result<()>
where
    F: FnMut(&mut R) -> io::Result<()>,
{
    let entry_count = VarIntWrapper::decode(reader)?.0;
    if entry_count < 0 || entry_count > maximum_length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Component list length out of bounds: {entry_count}"),
        ));
    }
    for _ in 0..entry_count {
        decode_entry(reader)?;
    }
    Ok(())
}

fn decode_string_list_payload<R: Read>(reader: &mut R, maximum_length: i32) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, maximum_length, |entry_reader| {
            String::decode(entry_reader).map(|_| ())
        })
    })
}

fn decode_text_component_list_payload<R: Read>(
    reader: &mut R,
    maximum_length: i32,
) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, maximum_length, |entry_reader| {
            NbtTextComponent::decode(entry_reader).map(|_| ())
        })
    })
}

fn decode_food_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        VarIntWrapper::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_consumable_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        f32::decode(capturing_reader)?;
        VarIntWrapper::decode(capturing_reader)?;
        SoundEvent::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        decode_consume_effects_payload(capturing_reader, i16::MAX as i32)
    })
}

fn decode_consume_effects_payload<R: Read>(reader: &mut R, maximum_length: i32) -> io::Result<()> {
    decode_list_payload(
        reader,
        maximum_length,
        |entry_reader| match VarIntWrapper::decode(entry_reader)?.0 {
            0 => {
                decode_list_payload(entry_reader, 256, |effect_reader| {
                    decode_custom_potion_effect_payload(effect_reader)
                })?;
                f32::decode(entry_reader)?;
                Ok(())
            }
            1 => decode_registry_tag_reference_payload(entry_reader),
            2 => Ok(()),
            3 => f32::decode(entry_reader).map(|_| ()),
            4 => SoundEvent::decode(entry_reader).map(|_| ()),
            effect_type => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown consume effect type: {effect_type}"),
            )),
        },
    )
}

fn decode_custom_potion_effect_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    VarIntWrapper::decode(reader)?;
    decode_potion_effect_settings_payload(reader, 0)
}

fn decode_potion_effect_settings_payload<R: Read>(
    reader: &mut R,
    hidden_effect_depth: usize,
) -> io::Result<()> {
    if hidden_effect_depth > 64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Potion effect hidden effect nesting is too deep",
        ));
    }
    VarIntWrapper::decode(reader)?;
    VarIntWrapper::decode(reader)?;
    bool::decode(reader)?;
    bool::decode(reader)?;
    bool::decode(reader)?;
    if bool::decode(reader)? {
        decode_potion_effect_settings_payload(reader, hidden_effect_depth + 1)?;
    }
    Ok(())
}

fn decode_registry_tag_reference_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    let reference_type = VarIntWrapper::decode(reader)?.0;
    if reference_type == 0 {
        String::decode(reader)?;
        return Ok(());
    }
    if reference_type == 1 {
        return Ok(());
    }
    if reference_type < 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid registry tag reference type: {reference_type}"),
        ));
    }
    for _ in 0..reference_type - 1 {
        VarIntWrapper::decode(reader)?;
    }
    Ok(())
}

fn decode_optional_registry_tag_reference_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    if bool::decode(reader)? {
        decode_registry_tag_reference_payload(reader)?;
    }
    Ok(())
}

fn decode_use_effects_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        bool::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_use_cooldown_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        f32::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_potion_contents_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        Option::<VarIntWrapper>::decode(capturing_reader)?;
        Option::<i32>::decode(capturing_reader)?;
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            decode_custom_potion_effect_payload(entry_reader)
        })?;
        Option::<String>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_suspicious_stew_effects_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            VarIntWrapper::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_bees_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            spinel_nbt::NbtCompound::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_attribute_modifiers_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            VarIntWrapper::decode(entry_reader)?;
            String::decode(entry_reader)?;
            f64::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            decode_attribute_modifier_display_payload(entry_reader)
        })
    })
}

fn decode_attribute_modifier_display_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    match VarIntWrapper::decode(reader)?.0 {
        0 | 1 => Ok(()),
        2 => JsonTextComponent::decode(reader).map(|_| ()),
        display_type => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown attribute modifier display type: {display_type}"),
        )),
    }
}

fn decode_var_int_map_payload<R: Read>(reader: &mut R, maximum_length: i32) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, maximum_length, |entry_reader| {
            VarIntWrapper::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_registry_holder_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    decode_data_type_payload::<VarIntWrapper, R>(reader)
}

fn decode_two_registry_holders_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        VarIntWrapper::decode(capturing_reader)?;
        VarIntWrapper::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_jukebox_playable_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        if bool::decode(capturing_reader)? {
            VarIntWrapper::decode(capturing_reader)?;
            return Ok(());
        }
        String::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_provides_trim_material_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        if bool::decode(capturing_reader)? {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Inline trim material payload is not supported by this component decoder",
            ));
        }
        String::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_either_registry_key_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        if bool::decode(capturing_reader)? {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Inline registry holder payload is not supported by this component decoder",
            ));
        }
        String::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_tag_key_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    decode_data_type_payload::<String, R>(reader)
}

fn decode_banner_patterns_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, 1024, |entry_reader| {
            VarIntWrapper::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_tool_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            decode_registry_tag_reference_payload(entry_reader)?;
            f32::decode(entry_reader)?;
            bool::decode(entry_reader)?;
            Ok(())
        })?;
        f32::decode(capturing_reader)?;
        VarIntWrapper::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_block_predicates_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            decode_optional_registry_tag_reference_payload(entry_reader)?;
            decode_optional_properties_predicate_payload(entry_reader)?;
            Option::<spinel_nbt::NbtCompound>::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            VarIntWrapper::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_optional_properties_predicate_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    if bool::decode(reader)? {
        decode_list_payload(reader, i16::MAX as i32, |entry_reader| {
            String::decode(entry_reader)?;
            decode_property_value_predicate_payload(entry_reader)
        })?;
    }
    Ok(())
}

fn decode_property_value_predicate_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    if bool::decode(reader)? {
        String::decode(reader)?;
        return Ok(());
    }
    Option::<String>::decode(reader)?;
    Option::<String>::decode(reader)?;
    Ok(())
}

fn decode_equippable_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        VarIntWrapper::decode(capturing_reader)?;
        SoundEvent::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        decode_optional_registry_tag_reference_payload(capturing_reader)?;
        bool::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        SoundEvent::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_blocks_attacks_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            f32::decode(entry_reader)?;
            Option::<i32>::decode(entry_reader)?;
            f32::decode(entry_reader)?;
            f32::decode(entry_reader)?;
            Ok(())
        })?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        Option::<SoundEvent>::decode(capturing_reader)?;
        Option::<SoundEvent>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_piercing_weapon_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        bool::decode(capturing_reader)?;
        bool::decode(capturing_reader)?;
        Option::<SoundEvent>::decode(capturing_reader)?;
        Option::<SoundEvent>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_kinetic_weapon_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        VarIntWrapper::decode(capturing_reader)?;
        VarIntWrapper::decode(capturing_reader)?;
        decode_optional_kinetic_weapon_condition_payload(capturing_reader)?;
        decode_optional_kinetic_weapon_condition_payload(capturing_reader)?;
        decode_optional_kinetic_weapon_condition_payload(capturing_reader)?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        Option::<SoundEvent>::decode(capturing_reader)?;
        Option::<SoundEvent>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_optional_kinetic_weapon_condition_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    if bool::decode(reader)? {
        VarIntWrapper::decode(reader)?;
        f32::decode(reader)?;
        f32::decode(reader)?;
    }
    Ok(())
}

fn decode_lodestone_tracker_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        if bool::decode(capturing_reader)? {
            String::decode(capturing_reader)?;
            Position::decode(capturing_reader)?;
        }
        bool::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_attack_range_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        f32::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_custom_model_data_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        Vec::<f32>::decode(capturing_reader)?;
        Vec::<bool>::decode(capturing_reader)?;
        Vec::<String>::decode(capturing_reader)?;
        Vec::<i32>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_tooltip_display_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        bool::decode(capturing_reader)?;
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            VarIntWrapper::decode(entry_reader).map(|_| ())
        })
    })
}

fn decode_firework_explosion_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_firework_explosion_value_payload(capturing_reader)
    })
}

fn decode_firework_list_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        VarIntWrapper::decode(capturing_reader)?;
        decode_list_payload(capturing_reader, 256, |entry_reader| {
            decode_firework_explosion_value_payload(entry_reader)
        })
    })
}

fn decode_firework_explosion_value_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    VarIntWrapper::decode(reader)?;
    Vec::<i32>::decode(reader)?;
    Vec::<i32>::decode(reader)?;
    bool::decode(reader)?;
    bool::decode(reader)?;
    Ok(())
}

fn decode_pot_decorations_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, 4, |entry_reader| {
            VarIntWrapper::decode(entry_reader).map(|_| ())
        })
    })
}

fn decode_item_block_state_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, i16::MAX as i32, |entry_reader| {
            String::decode(entry_reader)?;
            String::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_filtered_string_list_payload<R: Read>(
    reader: &mut R,
    maximum_length: i32,
) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, maximum_length, |entry_reader| {
            String::decode(entry_reader)?;
            Option::<String>::decode(entry_reader)?;
            Ok(())
        })
    })
}

fn decode_written_book_content_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        String::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        String::decode(capturing_reader)?;
        VarIntWrapper::decode(capturing_reader)?;
        decode_list_payload(capturing_reader, 100, |entry_reader| {
            JsonTextComponent::decode(entry_reader)?;
            Option::<JsonTextComponent>::decode(entry_reader)?;
            Ok(())
        })?;
        bool::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_profile_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        if bool::decode(capturing_reader)? {
            Uuid::decode(capturing_reader)?;
            String::decode(capturing_reader)?;
        } else {
            Option::<String>::decode(capturing_reader)?;
            Option::<Uuid>::decode(capturing_reader)?;
        }
        decode_profile_properties_payload(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        Option::<String>::decode(capturing_reader)?;
        Option::<bool>::decode(capturing_reader)?;
        Ok(())
    })
}

fn decode_profile_properties_payload<R: Read>(reader: &mut R) -> io::Result<()> {
    decode_list_payload(reader, 1024, |entry_reader| {
        String::decode(entry_reader)?;
        String::decode(entry_reader)?;
        Option::<String>::decode(entry_reader)?;
        Ok(())
    })
}

fn decode_item_stack_payload<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_item_stack_fields_payload(capturing_reader, 0)
    })
}

fn decode_item_stack_list_component_payload<R: Read>(
    reader: &mut R,
    maximum_length: i32,
) -> io::Result<Vec<u8>> {
    capture_component_payload(reader, |capturing_reader| {
        decode_list_payload(capturing_reader, maximum_length, |entry_reader| {
            decode_item_stack_fields_payload(entry_reader, 0)
        })
    })
}

fn decode_item_stack_fields_payload<R: Read>(reader: &mut R, depth: usize) -> io::Result<()> {
    if depth > 64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Nested item stack component depth is too deep",
        ));
    }
    let count = VarIntWrapper::decode(reader)?.0;
    if count <= 0 {
        return Ok(());
    }
    VarIntWrapper::decode(reader)?;
    decode_nested_component_changes_payload(reader, depth + 1)
}

fn decode_nested_component_changes_payload<R: Read>(
    reader: &mut R,
    depth: usize,
) -> io::Result<()> {
    let added_count = VarIntWrapper::decode(reader)?.0;
    let removed_count = VarIntWrapper::decode(reader)?.0;
    if added_count < 0 || removed_count < 0 || added_count + removed_count > 256 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Nested data component map size out of bounds: {}",
                added_count + removed_count
            ),
        ));
    }
    for _ in 0..added_count {
        let component_id = VarIntWrapper::decode(reader)?.0;
        let component_descriptor =
            DataComponentDescriptor::from_id(component_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown nested data component id: {component_id}"),
                )
            })?;
        if !component_descriptor.is_synced() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Nested data component has no network codec: {component_id}"),
            ));
        }
        decode_nested_component_payload(component_id, reader, depth)?;
    }
    for _ in 0..removed_count {
        let component_id = VarIntWrapper::decode(reader)?.0;
        DataComponentDescriptor::from_id(component_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown nested removed data component id: {component_id}"),
            )
        })?;
    }
    Ok(())
}

fn decode_nested_component_payload<R: Read>(
    component_id: i32,
    reader: &mut R,
    depth: usize,
) -> io::Result<()> {
    if component_id == USE_REMAINDER.id() {
        return decode_item_stack_fields_payload(reader, depth + 1);
    }
    if component_id == CHARGED_PROJECTILES.id() || component_id == BUNDLE_CONTENTS.id() {
        return decode_list_payload(reader, i16::MAX as i32, |entry_reader| {
            decode_item_stack_fields_payload(entry_reader, depth + 1)
        });
    }
    if component_id == spinel_registry::data_components::vanilla_components::CONTAINER.id() {
        return decode_list_payload(reader, 256, |entry_reader| {
            decode_item_stack_fields_payload(entry_reader, depth + 1)
        });
    }
    match component_id {
        id if id == MAX_STACK_SIZE.id()
            || id == MAX_DAMAGE.id()
            || id == DAMAGE.id()
            || id == REPAIR_COST.id()
            || id == ENCHANTABLE.id()
            || id == MAP_ID.id()
            || id == OMINOUS_BOTTLE_AMPLIFIER.id()
            || id == RARITY.id()
            || id == MAP_POST_PROCESSING.id()
            || id == BASE_COLOR.id()
            || id == WOLF_COLLAR.id()
            || id == TROPICAL_FISH_BASE_COLOR.id()
            || id == TROPICAL_FISH_PATTERN_COLOR.id()
            || id == CAT_COLLAR.id()
            || id == SHEEP_COLOR.id()
            || id == SHULKER_COLOR.id()
            || id == FOX_VARIANT.id()
            || id == SALMON_SIZE.id()
            || id == PARROT_VARIANT.id()
            || id == TROPICAL_FISH_PATTERN.id()
            || id == MOOSHROOM_VARIANT.id()
            || id == RABBIT_VARIANT.id()
            || id == HORSE_VARIANT.id()
            || id == LLAMA_VARIANT.id()
            || id == VILLAGER_VARIANT.id()
            || id == AXOLOTL_VARIANT.id()
            || id == DAMAGE_TYPE.id()
            || id == WOLF_VARIANT.id()
            || id == WOLF_SOUND_VARIANT.id()
            || id == PIG_VARIANT.id()
            || id == COW_VARIANT.id()
            || id == CHICKEN_VARIANT.id()
            || id == ZOMBIE_NAUTILUS_VARIANT.id()
            || id == FROG_VARIANT.id()
            || id == CAT_VARIANT.id()
            || id == PAINTING_VARIANT.id() =>
        {
            VarIntWrapper::decode(reader)?;
            Ok(())
        }
        id if id == MINIMUM_ATTACK_CHARGE.id() || id == POTION_DURATION_SCALE.id() => {
            f32::decode(reader)?;
            Ok(())
        }
        id if id == DYED_COLOR.id() || id == MAP_COLOR.id() => {
            i32::decode(reader)?;
            Ok(())
        }
        id if id == ENCHANTMENT_GLINT_OVERRIDE.id() => bool::decode(reader).map(|_| ()),
        id if id == CUSTOM_DATA.id()
            || id == BUCKET_ENTITY_DATA.id()
            || id == DEBUG_STICK_STATE.id() =>
        {
            spinel_nbt::NbtCompound::decode(reader)?;
            Ok(())
        }
        id if id == CUSTOM_NAME.id() || id == ITEM_NAME.id() => {
            NbtTextComponent::decode(reader)?;
            Ok(())
        }
        id if id == ITEM_MODEL.id()
            || id == TOOLTIP_STYLE.id()
            || id == NOTE_BLOCK_SOUND.id()
            || id == PROVIDES_BANNER_PATTERNS.id() =>
        {
            String::decode(reader)?;
            Ok(())
        }
        id if id == BREAK_SOUND.id() => SoundEvent::decode(reader).map(|_| ()),
        id if id == UNBREAKABLE.id() || id == CREATIVE_SLOT_LOCK.id() || id == GLIDER.id() => {
            Ok(())
        }
        nested_component_id => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unsupported nested data component decoder: {nested_component_id}"),
        )),
    }
}

fn decode_component_payload<R: Read>(component_id: i32, reader: &mut R) -> io::Result<Vec<u8>> {
    match component_id {
        id if id == MAX_STACK_SIZE.id()
            || id == MAX_DAMAGE.id()
            || id == DAMAGE.id()
            || id == REPAIR_COST.id()
            || id == ENCHANTABLE.id()
            || id == MAP_ID.id()
            || id == OMINOUS_BOTTLE_AMPLIFIER.id()
            || id == RARITY.id()
            || id == MAP_POST_PROCESSING.id()
            || id == BASE_COLOR.id()
            || id == WOLF_COLLAR.id()
            || id == TROPICAL_FISH_BASE_COLOR.id()
            || id == TROPICAL_FISH_PATTERN_COLOR.id()
            || id == CAT_COLLAR.id()
            || id == SHEEP_COLOR.id()
            || id == SHULKER_COLOR.id()
            || id == FOX_VARIANT.id()
            || id == SALMON_SIZE.id()
            || id == PARROT_VARIANT.id()
            || id == TROPICAL_FISH_PATTERN.id()
            || id == MOOSHROOM_VARIANT.id()
            || id == RABBIT_VARIANT.id()
            || id == HORSE_VARIANT.id()
            || id == LLAMA_VARIANT.id()
            || id == VILLAGER_VARIANT.id()
            || id == AXOLOTL_VARIANT.id() =>
        {
            decode_data_type_payload::<VarIntWrapper, R>(reader)
        }
        id if id == MINIMUM_ATTACK_CHARGE.id() || id == POTION_DURATION_SCALE.id() => {
            decode_data_type_payload::<f32, R>(reader)
        }
        id if id == ENCHANTMENT_GLINT_OVERRIDE.id() => decode_data_type_payload::<bool, R>(reader),
        id if id == CUSTOM_DATA.id() || id == BUCKET_ENTITY_DATA.id() => {
            decode_data_type_payload::<spinel_nbt::NbtCompound, R>(reader)
        }
        id if id == BLOCK_ENTITY_DATA.id() || id == ENTITY_DATA.id() => {
            capture_component_payload(reader, |capturing_reader| {
                VarIntWrapper::decode(capturing_reader)?;
                spinel_nbt::NbtCompound::decode(capturing_reader)?;
                Ok(())
            })
        }
        id if id == CUSTOM_NAME.id() || id == ITEM_NAME.id() => {
            decode_data_type_payload::<NbtTextComponent, R>(reader)
        }
        id if id == LORE.id() => decode_text_component_list_payload(reader, 256),
        id if id == ITEM_MODEL.id() || id == TOOLTIP_STYLE.id() || id == NOTE_BLOCK_SOUND.id() => {
            decode_data_type_payload::<String, R>(reader)
        }
        id if id == BREAK_SOUND.id() => decode_data_type_payload::<SoundEvent, R>(reader),
        id if id == RECIPES.id() => decode_string_list_payload(reader, i16::MAX as i32),
        id if id == FOOD.id() => decode_food_payload(reader),
        id if id == CONSUMABLE.id() => decode_consumable_payload(reader),
        id if id == USE_REMAINDER.id() => decode_item_stack_payload(reader),
        id if id == USE_EFFECTS.id() => decode_use_effects_payload(reader),
        id if id == USE_COOLDOWN.id() => decode_use_cooldown_payload(reader),
        id if id == POTION_CONTENTS.id() => decode_potion_contents_payload(reader),
        id if id == SUSPICIOUS_STEW_EFFECTS.id() => decode_suspicious_stew_effects_payload(reader),
        id if id == BEES.id() => decode_bees_payload(reader),
        id if id == ATTRIBUTE_MODIFIERS.id() => decode_attribute_modifiers_payload(reader),
        id if id == ENCHANTMENTS.id() || id == STORED_ENCHANTMENTS.id() => {
            decode_var_int_map_payload(reader, i16::MAX as i32)
        }
        id if id == DAMAGE_TYPE.id()
            || id == WOLF_VARIANT.id()
            || id == WOLF_SOUND_VARIANT.id()
            || id == PIG_VARIANT.id()
            || id == COW_VARIANT.id()
            || id == CHICKEN_VARIANT.id()
            || id == ZOMBIE_NAUTILUS_VARIANT.id()
            || id == FROG_VARIANT.id()
            || id == CAT_VARIANT.id() =>
        {
            decode_data_type_payload::<VarIntWrapper, R>(reader)
        }
        id if id == PAINTING_VARIANT.id() => decode_registry_holder_payload(reader),
        id if id == TRIM.id() => decode_two_registry_holders_payload(reader),
        id if id == JUKEBOX_PLAYABLE.id() => decode_jukebox_playable_payload(reader),
        id if id == PROVIDES_TRIM_MATERIAL.id() => decode_provides_trim_material_payload(reader),
        id if id == INSTRUMENT.id() => decode_either_registry_key_payload(reader),
        id if id == PROVIDES_BANNER_PATTERNS.id() => decode_tag_key_payload(reader),
        id if id == BANNER_PATTERNS.id() => decode_banner_patterns_payload(reader),
        id if id == DAMAGE_RESISTANT.id() || id == REPAIRABLE.id() => {
            capture_component_payload(reader, |capturing_reader| {
                decode_registry_tag_reference_payload(capturing_reader)
            })
        }
        id if id == TOOL.id() => decode_tool_payload(reader),
        id if id == CAN_PLACE_ON.id() || id == CAN_BREAK.id() => {
            decode_block_predicates_payload(reader)
        }
        id if id == EQUIPPABLE.id() => decode_equippable_payload(reader),
        id if id == WEAPON.id() => capture_component_payload(reader, |capturing_reader| {
            VarIntWrapper::decode(capturing_reader)?;
            f32::decode(capturing_reader)?;
            Ok(())
        }),
        id if id == DEATH_PROTECTION.id() => {
            capture_component_payload(reader, |capturing_reader| {
                decode_consume_effects_payload(capturing_reader, 256)
            })
        }
        id if id == BLOCKS_ATTACKS.id() => decode_blocks_attacks_payload(reader),
        id if id == PIERCING_WEAPON.id() => decode_piercing_weapon_payload(reader),
        id if id == KINETIC_WEAPON.id() => decode_kinetic_weapon_payload(reader),
        id if id == ATTACK_RANGE.id() => decode_attack_range_payload(reader),
        id if id == CUSTOM_MODEL_DATA.id() => decode_custom_model_data_payload(reader),
        id if id == TOOLTIP_DISPLAY.id() => decode_tooltip_display_payload(reader),
        id if id == SWING_ANIMATION.id() => capture_component_payload(reader, |capturing_reader| {
            VarIntWrapper::decode(capturing_reader)?;
            VarIntWrapper::decode(capturing_reader)?;
            Ok(())
        }),
        id if id == FIREWORK_EXPLOSION.id() => decode_firework_explosion_payload(reader),
        id if id == FIREWORKS.id() => decode_firework_list_payload(reader),
        id if id == CHARGED_PROJECTILES.id() || id == BUNDLE_CONTENTS.id() => {
            decode_item_stack_list_component_payload(reader, i16::MAX as i32)
        }
        id if id == spinel_registry::data_components::vanilla_components::CONTAINER.id() => {
            decode_item_stack_list_component_payload(reader, 256)
        }
        id if id == LODESTONE_TRACKER.id() => decode_lodestone_tracker_payload(reader),
        id if id == POT_DECORATIONS.id() => decode_pot_decorations_payload(reader),
        id if id == BLOCK_STATE.id() => decode_item_block_state_payload(reader),
        id if id == DEBUG_STICK_STATE.id() => {
            decode_data_type_payload::<spinel_nbt::NbtCompound, R>(reader)
        }
        id if id == PROFILE.id() => decode_profile_payload(reader),
        id if id == DYED_COLOR.id() || id == MAP_COLOR.id() => {
            decode_data_type_payload::<i32, R>(reader)
        }
        id if id == WRITABLE_BOOK_CONTENT.id() => decode_filtered_string_list_payload(reader, 100),
        id if id == WRITTEN_BOOK_CONTENT.id() => decode_written_book_content_payload(reader),
        id if id == UNBREAKABLE.id() || id == CREATIVE_SLOT_LOCK.id() || id == GLIDER.id() => {
            Ok(Vec::new())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unsupported data component network decoder: {component_id}"),
        )),
    }
}

impl From<&DataComponentMap> for ComponentChanges {
    fn from(component_patch: &DataComponentMap) -> Self {
        Self {
            added: component_patch
                .entries()
                .into_iter()
                .filter_map(|component| {
                    let component_descriptor =
                        DataComponentDescriptor::from_id(component.component_id)?;
                    if !component_descriptor.is_synced() {
                        return None;
                    }
                    encode_component_nbt(component.component_id, &component.component_nbt).map(
                        |data| ComponentEntry {
                            type_id: component.component_id,
                            data,
                        },
                    )
                })
                .collect(),
            removed: component_patch.removed_component_ids(),
        }
    }
}

fn encode_component_nbt(component_id: i32, component_nbt: &Nbt) -> Option<Vec<u8>> {
    let mut data = Vec::new();
    match component_id {
        id if id == MAX_STACK_SIZE.id()
            || id == MAX_DAMAGE.id()
            || id == DAMAGE.id()
            || id == REPAIR_COST.id()
            || id == ENCHANTABLE.id()
            || id == MAP_ID.id()
            || id == OMINOUS_BOTTLE_AMPLIFIER.id() =>
        {
            encode_var_int_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == MINIMUM_ATTACK_CHARGE.id() || id == POTION_DURATION_SCALE.id() => {
            encode_float_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == ENCHANTMENT_GLINT_OVERRIDE.id() => {
            encode_bool_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == RARITY.id() => {
            encode_item_rarity_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CUSTOM_DATA.id() || id == BUCKET_ENTITY_DATA.id() => {
            encode_nbt_compound_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BLOCK_ENTITY_DATA.id() => {
            encode_block_entity_data_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == ENTITY_DATA.id() => {
            encode_entity_data_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CUSTOM_NAME.id() || id == ITEM_NAME.id() => {
            encode_text_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == LORE.id() => {
            encode_text_component_list(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == UNBREAKABLE.id() || id == CREATIVE_SLOT_LOCK.id() || id == GLIDER.id() => {
            return Some(data);
        }
        id if id == ITEM_MODEL.id() || id == TOOLTIP_STYLE.id() || id == NOTE_BLOCK_SOUND.id() => {
            encode_string_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BREAK_SOUND.id() => {
            encode_sound_event_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == RECIPES.id() => {
            encode_string_list_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == FOOD.id() => {
            encode_food_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CONSUMABLE.id() => {
            encode_consumable_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == USE_REMAINDER.id() => {
            encode_item_stack_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == USE_EFFECTS.id() => {
            encode_use_effects_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == USE_COOLDOWN.id() => {
            encode_use_cooldown_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == POTION_CONTENTS.id() => {
            encode_potion_contents_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == SUSPICIOUS_STEW_EFFECTS.id() => {
            encode_suspicious_stew_effects_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BEES.id() => {
            encode_bees_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == ATTRIBUTE_MODIFIERS.id() => {
            encode_attribute_modifiers_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == ENCHANTMENTS.id() || id == STORED_ENCHANTMENTS.id() => {
            encode_enchantment_list_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == DAMAGE_TYPE.id() => {
            encode_dynamic_registry_key_component(component_nbt, &mut data, &DAMAGE_TYPE_REGISTRY)?;
            return Some(data);
        }
        id if id == WOLF_VARIANT.id() => {
            encode_dynamic_registry_key_component(
                component_nbt,
                &mut data,
                &WOLF_VARIANT_REGISTRY,
            )?;
            return Some(data);
        }
        id if id == WOLF_SOUND_VARIANT.id() => {
            encode_dynamic_registry_key_component(
                component_nbt,
                &mut data,
                &WOLF_SOUND_VARIANT_REGISTRY,
            )?;
            return Some(data);
        }
        id if id == PIG_VARIANT.id() => {
            encode_dynamic_registry_key_component(component_nbt, &mut data, &PIG_VARIANT_REGISTRY)?;
            return Some(data);
        }
        id if id == COW_VARIANT.id() => {
            encode_dynamic_registry_key_component(component_nbt, &mut data, &COW_VARIANT_REGISTRY)?;
            return Some(data);
        }
        id if id == CHICKEN_VARIANT.id() => {
            encode_dynamic_registry_key_component(
                component_nbt,
                &mut data,
                &CHICKEN_VARIANT_REGISTRY,
            )?;
            return Some(data);
        }
        id if id == ZOMBIE_NAUTILUS_VARIANT.id() => {
            encode_dynamic_registry_key_component(
                component_nbt,
                &mut data,
                &ZOMBIE_NAUTILUS_VARIANT_REGISTRY,
            )?;
            return Some(data);
        }
        id if id == FROG_VARIANT.id() => {
            encode_dynamic_registry_key_component(
                component_nbt,
                &mut data,
                &FROG_VARIANT_REGISTRY,
            )?;
            return Some(data);
        }
        id if id == PAINTING_VARIANT.id() => {
            encode_dynamic_registry_holder_key_component(
                component_nbt,
                &mut data,
                &PAINTING_VARIANT_REGISTRY,
            )?;
            return Some(data);
        }
        id if id == TRIM.id() => {
            encode_armor_trim_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == INSTRUMENT.id() => {
            encode_instrument_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == JUKEBOX_PLAYABLE.id() => {
            encode_jukebox_playable_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == PROVIDES_TRIM_MATERIAL.id() => {
            encode_provides_trim_material_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == PROVIDES_BANNER_PATTERNS.id() => {
            encode_tag_key_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BANNER_PATTERNS.id() => {
            encode_banner_patterns_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CAT_VARIANT.id() => {
            encode_dynamic_registry_key_component(component_nbt, &mut data, &CAT_VARIANT_REGISTRY)?;
            return Some(data);
        }
        id if id == DAMAGE_RESISTANT.id() => {
            encode_damage_resistant_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == TOOL.id() => {
            encode_tool_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CAN_PLACE_ON.id() || id == CAN_BREAK.id() => {
            encode_block_predicates_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == EQUIPPABLE.id() => {
            encode_equippable_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == REPAIRABLE.id() => {
            encode_material_registry_tag_reference_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == WEAPON.id() => {
            encode_weapon_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == DEATH_PROTECTION.id() => {
            encode_death_protection_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BLOCKS_ATTACKS.id() => {
            encode_blocks_attacks_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == PIERCING_WEAPON.id() => {
            encode_piercing_weapon_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == KINETIC_WEAPON.id() => {
            encode_kinetic_weapon_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == ATTACK_RANGE.id() => {
            encode_attack_range_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CUSTOM_MODEL_DATA.id() => {
            encode_custom_model_data_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == TOOLTIP_DISPLAY.id() => {
            encode_tooltip_display_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == SWING_ANIMATION.id() => {
            encode_swing_animation_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == MAP_POST_PROCESSING.id() => {
            encode_map_post_processing_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == FIREWORK_EXPLOSION.id() => {
            encode_firework_explosion_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == FIREWORKS.id() => {
            encode_firework_list_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == PROFILE.id() => {
            encode_profile_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == CHARGED_PROJECTILES.id()
            || id == BUNDLE_CONTENTS.id()
            || id == spinel_registry::data_components::vanilla_components::CONTAINER.id() =>
        {
            encode_item_stack_list_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == LODESTONE_TRACKER.id() => {
            encode_lodestone_tracker_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == POT_DECORATIONS.id() => {
            encode_pot_decorations_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BLOCK_STATE.id() => {
            encode_item_block_state_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == DEBUG_STICK_STATE.id() => {
            encode_debug_stick_state_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == DYED_COLOR.id() || id == MAP_COLOR.id() => {
            encode_color_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == BASE_COLOR.id()
            || id == WOLF_COLLAR.id()
            || id == TROPICAL_FISH_BASE_COLOR.id()
            || id == TROPICAL_FISH_PATTERN_COLOR.id()
            || id == CAT_COLLAR.id()
            || id == SHEEP_COLOR.id()
            || id == SHULKER_COLOR.id() =>
        {
            encode_dye_color_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == FOX_VARIANT.id() => {
            encode_named_enum_component(component_nbt, &mut data, &["red", "snow"])?;
            return Some(data);
        }
        id if id == SALMON_SIZE.id() => {
            encode_named_enum_component(component_nbt, &mut data, &["small", "medium", "large"])?;
            return Some(data);
        }
        id if id == PARROT_VARIANT.id() => {
            encode_named_enum_component(
                component_nbt,
                &mut data,
                &["red_blue", "blue", "green", "yellow_blue", "grey"],
            )?;
            return Some(data);
        }
        id if id == TROPICAL_FISH_PATTERN.id() => {
            encode_tropical_fish_pattern_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == MOOSHROOM_VARIANT.id() => {
            encode_named_enum_component(component_nbt, &mut data, &["red", "brown"])?;
            return Some(data);
        }
        id if id == RABBIT_VARIANT.id() => {
            encode_named_enum_component(
                component_nbt,
                &mut data,
                &[
                    "brown",
                    "white",
                    "black",
                    "black_and_white",
                    "gold",
                    "salt_and_pepper",
                    "killer_bunny",
                ],
            )?;
            return Some(data);
        }
        id if id == HORSE_VARIANT.id() => {
            encode_named_enum_component(
                component_nbt,
                &mut data,
                &[
                    "white",
                    "creamy",
                    "chestnut",
                    "brown",
                    "black",
                    "gray",
                    "dark_brown",
                ],
            )?;
            return Some(data);
        }
        id if id == LLAMA_VARIANT.id() => {
            encode_named_enum_component(
                component_nbt,
                &mut data,
                &["creamy", "white", "brown", "gray"],
            )?;
            return Some(data);
        }
        id if id == VILLAGER_VARIANT.id() => {
            encode_named_enum_component(
                component_nbt,
                &mut data,
                &[
                    "desert", "jungle", "plains", "savanna", "snow", "swamp", "taiga",
                ],
            )?;
            return Some(data);
        }
        id if id == AXOLOTL_VARIANT.id() => {
            encode_named_enum_component(
                component_nbt,
                &mut data,
                &["lucy", "wild", "gold", "cyan", "blue"],
            )?;
            return Some(data);
        }
        id if id == WRITABLE_BOOK_CONTENT.id() => {
            encode_writable_book_content_component(component_nbt, &mut data)?;
            return Some(data);
        }
        id if id == WRITTEN_BOOK_CONTENT.id() => {
            encode_written_book_content_component(component_nbt, &mut data)?;
            return Some(data);
        }
        _ => {}
    }
    match component_nbt {
        Nbt::Int(value) => VarIntWrapper(*value).encode(&mut data).ok()?,
        Nbt::Byte(value) => data.write_all(&value.to_be_bytes()).ok()?,
        Nbt::List(_) => return None,
        Nbt::Compound(_) => return None,
        _ => return None,
    }
    Some(data)
}

fn encode_nbt_compound_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::Compound(compound) = component_nbt else {
        return None;
    };
    compound.encode(data).ok()
}

fn encode_block_entity_data_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let block_entity_data = TypedCustomData::from_component_nbt(component_nbt)?;
    VarIntWrapper(block_entity_type_protocol_id(block_entity_data.type_id())?)
        .encode(data)
        .ok()?;
    block_entity_data.nbt().encode(data).ok()
}

fn encode_entity_data_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let entity_data = TypedCustomData::from_component_nbt(component_nbt)?;
    VarIntWrapper(entity_type_protocol_id(entity_data.type_id())?)
        .encode(data)
        .ok()?;
    entity_data.nbt().encode(data).ok()
}

fn entity_type_protocol_id(entity_type: &spinel_registry::Identifier) -> Option<i32> {
    EntityType::from_key(&entity_type.to_string()).map(EntityType::id)
}

fn block_entity_type_protocol_id(block_entity_type: &spinel_registry::Identifier) -> Option<i32> {
    BlockEntityType::ALL
        .iter()
        .find(|registered_block_entity_type| {
            registered_block_entity_type.key() == block_entity_type.path.as_ref()
        })
        .map(|registered_block_entity_type| registered_block_entity_type.id())
}

fn encode_string_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::String(value) = component_nbt else {
        return None;
    };
    value.clone().encode(data).ok()
}

fn encode_var_int_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::Int(value) = component_nbt else {
        return None;
    };
    VarIntWrapper(*value).encode(data).ok()
}

fn encode_float_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::Float(value) = component_nbt else {
        return None;
    };
    value.encode(data).ok()
}

fn encode_bool_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::Byte(value) = component_nbt else {
        return None;
    };
    (*value != 0).encode(data).ok()
}

fn encode_sound_event_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::String(sound_name) = component_nbt else {
        return None;
    };
    encode_sound_event_name(sound_name, data)
}

fn encode_string_list_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::List(values) = component_nbt else {
        return None;
    };
    VarIntWrapper(values.len() as i32).encode(data).ok()?;
    for value in values {
        let Nbt::String(value) = value else {
            return None;
        };
        value.clone().encode(data).ok()?;
    }
    Some(())
}

fn encode_text_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let component = TextComponent::from_component_nbt(component_nbt)?;
    NbtTextComponent(component).encode(data).ok()
}

fn encode_text_component_list(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let components = Vec::<TextComponent>::from_component_nbt(component_nbt)?;
    VarIntWrapper(components.len() as i32).encode(data).ok()?;
    for component in components {
        NbtTextComponent(component).encode(data).ok()?;
    }
    Some(())
}

fn encode_item_stack_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let item_stack = ItemStack::from_component_nbt(component_nbt)?;
    Slot::from_item_stack(&item_stack).encode(data).ok()
}

fn encode_item_stack_list_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let item_stacks = Vec::<ItemStack>::from_component_nbt(component_nbt)?;
    VarIntWrapper(item_stacks.len() as i32).encode(data).ok()?;
    for item_stack in item_stacks {
        Slot::from_item_stack(&item_stack).encode(data).ok()?;
    }
    Some(())
}

fn encode_color_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let color = Color::from_component_nbt(component_nbt)?;
    (color.as_rgb() as i32).encode(data).ok()
}

fn encode_dye_color_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let dye_color = DyeColor::from_component_nbt(component_nbt)?;
    VarIntWrapper(dye_color_protocol_id(dye_color))
        .encode(data)
        .ok()
}

fn encode_named_enum_component(
    component_nbt: &Nbt,
    data: &mut Vec<u8>,
    names: &[&str],
) -> Option<()> {
    let Nbt::String(value) = component_nbt else {
        return None;
    };
    let protocol_name = value
        .parse::<spinel_registry::Identifier>()
        .ok()
        .map(|identifier| identifier.path.to_string())
        .unwrap_or_else(|| value.clone());
    let protocol_id = names.iter().position(|name| name == &protocol_name)?;
    VarIntWrapper(protocol_id as i32).encode(data).ok()
}

fn encode_tropical_fish_pattern_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::String(value) = component_nbt else {
        return None;
    };
    let protocol_id = match value.as_str() {
        "kob" => 0,
        "sunstreak" => 256,
        "snooper" => 512,
        "dasher" => 768,
        "brinely" => 1024,
        "spotty" => 1280,
        "flopper" => 1,
        "stripey" => 257,
        "glitter" => 513,
        "blockfish" => 769,
        "betty" => 1025,
        "clayfish" => 1281,
        _ => return None,
    };
    VarIntWrapper(protocol_id).encode(data).ok()
}

fn encode_item_rarity_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let item_rarity = ItemRarity::from_component_nbt(component_nbt)?;
    VarIntWrapper(item_rarity.protocol_id()).encode(data).ok()
}

fn encode_food_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let food = Food::from_component_nbt(component_nbt)?;
    VarIntWrapper(food.nutrition()).encode(data).ok()?;
    food.saturation_modifier().encode(data).ok()?;
    food.can_always_eat().encode(data).ok()
}

fn encode_consumable_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let consumable = Consumable::from_component_nbt(component_nbt)?;
    consumable.consume_seconds().encode(data).ok()?;
    VarIntWrapper(consumable.animation().protocol_id())
        .encode(data)
        .ok()?;
    encode_sound_event_name(&consumable.sound().to_string(), data)?;
    consumable.has_consume_particles().encode(data).ok()?;
    encode_consume_effects(consumable.effects(), data)
}

fn encode_death_protection_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let death_protection = DeathProtection::from_component_nbt(component_nbt)?;
    encode_consume_effects(death_protection.death_effects(), data)
}

fn encode_consume_effects(
    consume_effects: &[spinel_registry::ConsumeEffect],
    data: &mut Vec<u8>,
) -> Option<()> {
    VarIntWrapper(consume_effects.len() as i32)
        .encode(data)
        .ok()?;
    for consume_effect in consume_effects {
        encode_consume_effect(consume_effect, data)?;
    }
    Some(())
}

fn encode_consume_effect(
    consume_effect: &spinel_registry::ConsumeEffect,
    data: &mut Vec<u8>,
) -> Option<()> {
    match consume_effect {
        spinel_registry::ConsumeEffect::ApplyEffects {
            effects,
            probability,
        } => {
            VarIntWrapper(0).encode(data).ok()?;
            VarIntWrapper(effects.len() as i32).encode(data).ok()?;
            for effect in effects {
                encode_custom_potion_effect(effect, data)?;
            }
            probability.encode(data).ok()
        }
        spinel_registry::ConsumeEffect::RemoveEffects { effects } => {
            VarIntWrapper(1).encode(data).ok()?;
            encode_potion_effect_registry_tag_reference(effects, data)
        }
        spinel_registry::ConsumeEffect::ClearAllEffects => VarIntWrapper(2).encode(data).ok(),
        spinel_registry::ConsumeEffect::TeleportRandomly { diameter } => {
            VarIntWrapper(3).encode(data).ok()?;
            diameter.encode(data).ok()
        }
        spinel_registry::ConsumeEffect::PlaySound { sound } => {
            VarIntWrapper(4).encode(data).ok()?;
            encode_sound_event_name(&sound.to_string(), data)
        }
    }
}

fn encode_potion_effect_registry_tag_reference(
    registry_tag_reference: &RegistryTagReference,
    data: &mut Vec<u8>,
) -> Option<()> {
    match registry_tag_reference {
        RegistryTagReference::Backed(tag_key) => {
            VarIntWrapper(0).encode(data).ok()?;
            tag_key.encode(data).ok()
        }
        RegistryTagReference::Empty => VarIntWrapper(1).encode(data).ok(),
        RegistryTagReference::Direct(effect_keys) => {
            VarIntWrapper(effect_keys.len() as i32 + 1)
                .encode(data)
                .ok()?;
            for effect_key in effect_keys {
                VarIntWrapper(potion_effect_protocol_id(effect_key)?)
                    .encode(data)
                    .ok()?;
            }
            Some(())
        }
    }
}

fn encode_damage_resistant_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let damage_resistant = DamageResistant::from_component_nbt(component_nbt)?;
    VarIntWrapper(0).encode(data).ok()?;
    damage_resistant.damage_type_tag().encode(data).ok()
}

fn encode_dynamic_registry_key_component(
    component_nbt: &Nbt,
    data: &mut Vec<u8>,
    registry_name: &spinel_registry::Identifier,
) -> Option<()> {
    let Nbt::String(entry_name) = component_nbt else {
        return None;
    };
    let entry_name = entry_name.parse().ok()?;
    let entry_id = VANILLA_REGISTRIES.dynamic_registry_id(registry_name, &entry_name)?;
    VarIntWrapper(entry_id).encode(data).ok()
}

fn encode_dynamic_registry_holder_key_component(
    component_nbt: &Nbt,
    data: &mut Vec<u8>,
    registry_name: &spinel_registry::Identifier,
) -> Option<()> {
    let Nbt::String(entry_name) = component_nbt else {
        return None;
    };
    let entry_name = entry_name.parse().ok()?;
    let entry_id = VANILLA_REGISTRIES.dynamic_registry_id(registry_name, &entry_name)?;
    VarIntWrapper(entry_id + 1).encode(data).ok()
}

fn encode_dynamic_registry_holder_identifier(
    entry_name: &spinel_registry::Identifier,
    data: &mut Vec<u8>,
    registry_name: &spinel_registry::Identifier,
) -> Option<()> {
    let entry_id = VANILLA_REGISTRIES.dynamic_registry_id(registry_name, entry_name)?;
    VarIntWrapper(entry_id + 1).encode(data).ok()
}

fn encode_armor_trim_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let armor_trim = ArmorTrim::from_component_nbt(component_nbt)?;
    encode_dynamic_registry_holder_identifier(
        armor_trim.material(),
        data,
        &TRIM_MATERIAL_REGISTRY,
    )?;
    encode_dynamic_registry_holder_identifier(armor_trim.pattern(), data, &TRIM_PATTERN_REGISTRY)
}

fn encode_instrument_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let instrument = InstrumentComponent::from_component_nbt(component_nbt)?;
    false.encode(data).ok()?;
    instrument.instrument().to_string().encode(data).ok()
}

fn encode_jukebox_playable_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::String(jukebox_song_name) = component_nbt else {
        return None;
    };
    let jukebox_song_name = jukebox_song_name.parse().ok()?;
    true.encode(data).ok()?;
    encode_dynamic_registry_holder_identifier(&jukebox_song_name, data, &JUKEBOX_SONG_REGISTRY)
}

fn encode_provides_trim_material_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::String(trim_material_name) = component_nbt else {
        return None;
    };
    false.encode(data).ok()?;
    trim_material_name.clone().encode(data).ok()
}

fn encode_tag_key_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let Nbt::String(tag_name) = component_nbt else {
        return None;
    };
    tag_name
        .strip_prefix('#')
        .unwrap_or(tag_name)
        .to_string()
        .encode(data)
        .ok()
}

fn encode_banner_patterns_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let banner_patterns = BannerPatterns::from_component_nbt(component_nbt)?;
    VarIntWrapper(banner_patterns.layers().len() as i32)
        .encode(data)
        .ok()?;
    for layer in banner_patterns.layers() {
        encode_dynamic_registry_holder_identifier(layer.pattern(), data, &BANNER_PATTERN_REGISTRY)?;
        VarIntWrapper(dye_color_protocol_id(layer.color()))
            .encode(data)
            .ok()?;
    }
    Some(())
}

fn encode_tool_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let tool = Tool::from_component_nbt(component_nbt)?;
    VarIntWrapper(tool.rules().len() as i32).encode(data).ok()?;
    for rule in tool.rules() {
        encode_block_registry_tag_reference(rule.blocks(), data)?;
        rule.speed().encode(data).ok()?;
        rule.correct_for_drops().encode(data).ok()?;
    }
    tool.default_mining_speed().encode(data).ok()?;
    VarIntWrapper(tool.damage_per_block()).encode(data).ok()?;
    tool.can_destroy_blocks_in_creative().encode(data).ok()
}

fn encode_block_registry_tag_reference(
    registry_tag_reference: &RegistryTagReference,
    data: &mut Vec<u8>,
) -> Option<()> {
    match registry_tag_reference {
        RegistryTagReference::Backed(tag_key) => {
            VarIntWrapper(0).encode(data).ok()?;
            tag_key.encode(data).ok()
        }
        RegistryTagReference::Empty => VarIntWrapper(1).encode(data).ok(),
        RegistryTagReference::Direct(block_keys) => {
            VarIntWrapper(block_keys.len() as i32 + 1)
                .encode(data)
                .ok()?;
            for block_key in block_keys {
                VarIntWrapper(block_protocol_id(block_key)?)
                    .encode(data)
                    .ok()?;
            }
            Some(())
        }
    }
}

fn block_protocol_id(block_key: &spinel_registry::Identifier) -> Option<i32> {
    Block::ALL
        .iter()
        .position(|block| {
            block_key.namespace.as_ref() == "minecraft" && block.path() == block_key.path.as_ref()
        })
        .map(|block_id| block_id as i32)
}

fn encode_block_predicates_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let block_predicates = BlockPredicates::from_component_nbt(component_nbt)?;
    VarIntWrapper(block_predicates.predicates().len() as i32)
        .encode(data)
        .ok()?;
    for predicate in block_predicates.predicates() {
        encode_optional_block_registry_tag_reference(predicate.blocks(), data)?;
        encode_optional_properties_predicate(predicate.state(), data)?;
        predicate.nbt().cloned().encode(data).ok()?;
        VarIntWrapper(0).encode(data).ok()?;
        VarIntWrapper(0).encode(data).ok()?;
    }
    Some(())
}

fn encode_optional_block_registry_tag_reference(
    registry_tag_reference: Option<&RegistryTagReference>,
    data: &mut Vec<u8>,
) -> Option<()> {
    match registry_tag_reference {
        Some(registry_tag_reference) => {
            true.encode(data).ok()?;
            encode_block_registry_tag_reference(registry_tag_reference, data)
        }
        None => false.encode(data).ok(),
    }
}

fn encode_optional_properties_predicate(
    properties_predicate: Option<&spinel_registry::PropertiesPredicate>,
    data: &mut Vec<u8>,
) -> Option<()> {
    match properties_predicate {
        Some(properties_predicate) => {
            true.encode(data).ok()?;
            encode_properties_predicate(properties_predicate, data)
        }
        None => false.encode(data).ok(),
    }
}

fn encode_properties_predicate(
    properties_predicate: &spinel_registry::PropertiesPredicate,
    data: &mut Vec<u8>,
) -> Option<()> {
    VarIntWrapper(properties_predicate.properties().len() as i32)
        .encode(data)
        .ok()?;
    for (property_name, property_value) in properties_predicate.properties() {
        property_name.clone().encode(data).ok()?;
        encode_property_value_predicate(property_value, data)?;
    }
    Some(())
}

fn encode_property_value_predicate(
    property_value: &PropertyValuePredicate,
    data: &mut Vec<u8>,
) -> Option<()> {
    match property_value {
        PropertyValuePredicate::Exact(value) => {
            true.encode(data).ok()?;
            value.clone().unwrap_or_default().encode(data).ok()
        }
        PropertyValuePredicate::Range { min, max } => {
            false.encode(data).ok()?;
            min.clone().encode(data).ok()?;
            max.clone().encode(data).ok()
        }
    }
}

fn encode_equippable_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let equippable = Equippable::from_component_nbt(component_nbt)?;
    VarIntWrapper(equippable.slot().protocol_id())
        .encode(data)
        .ok()?;
    encode_sound_event_name(&equippable.equip_sound().to_string(), data)?;
    equippable
        .asset_id()
        .map(str::to_string)
        .encode(data)
        .ok()?;
    equippable
        .camera_overlay()
        .map(str::to_string)
        .encode(data)
        .ok()?;
    encode_optional_keyed_registry_tag_reference(equippable.allowed_entities(), data)?;
    equippable.dispensable().encode(data).ok()?;
    equippable.swappable().encode(data).ok()?;
    equippable.damage_on_hurt().encode(data).ok()?;
    equippable.equip_on_interact().encode(data).ok()?;
    equippable.can_be_sheared().encode(data).ok()?;
    encode_sound_event_name(&equippable.shearing_sound().to_string(), data)
}

fn encode_material_registry_tag_reference_component(
    component_nbt: &Nbt,
    data: &mut Vec<u8>,
) -> Option<()> {
    let registry_tag_reference = RegistryTagReference::from_component_nbt(component_nbt)?;
    encode_material_registry_tag_reference(&registry_tag_reference, data)
}

fn encode_material_registry_tag_reference(
    registry_tag_reference: &RegistryTagReference,
    data: &mut Vec<u8>,
) -> Option<()> {
    match registry_tag_reference {
        RegistryTagReference::Backed(tag_key) => {
            VarIntWrapper(0).encode(data).ok()?;
            tag_key.encode(data).ok()
        }
        RegistryTagReference::Empty => VarIntWrapper(1).encode(data).ok(),
        RegistryTagReference::Direct(material_keys) => {
            VarIntWrapper(material_keys.len() as i32 + 1)
                .encode(data)
                .ok()?;
            for material_key in material_keys {
                let material = Material::from_key(&material_key.to_string())?;
                VarIntWrapper(material.id()).encode(data).ok()?;
            }
            Some(())
        }
    }
}

fn encode_optional_keyed_registry_tag_reference(
    registry_tag_reference: Option<&RegistryTagReference>,
    data: &mut Vec<u8>,
) -> Option<()> {
    match registry_tag_reference {
        Some(registry_tag_reference) => {
            true.encode(data).ok()?;
            encode_keyed_registry_tag_reference(registry_tag_reference, data)
        }
        None => false.encode(data).ok(),
    }
}

fn encode_keyed_registry_tag_reference(
    registry_tag_reference: &RegistryTagReference,
    data: &mut Vec<u8>,
) -> Option<()> {
    match registry_tag_reference {
        RegistryTagReference::Backed(tag_key) => {
            VarIntWrapper(0).encode(data).ok()?;
            tag_key.encode(data).ok()
        }
        RegistryTagReference::Empty => VarIntWrapper(1).encode(data).ok(),
        RegistryTagReference::Direct(_) => None,
    }
}

fn encode_blocks_attacks_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let blocks_attacks = BlocksAttacks::from_component_nbt(component_nbt)?;
    blocks_attacks.block_delay_seconds().encode(data).ok()?;
    blocks_attacks.disable_cooldown_scale().encode(data).ok()?;
    VarIntWrapper(blocks_attacks.damage_reductions().len() as i32)
        .encode(data)
        .ok()?;
    for damage_reduction in blocks_attacks.damage_reductions() {
        damage_reduction
            .horizontal_blocking_angle()
            .encode(data)
            .ok()?;
        Option::<i32>::None.encode(data).ok()?;
        damage_reduction.base().encode(data).ok()?;
        damage_reduction.factor().encode(data).ok()?;
    }
    let item_damage = blocks_attacks.item_damage();
    item_damage.threshold().encode(data).ok()?;
    item_damage.base().encode(data).ok()?;
    item_damage.factor().encode(data).ok()?;
    Option::<String>::None.encode(data).ok()?;
    encode_optional_named_sound_event(blocks_attacks.block_sound(), data)?;
    encode_optional_named_sound_event(blocks_attacks.disable_sound(), data)
}

fn encode_piercing_weapon_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let piercing_weapon = PiercingWeapon::from_component_nbt(component_nbt)?;
    piercing_weapon.deals_knockback().encode(data).ok()?;
    piercing_weapon.dismounts().encode(data).ok()?;
    encode_optional_named_sound_event(piercing_weapon.sound(), data)?;
    encode_optional_named_sound_event(piercing_weapon.hit_sound(), data)
}

fn encode_kinetic_weapon_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let kinetic_weapon = KineticWeapon::from_component_nbt(component_nbt)?;
    VarIntWrapper(kinetic_weapon.contact_cooldown_ticks())
        .encode(data)
        .ok()?;
    VarIntWrapper(kinetic_weapon.delay_ticks())
        .encode(data)
        .ok()?;
    encode_optional_kinetic_weapon_condition(kinetic_weapon.dismount_conditions(), data)?;
    encode_optional_kinetic_weapon_condition(kinetic_weapon.knockback_conditions(), data)?;
    encode_optional_kinetic_weapon_condition(kinetic_weapon.damage_conditions(), data)?;
    kinetic_weapon.forward_movement().encode(data).ok()?;
    kinetic_weapon.damage_multiplier().encode(data).ok()?;
    encode_optional_named_sound_event(kinetic_weapon.sound(), data)?;
    encode_optional_named_sound_event(kinetic_weapon.hit_sound(), data)
}

fn encode_optional_kinetic_weapon_condition(
    condition: Option<KineticWeaponCondition>,
    data: &mut Vec<u8>,
) -> Option<()> {
    match condition {
        Some(condition) => {
            true.encode(data).ok()?;
            VarIntWrapper(condition.max_duration_ticks())
                .encode(data)
                .ok()?;
            condition.min_speed().encode(data).ok()?;
            condition.min_relative_speed().encode(data).ok()
        }
        None => false.encode(data).ok(),
    }
}

fn encode_lodestone_tracker_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let lodestone_tracker = LodestoneTracker::from_component_nbt(component_nbt)?;
    encode_optional_world_position(lodestone_tracker.target(), data)?;
    lodestone_tracker.tracked().encode(data).ok()
}

fn encode_optional_world_position(
    world_position: Option<&WorldPosition>,
    data: &mut Vec<u8>,
) -> Option<()> {
    match world_position {
        Some(world_position) => {
            true.encode(data).ok()?;
            world_position.dimension().to_string().encode(data).ok()?;
            Position {
                x: world_position.x(),
                y: world_position.y(),
                z: world_position.z(),
            }
            .encode(data)
            .ok()
        }
        None => false.encode(data).ok(),
    }
}

fn encode_sound_event_name(sound_name: &str, data: &mut Vec<u8>) -> Option<()> {
    let sound_identifier = sound_name.parse().ok()?;
    match BuiltinSoundEvent::from_key(&sound_identifier) {
        Some(builtin_sound_event) => SoundEvent::Id(builtin_sound_event.id()).encode(data).ok(),
        None => SoundEvent::Named {
            name: sound_identifier.to_string(),
            fixed_range: None,
        }
        .encode(data)
        .ok(),
    }
}

fn encode_optional_named_sound_event(
    sound_name: Option<&spinel_registry::Identifier>,
    data: &mut Vec<u8>,
) -> Option<()> {
    match sound_name {
        Some(sound_name) => Some(encode_sound_event_name(&sound_name.to_string(), data)?),
        None => Option::<SoundEvent>::None.encode(data).ok(),
    }
}

fn encode_use_effects_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let use_effects = UseEffects::from_component_nbt(component_nbt)?;
    use_effects.can_sprint().encode(data).ok()?;
    use_effects.interact_vibrations().encode(data).ok()?;
    use_effects.speed_multiplier().encode(data).ok()
}

fn encode_use_cooldown_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let use_cooldown = UseCooldown::from_component_nbt(component_nbt)?;
    use_cooldown.seconds().encode(data).ok()?;
    use_cooldown
        .cooldown_group()
        .map(str::to_string)
        .encode(data)
        .ok()
}

fn encode_potion_contents_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let potion_contents = PotionContents::from_component_nbt(component_nbt)?;
    match potion_contents.potion() {
        Some(potion) => {
            true.encode(data).ok()?;
            VarIntWrapper(potion_type_protocol_id(potion)?)
                .encode(data)
                .ok()?;
        }
        None => false.encode(data).ok()?,
    }
    match potion_contents.custom_color() {
        Some(custom_color) => {
            true.encode(data).ok()?;
            (custom_color.as_rgb() as i32).encode(data).ok()?;
        }
        None => false.encode(data).ok()?,
    }
    VarIntWrapper(potion_contents.custom_effects().len() as i32)
        .encode(data)
        .ok()?;
    for custom_effect in potion_contents.custom_effects() {
        encode_custom_potion_effect(custom_effect, data)?;
    }
    potion_contents
        .custom_name()
        .map(str::to_string)
        .encode(data)
        .ok()
}

fn encode_suspicious_stew_effects_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let suspicious_stew_effects = SuspiciousStewEffects::from_component_nbt(component_nbt)?;
    VarIntWrapper(suspicious_stew_effects.effects().len() as i32)
        .encode(data)
        .ok()?;
    for effect in suspicious_stew_effects.effects() {
        VarIntWrapper(potion_effect_protocol_id(effect.effect_id())?)
            .encode(data)
            .ok()?;
        VarIntWrapper(effect.duration_ticks()).encode(data).ok()?;
    }
    Some(())
}

fn encode_bees_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let bees = Vec::<Bee>::from_component_nbt(component_nbt)?;
    VarIntWrapper(bees.len() as i32).encode(data).ok()?;
    for bee in bees {
        bee.entity_data().encode(data).ok()?;
        VarIntWrapper(bee.ticks_in_hive()).encode(data).ok()?;
        VarIntWrapper(bee.min_ticks_in_hive()).encode(data).ok()?;
    }
    Some(())
}

fn encode_attribute_modifiers_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let attribute_list = AttributeList::from_component_nbt(component_nbt)?;
    VarIntWrapper(attribute_list.modifiers().len() as i32)
        .encode(data)
        .ok()?;
    for modifier in attribute_list.modifiers() {
        VarIntWrapper(attribute_protocol_id(modifier.attribute_type())?)
            .encode(data)
            .ok()?;
        modifier.id().to_string().encode(data).ok()?;
        modifier.amount().encode(data).ok()?;
        VarIntWrapper(modifier.operation().protocol_id())
            .encode(data)
            .ok()?;
        VarIntWrapper(modifier.slot().protocol_id())
            .encode(data)
            .ok()?;
        encode_attribute_modifier_display(modifier.display(), data)?;
    }
    Some(())
}

fn encode_attribute_modifier_display(
    display: &AttributeModifierDisplay,
    data: &mut Vec<u8>,
) -> Option<()> {
    match display {
        AttributeModifierDisplay::Default => VarIntWrapper(0).encode(data).ok(),
        AttributeModifierDisplay::Hidden => VarIntWrapper(1).encode(data).ok(),
        AttributeModifierDisplay::Override(component_nbt) => {
            VarIntWrapper(2).encode(data).ok()?;
            encode_text_component(component_nbt, data)
        }
    }
}

fn attribute_protocol_id(attribute: &spinel_registry::Identifier) -> Option<i32> {
    spinel_registry::Attribute::from_identifier(attribute)
        .map(spinel_registry::Attribute::protocol_id)
}

fn encode_enchantment_list_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let enchantment_list = EnchantmentList::from_component_nbt(component_nbt)?;
    VarIntWrapper(enchantment_list.enchantments().len() as i32)
        .encode(data)
        .ok()?;
    for (enchantment, level) in enchantment_list.enchantments() {
        let enchantment_id =
            VANILLA_REGISTRIES.dynamic_registry_id(&ENCHANTMENT_REGISTRY, enchantment)?;
        VarIntWrapper(enchantment_id).encode(data).ok()?;
        VarIntWrapper(*level).encode(data).ok()?;
    }
    Some(())
}

fn encode_custom_potion_effect(
    custom_effect: &CustomPotionEffect,
    data: &mut Vec<u8>,
) -> Option<()> {
    VarIntWrapper(potion_effect_protocol_id(custom_effect.effect_id())?)
        .encode(data)
        .ok()?;
    encode_potion_effect_settings(custom_effect.settings(), data)
}

fn encode_potion_effect_settings(
    settings: &PotionEffectSettings,
    data: &mut Vec<u8>,
) -> Option<()> {
    VarIntWrapper(settings.amplifier()).encode(data).ok()?;
    VarIntWrapper(settings.duration()).encode(data).ok()?;
    settings.is_ambient().encode(data).ok()?;
    settings.show_particles().encode(data).ok()?;
    settings.show_icon().encode(data).ok()?;
    match settings.hidden_effect() {
        Some(hidden_effect) => {
            true.encode(data).ok()?;
            encode_potion_effect_settings(hidden_effect, data)
        }
        None => false.encode(data).ok(),
    }
}

fn potion_type_protocol_id(potion: &spinel_registry::Identifier) -> Option<i32> {
    static POTION_TYPES: &[&str] = &[
        "water",
        "mundane",
        "thick",
        "awkward",
        "night_vision",
        "long_night_vision",
        "invisibility",
        "long_invisibility",
        "leaping",
        "long_leaping",
        "strong_leaping",
        "fire_resistance",
        "long_fire_resistance",
        "swiftness",
        "long_swiftness",
        "strong_swiftness",
        "slowness",
        "long_slowness",
        "strong_slowness",
        "turtle_master",
        "long_turtle_master",
        "strong_turtle_master",
        "water_breathing",
        "long_water_breathing",
        "healing",
        "strong_healing",
        "harming",
        "strong_harming",
        "poison",
        "long_poison",
        "strong_poison",
        "regeneration",
        "long_regeneration",
        "strong_regeneration",
        "strength",
        "long_strength",
        "strong_strength",
        "weakness",
        "long_weakness",
        "luck",
        "slow_falling",
        "long_slow_falling",
        "wind_charged",
        "weaving",
        "oozing",
        "infested",
    ];
    protocol_id_from_identifier_path(potion, POTION_TYPES)
}

fn potion_effect_protocol_id(effect: &spinel_registry::Identifier) -> Option<i32> {
    static POTION_EFFECTS: &[&str] = &[
        "speed",
        "slowness",
        "haste",
        "mining_fatigue",
        "strength",
        "instant_health",
        "instant_damage",
        "jump_boost",
        "nausea",
        "regeneration",
        "resistance",
        "fire_resistance",
        "water_breathing",
        "invisibility",
        "blindness",
        "night_vision",
        "hunger",
        "weakness",
        "poison",
        "wither",
        "health_boost",
        "absorption",
        "saturation",
        "glowing",
        "levitation",
        "luck",
        "unluck",
        "slow_falling",
        "conduit_power",
        "dolphins_grace",
        "bad_omen",
        "hero_of_the_village",
        "darkness",
        "trial_omen",
        "raid_omen",
        "wind_charged",
        "weaving",
        "oozing",
        "infested",
        "breath_of_the_nautilus",
    ];
    protocol_id_from_identifier_path(effect, POTION_EFFECTS)
}

fn protocol_id_from_identifier_path(
    identifier: &spinel_registry::Identifier,
    protocol_names: &[&str],
) -> Option<i32> {
    protocol_names
        .iter()
        .position(|protocol_name| protocol_name == &identifier.path.as_ref())
        .map(|protocol_id| protocol_id as i32)
}

fn encode_weapon_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let weapon = Weapon::from_component_nbt(component_nbt)?;
    VarIntWrapper(weapon.item_damage_per_attack())
        .encode(data)
        .ok()?;
    weapon.disable_blocking_for_seconds().encode(data).ok()
}

fn encode_attack_range_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let attack_range = AttackRange::from_component_nbt(component_nbt)?;
    attack_range.min_reach().encode(data).ok()?;
    attack_range.max_reach().encode(data).ok()?;
    attack_range.min_creative_reach().encode(data).ok()?;
    attack_range.max_creative_reach().encode(data).ok()?;
    attack_range.hitbox_margin().encode(data).ok()?;
    attack_range.mob_factor().encode(data).ok()
}

fn encode_custom_model_data_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let custom_model_data = CustomModelData::from_component_nbt(component_nbt)?;
    custom_model_data.floats().to_vec().encode(data).ok()?;
    custom_model_data.flags().to_vec().encode(data).ok()?;
    custom_model_data.strings().to_vec().encode(data).ok()?;
    custom_model_data.colors().to_vec().encode(data).ok()
}

fn encode_tooltip_display_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let tooltip_display = TooltipDisplay::from_component_nbt(component_nbt)?;
    tooltip_display.hide_tooltip().encode(data).ok()?;
    VarIntWrapper(tooltip_display.hidden_component_ids().len() as i32)
        .encode(data)
        .ok()?;
    for component_id in tooltip_display.hidden_component_ids() {
        DataComponentDescriptor::from_id(*component_id)?;
        VarIntWrapper(*component_id).encode(data).ok()?;
    }
    Some(())
}

fn encode_swing_animation_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let swing_animation = SwingAnimation::from_component_nbt(component_nbt)?;
    VarIntWrapper(swing_animation.animation_type().protocol_id())
        .encode(data)
        .ok()?;
    VarIntWrapper(swing_animation.duration()).encode(data).ok()
}

fn encode_map_post_processing_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let map_post_processing = MapPostProcessing::from_component_nbt(component_nbt)?;
    VarIntWrapper(map_post_processing.protocol_id())
        .encode(data)
        .ok()
}

fn encode_firework_explosion_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let firework_explosion = FireworkExplosion::from_component_nbt(component_nbt)?;
    encode_firework_explosion_value(&firework_explosion, data)
}

fn encode_firework_list_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let firework_list = FireworkList::from_component_nbt(component_nbt)?;
    VarIntWrapper(firework_list.flight_duration())
        .encode(data)
        .ok()?;
    VarIntWrapper(firework_list.explosions().len() as i32)
        .encode(data)
        .ok()?;
    for firework_explosion in firework_list.explosions() {
        encode_firework_explosion_value(firework_explosion, data)?;
    }
    Some(())
}

fn encode_firework_explosion_value(
    firework_explosion: &FireworkExplosion,
    data: &mut Vec<u8>,
) -> Option<()> {
    VarIntWrapper(firework_explosion.shape().protocol_id())
        .encode(data)
        .ok()?;
    firework_explosion.colors().to_vec().encode(data).ok()?;
    firework_explosion
        .fade_colors()
        .to_vec()
        .encode(data)
        .ok()?;
    firework_explosion.has_trail().encode(data).ok()?;
    firework_explosion.has_twinkle().encode(data).ok()
}

fn encode_pot_decorations_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let pot_decorations = PotDecorations::from_component_nbt(component_nbt)?;
    let materials = pot_decorations.as_list();
    VarIntWrapper(materials.len() as i32).encode(data).ok()?;
    for material in materials {
        VarIntWrapper(material.id()).encode(data).ok()?;
    }
    Some(())
}

fn encode_item_block_state_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let item_block_state = ItemBlockState::from_component_nbt(component_nbt)?;
    VarIntWrapper(item_block_state.properties().len() as i32)
        .encode(data)
        .ok()?;
    for (key, value) in item_block_state.properties() {
        key.clone().encode(data).ok()?;
        value.clone().encode(data).ok()?;
    }
    Some(())
}

fn encode_debug_stick_state_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let debug_stick_state = DebugStickState::from_component_nbt(component_nbt)?;
    let Nbt::Compound(compound) = debug_stick_state.to_component_nbt() else {
        return None;
    };
    compound.encode(data).ok()
}

fn encode_writable_book_content_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let writable_book_content = WritableBookContent::from_component_nbt(component_nbt)?;
    VarIntWrapper(writable_book_content.pages().len() as i32)
        .encode(data)
        .ok()?;
    for page in writable_book_content.pages() {
        page.text().to_string().encode(data).ok()?;
        page.filtered().map(str::to_string).encode(data).ok()?;
    }
    Some(())
}

fn encode_written_book_content_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let written_book_content = WrittenBookContent::from_component_nbt(component_nbt)?;
    written_book_content
        .title()
        .text()
        .to_string()
        .encode(data)
        .ok()?;
    written_book_content
        .title()
        .filtered()
        .map(str::to_string)
        .encode(data)
        .ok()?;
    written_book_content
        .author()
        .to_string()
        .encode(data)
        .ok()?;
    VarIntWrapper(written_book_content.generation())
        .encode(data)
        .ok()?;
    VarIntWrapper(written_book_content.pages().len() as i32)
        .encode(data)
        .ok()?;
    for page in written_book_content.pages() {
        JsonTextComponent(page.text().clone()).encode(data).ok()?;
        page.filtered()
            .cloned()
            .map(JsonTextComponent)
            .encode(data)
            .ok()?;
    }
    written_book_content.resolved().encode(data).ok()
}

fn encode_profile_component(component_nbt: &Nbt, data: &mut Vec<u8>) -> Option<()> {
    let profile = ComponentResolvableProfile::from_component_nbt(component_nbt)?;
    let uuid = profile.uuid().and_then(|uuid| uuid.parse::<Uuid>().ok());
    let name = profile.name().map(str::to_string);
    let properties = profile
        .properties()
        .iter()
        .map(|property| NetworkGameProfileProperty {
            name: property.name().to_string(),
            value: property.value().to_string(),
            signature: property.signature().map(str::to_string),
        })
        .collect();
    let identity = match (uuid, name.as_ref().filter(|name| name.len() <= 16)) {
        (Some(uuid), Some(name)) => ResolvableProfileIdentity::Complete(NetworkGameProfile {
            uuid,
            username: name.clone(),
            properties,
        }),
        _ => ResolvableProfileIdentity::Partial(PartialGameProfile {
            name,
            uuid,
            properties,
        }),
    };
    let body = profile.texture().map(str::parse).transpose().ok()?;
    NetworkResolvableProfile {
        identity,
        skin_patch: PlayerSkinPatch {
            body,
            cape: None,
            elytra: None,
            is_slim: None,
        },
    }
    .encode(data)
    .ok()
}
