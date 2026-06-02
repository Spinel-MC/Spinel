use crate::entity::EntityId;
use spinel_nbt::{TagHandler, Taggable};
use spinel_network::types::Vector3d;
use spinel_network::types::sound::SoundEvent;
use spinel_registry::damage_type::DamageType;
use spinel_registry::{BuiltinSoundEvent, DAMAGE_TYPE_REGISTRY, Registries, RegistryKey};
use spinel_utils::component::text::TextComponent;

#[derive(Clone, Debug, PartialEq)]
pub struct Damage {
    damage_type: RegistryKey<DamageType>,
    attacker: Option<EntityId>,
    source: Option<EntityId>,
    source_position: Option<Vector3d>,
    amount: f32,
    sound: Option<SoundEvent>,
    should_animate: bool,
    resolved_type: Option<DamageType>,
    tag_handler: TagHandler,
}

impl Damage {
    pub fn new(damage_type: RegistryKey<DamageType>, amount: f32) -> Self {
        Self {
            damage_type,
            attacker: None,
            source: None,
            source_position: None,
            amount: amount.max(0.0),
            sound: None,
            should_animate: true,
            resolved_type: None,
            tag_handler: TagHandler::new_handler(),
        }
    }

    pub fn with_attacker(mut self, attacker: EntityId) -> Self {
        self.attacker = Some(attacker);
        self
    }

    pub fn with_source(mut self, source: EntityId) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_source_position(mut self, source_position: Vector3d) -> Self {
        self.source_position = Some(source_position);
        self
    }

    pub fn with_sound(mut self, sound: SoundEvent) -> Self {
        self.sound = Some(sound);
        self
    }

    pub fn without_animation(mut self) -> Self {
        self.should_animate = false;
        self
    }

    pub fn damage_type(&self) -> &RegistryKey<DamageType> {
        &self.damage_type
    }

    pub const fn attacker(&self) -> Option<EntityId> {
        self.attacker
    }

    pub const fn source(&self) -> Option<EntityId> {
        self.source
    }

    pub const fn source_position(&self) -> Option<Vector3d> {
        self.source_position
    }

    pub const fn amount(&self) -> f32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.max(0.0);
    }

    pub fn sound(&self) -> Option<SoundEvent> {
        self.sound.clone()
    }

    pub fn set_sound(&mut self, sound: Option<SoundEvent>) {
        self.sound = sound;
    }

    pub const fn should_animate(&self) -> bool {
        self.should_animate
    }

    pub fn set_should_animate(&mut self, should_animate: bool) {
        self.should_animate = should_animate;
    }

    pub fn damage_type_id(&self, registries: &Registries) -> Option<i32> {
        registries.dynamic_registry_id(&DAMAGE_TYPE_REGISTRY, self.damage_type.key())
    }

    pub fn resolve_type(&mut self, registries: &Registries) -> bool {
        let Some(damage_type) = registries.damage_type(&self.damage_type) else {
            return false;
        };
        self.resolved_type = Some(damage_type.clone());
        true
    }

    pub fn default_sound(&self, is_player: bool) -> SoundEvent {
        if !is_player {
            return SoundEvent::Id(BuiltinSoundEvent::ENTITY_GENERIC_HURT.id());
        }
        if self.damage_type == DamageType::ON_FIRE {
            return SoundEvent::Id(BuiltinSoundEvent::ENTITY_PLAYER_HURT_ON_FIRE.id());
        }
        SoundEvent::Id(BuiltinSoundEvent::ENTITY_PLAYER_HURT.id())
    }

    pub fn build_death_screen_text(&self) -> Option<TextComponent> {
        Some(TextComponent::translatable(self.death_translation_key()).build())
    }

    pub fn build_death_message(&self, killed_username: &str) -> Option<TextComponent> {
        Some(
            TextComponent::translatable(self.death_translation_key())
                .argument(TextComponent::literal(killed_username))
                .build(),
        )
    }

    fn death_translation_key(&self) -> String {
        format!(
            "death.attack.{}",
            self.resolved_type
                .as_ref()
                .map(DamageType::message_id)
                .unwrap_or_else(|| self.damage_type.key().path.as_ref())
        )
    }
}

impl Taggable for Damage {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}
