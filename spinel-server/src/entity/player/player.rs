use crate::entity::PlayerSpawnPoint;
use crate::entity::player::chunks::PlayerChunk;
use crate::entity::player::position::PlayerPosition;
use crate::events::inventory_close::InventoryCloseEvent;
use crate::events::item_drop::ItemDropEvent;
use crate::events::player_swap_item::PlayerSwapItemEvent;
use crate::inventory::{ClickPreprocessor, Inventory, PlayerInventory};
use crate::network::client::instance::Client;
use spinel_core::entity::game_mode::GameMode;
use spinel_core::network::clientbound::play::container_close::ContainerClosePacket;
use spinel_core::network::clientbound::play::ticking_state::TickingStatePacket;
use spinel_core::network::clientbound::play::ticking_step::TickingStepPacket;
use spinel_nbt::{TagHandler, Taggable};
use spinel_registry::ItemStack;
use std::io;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub protocol_version: i32,
    pub addr: SocketAddr,
    pub(crate) loaded_chunk: PlayerChunk,
    pub(crate) position: PlayerPosition,
    game_mode: GameMode,
    respawn_point: PlayerSpawnPoint,
    inventory: PlayerInventory,
    open_inventory: Option<Inventory>,
    pub(super) click_preprocessor: ClickPreprocessor,
    held_slot: i32,
    tag_handler: TagHandler,
    pub(super) last_completed_client_tick: u64,
    did_close_inventory: bool,
}

impl Player {
    pub fn new(uuid: Uuid, username: String, protocol_version: i32, addr: SocketAddr) -> Self {
        let respawn_point = PlayerSpawnPoint::default();
        let position = PlayerPosition::from(respawn_point);
        Self {
            uuid,
            username,
            protocol_version,
            addr,
            loaded_chunk: PlayerChunk::from_position(position),
            position,
            game_mode: GameMode::Survival,
            respawn_point,
            inventory: PlayerInventory::new(),
            open_inventory: None,
            click_preprocessor: ClickPreprocessor::default(),
            held_slot: 0,
            tag_handler: TagHandler::new_handler(),
            last_completed_client_tick: 0,
            did_close_inventory: false,
        }
    }

    pub fn set_game_mode(&mut self, game_mode: GameMode) -> bool {
        self.game_mode = game_mode;
        true
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn set_respawn_point(&mut self, respawn_point: PlayerSpawnPoint) {
        self.respawn_point = respawn_point;
    }

    pub fn respawn_point(&self) -> PlayerSpawnPoint {
        self.respawn_point
    }

    pub fn inventory(&mut self) -> &mut PlayerInventory {
        &mut self.inventory
    }

    pub fn inventory_ref(&self) -> &PlayerInventory {
        &self.inventory
    }

    pub fn open_inventory(&mut self, inventory: Inventory) {
        self.open_inventory = Some(inventory);
    }

    pub fn close_inventory(&mut self) {
        self.click_preprocessor.clear_cache();
        self.open_inventory = None;
    }

    pub(crate) fn close_inventory_with_client(
        &mut self,
        from_client: bool,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let window_id = self
            .opened_inventory()
            .map(|inventory| inventory.window_id())
            .unwrap_or_else(|| self.inventory_ref().window_id());
        self.close_inventory_window_with_client(from_client, window_id, server, client)
    }

    pub(crate) fn close_inventory_window_with_client(
        &mut self,
        from_client: bool,
        window_id: i32,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if window_id == self.inventory_ref().window_id() {
            self.close_inventory();
            return self.sync_player_inventory_window_contents(client).is_ok();
        }
        let Some(open_inventory) = self.opened_inventory().cloned() else {
            return false;
        };
        if window_id != open_inventory.window_id() {
            return false;
        }
        let mut event = InventoryCloseEvent::new(self as *mut Player, open_inventory, from_client);
        event.dispatch(server, client);
        if !from_client {
            self.did_close_inventory = true;
        }
        let cursor_item = self.inventory_ref().cursor_item().clone();
        self.close_inventory();
        self.inventory().set_cursor_item(ItemStack::air());
        if !cursor_item.is_air() && !self.drop_item(cursor_item.clone(), server, client) {
            let _ = self.inventory().add_item_stack(cursor_item);
        }
        let player_inventory_window_is_synced =
            self.sync_player_inventory_window_contents(client).is_ok();
        if !from_client {
            let packet_result = ContainerClosePacket {
                container_id: event.inventory().id().into(),
            }
            .dispatch(client)
            .is_ok();
            self.did_close_inventory = false;
            return packet_result && player_inventory_window_is_synced;
        }
        self.did_close_inventory = false;
        player_inventory_window_is_synced
    }

    pub fn opened_inventory(&self) -> Option<&Inventory> {
        self.open_inventory.as_ref()
    }

    pub(crate) fn opened_inventory_mut(&mut self) -> Option<&mut Inventory> {
        self.open_inventory.as_mut()
    }

    pub(crate) fn click_preprocessor(&mut self) -> &mut ClickPreprocessor {
        &mut self.click_preprocessor
    }

    pub(crate) fn did_close_inventory(&self) -> bool {
        self.did_close_inventory
    }

    pub(crate) fn set_did_close_inventory(&mut self, did_close_inventory: bool) {
        self.did_close_inventory = did_close_inventory;
    }

    pub fn held_slot(&self) -> i32 {
        self.held_slot
    }

    pub fn set_held_slot(&mut self, held_slot: i32) -> bool {
        if !(0..=8).contains(&held_slot) {
            return false;
        }
        self.held_slot = held_slot;
        true
    }

    pub fn item_in_hand(&self, hand: PlayerHand) -> ItemStack {
        let slot = match hand {
            PlayerHand::Main => self.held_slot as usize,
            PlayerHand::Off => crate::inventory::slot_conversion::OFFHAND_SLOT as usize,
        };
        self.inventory_ref()
            .item_stack(slot)
            .cloned()
            .unwrap_or_else(ItemStack::air)
    }

    pub fn set_item_in_hand(&mut self, hand: PlayerHand, item_stack: ItemStack) -> bool {
        let slot = match hand {
            PlayerHand::Main => self.held_slot as usize,
            PlayerHand::Off => crate::inventory::slot_conversion::OFFHAND_SLOT as usize,
        };
        self.inventory.set_item_stack(slot, item_stack)
    }

    pub(crate) fn swap_item_hands(
        &mut self,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let main_hand_item = self.item_in_hand(PlayerHand::Main);
        let off_hand_item = self.item_in_hand(PlayerHand::Off);
        let mut event =
            PlayerSwapItemEvent::new(self as *mut Player, off_hand_item, main_hand_item);
        event.dispatch(server, client);
        if event.is_cancelled() {
            return false;
        }
        let main_hand_item = event.main_hand_item().clone();
        let off_hand_item = event.off_hand_item().clone();
        self.set_item_in_hand(PlayerHand::Main, main_hand_item);
        self.set_item_in_hand(PlayerHand::Off, off_hand_item);
        let _ = self.sync_slot(self.held_slot, client);
        let _ = self.sync_slot(crate::inventory::slot_conversion::OFFHAND_SLOT, client);
        true
    }

    pub(crate) fn drop_main_hand_item(
        &mut self,
        all: bool,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let hand_item = self.item_in_hand(PlayerHand::Main);
        if hand_item.is_air() {
            return false;
        }
        let dropped_item = if all {
            hand_item.clone()
        } else {
            hand_item.with_amount(1)
        };
        if !self.drop_item(dropped_item, server, client) {
            let _ = self.sync_inventory(client);
            return false;
        }
        let updated_item = if all {
            ItemStack::air()
        } else {
            hand_item.consume(1)
        };
        self.set_item_in_hand(PlayerHand::Main, updated_item);
        self.sync_slot(self.held_slot, client).is_ok()
    }

    pub(crate) fn drop_item(
        &mut self,
        item_stack: ItemStack,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if item_stack.is_air() {
            return false;
        }
        let mut event = ItemDropEvent::new(self as *mut Player, item_stack);
        event.dispatch(server, client);
        !event.is_cancelled()
    }

    pub(crate) fn tick(&mut self) {}

    pub(super) fn send_tick_rate(
        &self,
        client: &mut Client,
        ticks_per_second: u32,
    ) -> io::Result<()> {
        TickingStatePacket {
            tick_rate: ticks_per_second as f32,
            is_frozen: false,
        }
        .dispatch(client)?;
        TickingStepPacket::new(0).dispatch(client)
    }

    pub(crate) fn finish_client_tick(&mut self, server_tick: u64) {
        self.last_completed_client_tick = server_tick;
    }

    pub(crate) fn look(&mut self, yaw: f32, pitch: f32) {
        self.position = self.position.looking_at(yaw, pitch);
    }
}

impl Taggable for Player {
    fn tag_handler(&self) -> &TagHandler {
        &self.tag_handler
    }

    fn tag_handler_mut(&mut self) -> &mut TagHandler {
        &mut self.tag_handler
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerHand {
    Main,
    Off,
}

#[cfg(test)]
mod tests {
    use super::{Player, PlayerHand};
    use crate::network::client::instance::Client;
    use crate::server::MinecraftServer;
    use spinel_core::entity::game_mode::GameMode;
    use spinel_registry::{ItemStack, Material};
    use std::net::TcpListener;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use uuid::Uuid;

    #[test]
    fn player_game_mode_defaults_to_survival_and_can_be_set_during_configuration() {
        let mut player = Player::new(
            Uuid::nil(),
            "Player".to_string(),
            0,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        );

        assert_eq!(player.game_mode(), GameMode::Survival);
        assert!(player.set_game_mode(GameMode::Creative));
        assert_eq!(player.game_mode(), GameMode::Creative);
    }

    #[test]
    fn swap_item_hands_matches_minestom_player_action_swap() {
        let mut player = Player::new(
            Uuid::nil(),
            "Player".to_string(),
            0,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
        );
        player.set_item_in_hand(PlayerHand::Main, ItemStack::of(Material::DIAMOND));
        player.set_item_in_hand(PlayerHand::Off, ItemStack::of(Material::EMERALD));
        let mut server = MinecraftServer::new();
        let mut client = test_client();

        assert!(player.swap_item_hands(&mut server, &mut client));
        assert_eq!(
            player.item_in_hand(PlayerHand::Main).material(),
            &Material::EMERALD
        );
        assert_eq!(
            player.item_in_hand(PlayerHand::Off).material(),
            &Material::DIAMOND
        );
    }

    fn test_client() -> Client {
        let listener =
            TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
        let addr = listener.local_addr().unwrap();
        let stream = std::net::TcpStream::connect(addr).unwrap();
        let _ = listener.accept().unwrap();
        Client::new(stream, addr)
    }
}
