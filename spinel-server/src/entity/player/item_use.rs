use crate::entity::metadata::definitions;
use crate::entity::{Player, PlayerHand};
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
use spinel_registry::data_components::vanilla_components::{
    BLOCKS_ATTACKS, CONSUMABLE, EQUIPPABLE, FOOD, INSTRUMENT, USE_REMAINDER,
};
use spinel_registry::{ConsumeEffect, ItemAnimation, ItemStack, Material};
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

    pub fn item_use_hand(&self) -> Option<PlayerHand> {
        self.item_use_hand
    }

    pub fn current_item_use_time(&self) -> u64 {
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
            .set_flag(&definitions::active_hand(), is_off_hand);
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
        let item_use_state = item_use_state(&item_stack);
        let item_use_time = item_use_state.as_ref().map_or(0, |state| state.duration);
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
        let currently_equipped_item = self.equipment(equipment_slot);
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
            PlayerHand::Main => self.held_slot(),
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
            self.current_item_use_time(),
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
        if self.current_item_use_time() < self.item_use_time {
            return None;
        }
        let item_stack = self.item_in_hand(hand);
        let duration = self.current_item_use_time();
        self.refresh_active_hand(false, self.item_use_hand == Some(PlayerHand::Off), false);
        self.clear_item_use();
        Some(PlayerItemUseCompletion {
            entity_id: self.entity_id().value(),
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
        self.apply_consumed_item(hand, item_stack, client)
    }

    fn apply_consumed_item(
        &mut self,
        hand: PlayerHand,
        item_stack: ItemStack,
        client: &mut Client,
    ) -> io::Result<()> {
        let Some(consumable) = item_stack.get(CONSUMABLE) else {
            return Ok(());
        };
        self.apply_consumable_effects(consumable.effects(), client)?;
        let replacement_item = item_stack.get(USE_REMAINDER);
        let updated_item = replacement_item.unwrap_or_else(|| item_stack.consume(1));
        self.set_item_in_hand(hand, updated_item);
        self.sync_slot(self.inventory_slot_for_hand(hand), client)?;
        self.sync_inventory(client)
    }

    fn apply_consumable_effects(
        &mut self,
        effects: &[ConsumeEffect],
        client: &mut Client,
    ) -> io::Result<()> {
        effects
            .iter()
            .try_for_each(|effect| self.apply_consumable_effect(effect, client))
    }

    fn apply_consumable_effect(
        &mut self,
        effect: &ConsumeEffect,
        client: &mut Client,
    ) -> io::Result<()> {
        let ConsumeEffect::PlaySound { sound } = effect else {
            return Ok(());
        };
        EntitySoundEffectPacket {
            sound_event: NetworkSoundEvent(SoundEvent::Named {
                name: sound.to_string(),
                fixed_range: None,
            }),
            source_id: 7,
            entity_id: self.entity_id().value(),
            volume: 1.0,
            pitch: 1.0,
            seed: 0,
        }
        .dispatch(client)
    }
}

fn player_hand_item_is_food(item_stack: ItemStack) -> bool {
    item_stack.has(FOOD) || item_stack.material() == &Material::POTION
}

fn item_use_state(item_stack: &ItemStack) -> Option<ItemUseState> {
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
    if material == &Material::GOAT_HORN && item_stack.has(INSTRUMENT) {
        return Some(ItemUseState {
            duration: 0,
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

#[cfg(test)]
mod tests {
    use super::Player;
    use crate::entity::{EquipmentSlot, PlayerHand};
    use crate::network::client::instance::Client;
    use crate::server::MinecraftServer;
    use spinel_registry::data_components::vanilla_components::{CONSUMABLE, USE_REMAINDER};
    use spinel_registry::{
        Consumable, ConsumeEffect, Identifier, ItemAnimation, ItemStack, Material,
    };
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use uuid::Uuid;

    #[test]
    fn right_click_swappable_armor_matches_minestom_equipment_swap() {
        let mut player = test_player();
        let mut server = MinecraftServer::new();
        let (mut client, _peer_stream) = test_client();
        player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND_HELMET));

        assert!(
            player
                .use_item(PlayerHand::Main, 0, &mut server, &mut client)
                .unwrap()
        );
        assert_eq!(
            player.equipment(EquipmentSlot::Helmet).material(),
            &Material::DIAMOND_HELMET
        );
        assert_eq!(
            player.item_in_hand(PlayerHand::Main).material(),
            &Material::AIR
        );
    }

    #[test]
    fn right_click_armor_places_previous_equipment_in_used_hand() {
        let mut player = test_player();
        let mut server = MinecraftServer::new();
        let (mut client, _peer_stream) = test_client();
        player.set_equipment(
            EquipmentSlot::Helmet,
            ItemStack::of(Material::GOLDEN_HELMET),
        );
        player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND_HELMET));

        assert!(
            player
                .use_item(PlayerHand::Main, 0, &mut server, &mut client)
                .unwrap()
        );
        assert_eq!(
            player.equipment(EquipmentSlot::Helmet).material(),
            &Material::DIAMOND_HELMET
        );
        assert_eq!(
            player.item_in_hand(PlayerHand::Main).material(),
            &Material::GOLDEN_HELMET
        );
    }

    #[test]
    fn finished_consumable_decrements_used_hand() {
        let mut player = test_player();
        let mut server = MinecraftServer::new();
        let (mut client, _peer_stream) = test_client();
        let apple_stack = ItemStack::of(Material::APPLE).with_amount(2).with(
            CONSUMABLE,
            Consumable::new(
                1.6,
                ItemAnimation::Eat,
                Identifier::vanilla_static("entity.generic.eat"),
                true,
                Vec::new(),
            ),
        );
        player.set_item_in_hand(PlayerHand::Main, apple_stack.clone());

        player
            .finish_item_use(PlayerHand::Main, apple_stack, 32, &mut server, &mut client)
            .unwrap();

        assert_eq!(player.item_in_hand(PlayerHand::Main).amount(), 1);
    }

    #[test]
    fn finished_consumable_uses_remainder_and_dispatches_play_sound_effect() {
        let mut player = test_player();
        let mut server = MinecraftServer::new();
        let (mut client, _peer_stream) = test_client();
        let potion_stack = ItemStack::of(Material::POTION)
            .with(USE_REMAINDER, ItemStack::of(Material::GLASS_BOTTLE))
            .with(
                CONSUMABLE,
                Consumable::new(
                    1.6,
                    ItemAnimation::Drink,
                    Identifier::vanilla_static("entity.generic.drink"),
                    true,
                    vec![ConsumeEffect::PlaySound {
                        sound: Identifier::vanilla_static("entity.generic.drink"),
                    }],
                ),
            );
        player.set_item_in_hand(PlayerHand::Main, potion_stack.clone());

        player
            .finish_item_use(PlayerHand::Main, potion_stack, 32, &mut server, &mut client)
            .unwrap();

        assert_eq!(
            player.item_in_hand(PlayerHand::Main).material(),
            &Material::GLASS_BOTTLE
        );
    }

    fn test_player() -> Player {
        Player::new(
            Uuid::new_v4(),
            "Player".to_string(),
            0,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        )
    }

    fn test_client() -> (Client, TcpStream) {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
        let addr = listener.local_addr().unwrap();
        let peer_stream = TcpStream::connect(addr).unwrap();
        let (stream, _) = listener.accept().unwrap();
        (Client::new(stream, addr), peer_stream)
    }
}
