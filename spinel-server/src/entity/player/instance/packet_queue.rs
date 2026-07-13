use crate::network::client::instance::Client;
use spinel_network::ConnectionState;
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::text::TextComponent;

use super::state::Player;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueuedPlayerPacket {
    pub state: ConnectionState,
    pub packet_id: i32,
    pub payload: Vec<u8>,
}

impl QueuedPlayerPacket {
    pub fn new(state: ConnectionState, packet_id: i32, payload: Vec<u8>) -> Self {
        Self {
            state,
            packet_id,
            payload,
        }
    }
}

impl Player {
    pub fn add_packet_to_queue(&mut self, packet: QueuedPlayerPacket) -> bool {
        if self.packet_queue.len() >= Self::PLAYER_PACKET_QUEUE_SIZE {
            let _ = self.kick(Self::too_many_packets_text());
            return false;
        }
        self.packet_queue.push_back(packet);
        true
    }

    pub(crate) fn pop_next_packet_from_queue(&mut self) -> Option<QueuedPlayerPacket> {
        self.packet_queue.pop_front()
    }

    pub fn interpret_packet_queue(
        &mut self,
        server: &mut crate::server::MinecraftServer,
        client: &mut Client,
    ) -> usize {
        let mut interpreted_packets = 0;
        while interpreted_packets < Self::PLAYER_PACKET_PER_TICK {
            let Some(queued_packet) = self.packet_queue.pop_front() else {
                return interpreted_packets;
            };
            client.state = queued_packet.state;
            server.dispatch_packet(queued_packet.packet_id, client, queued_packet.payload);
            interpreted_packets += 1;
        }
        interpreted_packets
    }

    pub fn get_queued_packet_count(&self) -> usize {
        self.packet_queue.len()
    }

    pub(super) fn too_many_packets_text() -> TextComponent {
        TextComponent::literal_with_color(
            "Too Many Packets",
            TextColor::from_named(NamedTextColor::Red),
        )
    }
}
