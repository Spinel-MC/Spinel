use crate::entity::{Entity, EntityId, PlayerHand};
use crate::events::entity_attack::EntityAttackEvent;
use crate::events::player_entity_interact::PlayerEntityInteractEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::interact::{InteractAction, InteractPacket};
use spinel_macros::packet_listener;

#[packet_listener]
fn on_interact(client: &mut Client, packet: InteractPacket, server: &mut MinecraftServer) -> bool {
    let Some(interaction) = interaction_input(client, packet.entity_id, server) else {
        return true;
    };
    match packet.action {
        InteractAction::Attack => {
            if interaction.target_is_dead {
                return true;
            }
            EntityAttackEvent::new(interaction.player_id, interaction.target_id)
                .dispatch(server, client);
            true
        }
        InteractAction::InteractAt {
            target_x,
            target_y,
            target_z,
            hand,
        } => {
            let Some(hand) = PlayerHand::from_protocol_id(hand) else {
                return false;
            };
            PlayerEntityInteractEvent::new(
                interaction.player,
                interaction.target_id,
                hand,
                (target_x, target_y, target_z),
            )
            .dispatch(server, client);
            true
        }
        InteractAction::Interact { .. } => true,
    }
}

struct InteractionInput {
    player: *mut crate::entity::Player,
    player_id: EntityId,
    target_id: EntityId,
    target_is_dead: bool,
}

fn interaction_input(
    client: &Client,
    target_entity_id: i32,
    server: &mut MinecraftServer,
) -> Option<InteractionInput> {
    let player = server.world_manager.player_pointer_for_client(client)?;
    let player_id = unsafe { &*player }.entity_id();
    let target_id = EntityId::from_raw(target_entity_id);
    let world_uuid = server.world_manager.world_uuid_for_client(client)?;
    let world = server.world_manager.world(world_uuid)?;
    let target = world.entity_by_id(target_id)?;
    if target.entity_id() == player_id {
        return None;
    }
    if !target_is_viewable_by_player(target, player_id) {
        return None;
    }
    Some(InteractionInput {
        player,
        player_id,
        target_id,
        target_is_dead: target_is_dead(target),
    })
}

fn target_is_viewable_by_player(target: &Entity, player_id: EntityId) -> bool {
    match target {
        Entity::Generic(entity) => entity.is_viewer(player_id),
        Entity::Item(entity) => entity.is_viewer(player_id),
        Entity::Player(_) => true,
    }
}

fn target_is_dead(target: &Entity) -> bool {
    match target {
        Entity::Generic(entity) => entity.is_dead(),
        Entity::Item(_) => false,
        Entity::Player(_) => false,
    }
}
