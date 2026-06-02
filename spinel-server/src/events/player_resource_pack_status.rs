use crate::entity::Player;
use spinel_core::network::resource_pack::ResourcePackStatus;
use spinel_macros::event_dispatcher;
use uuid::Uuid;

#[event_dispatcher(with_client: true)]
pub struct PlayerResourcePackStatusEvent {
    player: *mut Player,
    pack_id: Uuid,
    status: ResourcePackStatus,
}

impl PlayerResourcePackStatusEvent {
    pub fn new(player: *mut Player, pack_id: Uuid, status: ResourcePackStatus) -> Self {
        Self {
            player,
            pack_id,
            status,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub const fn pack_id(&self) -> Uuid {
        self.pack_id
    }

    pub const fn status(&self) -> ResourcePackStatus {
        self.status
    }
}
