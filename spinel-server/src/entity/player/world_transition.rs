use crate::entity::{EntityPosition, Player};
use crate::world::{PlayerWorldTransitionTicket, WorldHandle};
use std::io;

impl Player {
    pub fn set_world(
        &mut self,
        target_world: WorldHandle,
        target_position: EntityPosition,
    ) -> io::Result<PlayerWorldTransitionTicket> {
        let player_uuid = self.get_uuid();
        let Some(client) = self.get_client() else {
            return Err(io::ErrorKind::NotConnected.into());
        };
        let Some(server_pointer) = client.server_ptr else {
            return Err(io::ErrorKind::NotConnected.into());
        };
        let server = unsafe { &mut *(server_pointer as *mut crate::server::MinecraftServer) };
        server.world_manager.set_player_world_at_position_future(
            player_uuid,
            target_world.uuid(),
            target_position,
        )
    }
}
