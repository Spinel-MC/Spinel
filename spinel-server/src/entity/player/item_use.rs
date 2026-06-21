use crate::entity::metadata::definitions;
use crate::entity::{EntityPosition, Player, PlayerHand, TimedPotionEffect};
use crate::events::player_begin_item_use::PlayerBeginItemUseEvent;
use crate::events::player_cancel_item_use::PlayerCancelItemUseEvent;
use crate::events::player_finish_item_use::PlayerFinishItemUseEvent;
use crate::events::player_use_item::PlayerUseItemEvent;
use crate::inventory::slot_conversion::OFFHAND_SLOT;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::entity_sound_effect::{
    EntitySoundEffectPacket, NetworkSoundEvent,
};
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{TeleportFlags, Vector3d};
use spinel_registry::data_components::vanilla_components::{
    BLOCKS_ATTACKS, CONSUMABLE, EQUIPPABLE, FOOD, INSTRUMENT, USE_REMAINDER,
};
use spinel_registry::{
    ConsumeEffect, CustomPotionEffect, Identifier, ItemAnimation, ItemStack, Material,
    RegistryTagReference,
};
use std::io;

const MARK_ITEM_FINISHED: i8 = 9;
const BOW_USE_DURATION_TICKS: u64 = 72_000;
const CROSSBOW_USE_DURATION_TICKS: u64 = 7_200;
const BLOCK_USE_DURATION_TICKS: u64 = 72_000;
const TRIDENT_USE_DURATION_TICKS: u64 = 72_000;
const SPYGLASS_USE_DURATION_TICKS: u64 = 1_200;
const BRUSH_USE_DURATION_TICKS: u64 = 200;
const BUNDLE_USE_DURATION_TICKS: u64 = 200;

pub(crate) struct PlayerItemUseCompletion {
    pub(crate) entity_id: i32,
    pub(crate) status: i8,
    pub(crate) player: *mut Player,
    pub(crate) hand: PlayerHand,
    pub(crate) item_stack: ItemStack,
    pub(crate) duration: u64,
}

struct ItemUseState {
    duration: u64,
    animation: ItemAnimation,
}

impl Player {
    pub fn is_using_item(&self) -> bool {
        self.item_use_hand.is_some()
    }

    pub fn is_eating(&self) -> bool {
        self.item_use_hand
            .is_some_and(|hand| player_hand_item_is_food(self.item_in_hand(hand)))
    }

    pub fn get_item_use_hand(&self) -> Option<PlayerHand> {
        self.item_use_hand
    }

    pub fn get_current_item_use_time(&self) -> u64 {
        if self.item_use_hand.is_none() {
            return 0;
        }
        self.alive_ticks.saturating_sub(self.start_item_use_time)
    }

    pub fn refresh_item_use(&mut self, item_use_hand: Option<PlayerHand>, item_use_time: u64) {
        self.item_use_hand = item_use_hand;
        self.item_use_time = item_use_time;
        self.start_item_use_time = match item_use_hand {
            Some(_) => self.alive_ticks,
            None => 0,
        };
    }

    pub fn clear_item_use(&mut self) {
        self.refresh_item_use(None, 0);
    }

    pub(crate) fn refresh_active_hand(
        &mut self,
        is_hand_active: bool,
        is_off_hand: bool,
        is_riptide_spin_attack: bool,
    ) {
        self.metadata.set(
            &definitions::living_entity_flags(),
            spinel_network::types::entity_metadata::MetadataValue::Byte(0),
        );
        self.metadata
            .set_flag(&definitions::is_hand_active(), is_hand_active);
        self.metadata
            .set_flag(&definitions::get_active_hand(), is_off_hand);
        self.metadata.set_flag(
            &definitions::is_riptide_spin_attack(),
            is_riptide_spin_attack,
        );
        self.refresh_pose();
    }

    pub(crate) fn use_item(
        &mut self,
        hand: PlayerHand,
        current_tick: u64,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> io::Result<bool> {
        if !self.use_item_with_cooldown(hand, current_tick, client)? {
            return Ok(false);
        }
        let item_stack = self.item_in_hand(hand);
        let item_use_state = item_use_state(&item_stack, &server.registries);
        let is_using_main_hand_while_off_hand_was_requested =
            self.item_use_hand == Some(PlayerHand::Main) && hand == PlayerHand::Off;
        let item_use_time = if is_using_main_hand_while_off_hand_was_requested {
            0
        } else {
            item_use_state.as_ref().map_or(0, |state| state.duration)
        };
        let mut use_item_event =
            PlayerUseItemEvent::new(self as *mut Player, hand, item_stack.clone(), item_use_time);
        use_item_event.dispatch(server, client);
        if use_item_event.is_cancelled() {
            let _ = self.sync_inventory(client);
            return Ok(false);
        }
        let Some(item_use_state) = item_use_state else {
            return self.equip_swappable_armor_item_from_hand(hand, item_stack, client);
        };
        if use_item_event.item_use_time() == 0 {
            return self.equip_swappable_armor_item_from_hand(hand, item_stack, client);
        }
        let mut begin_item_use_event = PlayerBeginItemUseEvent::new(
            self as *mut Player,
            hand,
            item_stack,
            item_use_state.animation,
            use_item_event.item_use_time(),
        );
        begin_item_use_event.dispatch(server, client);
        if begin_item_use_event.is_cancelled() {
            return Ok(false);
        }
        self.refresh_item_use(Some(hand), begin_item_use_event.item_use_duration());
        self.refresh_active_hand(true, hand == PlayerHand::Off, false);
        Ok(true)
    }

    fn equip_swappable_armor_item_from_hand(
        &mut self,
        hand: PlayerHand,
        item_stack: ItemStack,
        client: &mut Client,
    ) -> io::Result<bool> {
        let Some(equippable) = item_stack.get(EQUIPPABLE) else {
            return Ok(true);
        };
        if !equippable.swappable() {
            return Ok(true);
        }
        let Some(equipment_slot) =
            crate::entity::EquipmentSlot::from_equippable_armor_slot(equippable.slot())
        else {
            return Ok(true);
        };
        let currently_equipped_item = self.get_equipment(equipment_slot);
        if !self.set_equipment(equipment_slot, item_stack) {
            return Ok(false);
        }
        if !self.set_item_in_hand(hand, currently_equipped_item) {
            return Ok(false);
        }
        self.sync_slot(equipment_slot.armor_slot(), client)?;
        self.sync_slot(self.inventory_slot_for_hand(hand), client)?;
        if hand == PlayerHand::Main {
            self.sync_main_hand_attributes(client)?;
        }
        Ok(true)
    }

    fn inventory_slot_for_hand(&self, hand: PlayerHand) -> i32 {
        match hand {
            PlayerHand::Main => self.get_held_slot(),
            PlayerHand::Off => OFFHAND_SLOT,
        }
    }

    pub(crate) fn cancel_item_use(
        &mut self,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(hand) = self.item_use_hand else {
            return true;
        };
        let item_stack = self.item_in_hand(hand);
        let mut cancel_item_use_event = PlayerCancelItemUseEvent::new(
            self as *mut Player,
            hand,
            item_stack,
            self.get_current_item_use_time(),
        );
        cancel_item_use_event.dispatch(server, client);
        self.refresh_active_hand(
            false,
            hand == PlayerHand::Off,
            cancel_item_use_event.is_riptide_spin_attack(),
        );
        self.clear_item_use();
        true
    }

    pub(crate) fn tick_item_use(&mut self) -> Option<PlayerItemUseCompletion> {
        self.alive_ticks = self.alive_ticks.saturating_add(1);
        let Some(hand) = self.item_use_hand else {
            return None;
        };
        if self.get_current_item_use_time() < self.item_use_time {
            return None;
        }
        let item_stack = self.item_in_hand(hand);
        let duration = self.get_current_item_use_time();
        self.refresh_active_hand(false, self.item_use_hand == Some(PlayerHand::Off), false);
        self.clear_item_use();
        Some(PlayerItemUseCompletion {
            entity_id: self.get_entity_id().get_value(),
            status: MARK_ITEM_FINISHED,
            player: self as *mut Player,
            hand,
            item_stack,
            duration,
        })
    }

    pub(crate) fn finish_item_use(
        &mut self,
        hand: PlayerHand,
        item_stack: ItemStack,
        duration: u64,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> io::Result<()> {
        let mut event =
            PlayerFinishItemUseEvent::new(self as *mut Player, hand, item_stack.clone(), duration);
        event.dispatch(server, client);
        self.apply_consumed_item(hand, item_stack, server, client)
    }

    fn apply_consumed_item(
        &mut self,
        hand: PlayerHand,
        item_stack: ItemStack,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> io::Result<()> {
        let Some(consumable) = item_stack.get(CONSUMABLE) else {
            return Ok(());
        };
        self.apply_consumable_effects(consumable.effects(), server, client)?;
        self.increment_statistic_value(
            format!("minecraft:used:{}", item_stack.material().key()),
            1,
        );
        let replacement_item = item_stack.get(USE_REMAINDER);
        let updated_item = replacement_item.unwrap_or_else(|| item_stack.consume(1));
        self.set_item_in_hand(hand, updated_item);
        self.sync_slot(self.inventory_slot_for_hand(hand), client)?;
        self.sync_inventory(client)
    }

    fn apply_consumable_effects(
        &mut self,
        effects: &[ConsumeEffect],
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> io::Result<()> {
        effects
            .iter()
            .try_for_each(|effect| self.apply_consumable_effect(effect, server, client))
    }

    fn apply_consumable_effect(
        &mut self,
        effect: &ConsumeEffect,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> io::Result<()> {
        match effect {
            ConsumeEffect::ApplyEffects {
                effects,
                probability,
            } => self.apply_consumable_potion_effects(effects, *probability, server, client),
            ConsumeEffect::RemoveEffects { effects } => {
                self.remove_consumable_potion_effects(effects, server, client)
            }
            ConsumeEffect::ClearAllEffects => self.clear_consumable_potion_effects(client),
            ConsumeEffect::TeleportRandomly { diameter } => {
                self.teleport_randomly_after_consuming(*diameter)
            }
            ConsumeEffect::PlaySound { sound } => self.play_consumable_sound(sound, client),
        }
    }

    fn apply_consumable_potion_effects(
        &mut self,
        effects: &[CustomPotionEffect],
        probability: f32,
        server: &MinecraftServer,
        client: &mut Client,
    ) -> io::Result<()> {
        if probability <= 0.0 {
            return Ok(());
        }
        if probability < 1.0 && !probability_succeeds(probability, self.alive_ticks) {
            return Ok(());
        }
        effects.iter().try_for_each(|effect| {
            let Some(effect_id) = server
                .registries
                .dynamic_registry_id(&spinel_registry::MOB_EFFECT_REGISTRY, effect.effect_id())
            else {
                return Ok(());
            };
            let settings = effect.get_settings();
            let packet = self.add_effect(TimedPotionEffect::new(
                effect_id,
                settings.amplifier(),
                settings.duration(),
                potion_effect_flags(settings),
                self.alive_ticks,
            ));
            packet.dispatch(client)
        })
    }

    fn remove_consumable_potion_effects(
        &mut self,
        effects: &RegistryTagReference,
        server: &MinecraftServer,
        client: &mut Client,
    ) -> io::Result<()> {
        let removable_effect_ids = self
            .get_active_effects()
            .into_iter()
            .filter_map(|effect| {
                effect_reference_contains(effects, effect.get_effect_id(), server)
                    .then_some(effect.get_effect_id())
            })
            .collect::<Vec<_>>();
        removable_effect_ids.into_iter().try_for_each(|effect_id| {
            let Some(packet) = self.remove_effect(effect_id) else {
                return Ok(());
            };
            packet.dispatch(client)
        })
    }

    fn clear_consumable_potion_effects(&mut self, client: &mut Client) -> io::Result<()> {
        self.clear_effects()
            .into_iter()
            .try_for_each(|packet| packet.dispatch(client))
    }

    fn teleport_randomly_after_consuming(&mut self, diameter: f32) -> io::Result<()> {
        let radius = f64::from(diameter.max(0.0)) / 2.0;
        let (offset_x, offset_z) = random_teleport_offsets(radius, self.alive_ticks);
        let position = self.get_position();
        let target_position = EntityPosition::new(
            position.get_x() + offset_x,
            position.get_y(),
            position.get_z() + offset_z,
            position.get_yaw(),
            position.get_pitch(),
        );
        self.set_position(target_position);
        self.synchronize_position_after_teleport(
            target_position,
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            TeleportFlags::absolute(),
            true,
        )
        .map(|_| ())
    }

    fn play_consumable_sound(&mut self, sound: &Identifier, client: &mut Client) -> io::Result<()> {
        EntitySoundEffectPacket {
            sound_event: NetworkSoundEvent(SoundEvent::Named {
                name: sound.to_string(),
                fixed_range: None,
            }),
            source_id: 7,
            entity_id: self.get_entity_id().get_value(),
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        }
        .dispatch(client)
    }
}

fn effect_reference_contains(
    reference: &RegistryTagReference,
    effect_id: i32,
    server: &MinecraftServer,
) -> bool {
    match reference {
        RegistryTagReference::Backed(tag_name) => server
            .registries
            .mob_effect_tag_contains(tag_name, effect_id),
        RegistryTagReference::Direct(effect_names) => server
            .registries
            .mob_effect_key(effect_id)
            .is_some_and(|effect_key| {
                effect_names
                    .iter()
                    .any(|effect_name| effect_name == effect_key)
            }),
        RegistryTagReference::Empty => false,
    }
}

fn potion_effect_flags(settings: &spinel_registry::PotionEffectSettings) -> i8 {
    i8::from(settings.is_ambient())
        | (i8::from(settings.show_particles()) << 1)
        | (i8::from(settings.show_icon()) << 2)
}

fn probability_succeeds(probability: f32, tick: u64) -> bool {
    random_unit(tick, 0) < probability.clamp(0.0, 1.0)
}

fn random_teleport_offsets(radius: f64, tick: u64) -> (f64, f64) {
    let offset_x = (random_unit(tick, 1) as f64 * 2.0 - 1.0) * radius;
    let offset_z = (random_unit(tick, 2) as f64 * 2.0 - 1.0) * radius;
    (offset_x, offset_z)
}

fn random_unit(tick: u64, salt: u64) -> f32 {
    let mixed = tick
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(salt.wrapping_mul(0xBF58_476D_1CE4_E5B9));
    let value = mixed ^ (mixed >> 33);
    (value as f64 / u64::MAX as f64) as f32
}

fn player_hand_item_is_food(item_stack: ItemStack) -> bool {
    item_stack.has(FOOD) || item_stack.material() == &Material::POTION
}

fn item_use_state(
    item_stack: &ItemStack,
    registries: &spinel_registry::Registries,
) -> Option<ItemUseState> {
    let material = item_stack.material();
    if material == &Material::BOW {
        return Some(ItemUseState {
            duration: BOW_USE_DURATION_TICKS,
            animation: ItemAnimation::Bow,
        });
    }
    if material == &Material::CROSSBOW {
        return Some(ItemUseState {
            duration: CROSSBOW_USE_DURATION_TICKS,
            animation: ItemAnimation::Crossbow,
        });
    }
    if item_stack.has(BLOCKS_ATTACKS) {
        return Some(ItemUseState {
            duration: BLOCK_USE_DURATION_TICKS,
            animation: ItemAnimation::Block,
        });
    }
    if material == &Material::TRIDENT {
        return Some(ItemUseState {
            duration: TRIDENT_USE_DURATION_TICKS,
            animation: ItemAnimation::Spear,
        });
    }
    if material == &Material::SPYGLASS {
        return Some(ItemUseState {
            duration: SPYGLASS_USE_DURATION_TICKS,
            animation: ItemAnimation::Spyglass,
        });
    }
    if material == &Material::GOAT_HORN {
        let instrument_use_duration = goat_horn_instrument_use_duration(item_stack, registries)?;
        return Some(ItemUseState {
            duration: instrument_use_duration,
            animation: ItemAnimation::TootHorn,
        });
    }
    if material == &Material::BRUSH {
        return Some(ItemUseState {
            duration: BRUSH_USE_DURATION_TICKS,
            animation: ItemAnimation::Brush,
        });
    }
    if material.key().to_string().contains("bundle") {
        return Some(ItemUseState {
            duration: BUNDLE_USE_DURATION_TICKS,
            animation: ItemAnimation::Bundle,
        });
    }
    item_stack.get(CONSUMABLE).map(|consumable| ItemUseState {
        duration: u64::try_from(consumable.consume_ticks()).unwrap_or(0),
        animation: consumable.animation(),
    })
}

fn goat_horn_instrument_use_duration(
    item_stack: &ItemStack,
    registries: &spinel_registry::Registries,
) -> Option<u64> {
    let instrument = item_stack.get(INSTRUMENT)?;
    let use_duration_ticks = registries
        .instrument(instrument.instrument())?
        .use_duration_ticks();
    u64::try_from(use_duration_ticks).ok()
}
