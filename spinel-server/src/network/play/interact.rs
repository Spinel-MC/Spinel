use crate::entity::{Entity, EntityId, PlayerHand};
use crate::events::entity_attack::EntityAttackEvent;
use crate::events::player_entity_interact::PlayerEntityInteractEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::interact::{InteractAction, InteractPacket};
use spinel_macros::packet_listener;
use spinel_registry::Attribute;

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
            EntityAttackEvent::new(interaction.player_id, interaction.target_id).dispatch(server);
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
                interaction.target,
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
    target: *mut Entity,
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
    let world = server.world_manager.world_mut(world_uuid)?;
    let target = world.entity_by_id_mut(target_id)?;
    if target.entity_id() == player_id {
        return None;
    }
    if !target_is_viewable_by_player(target, player_id) {
        return None;
    }
    if server.enforce_entity_interaction_range
        && !target_is_within_interaction_range(unsafe { &*player }, target)
    {
        return None;
    }
    Some(InteractionInput {
        player,
        player_id,
        target_id,
        target,
        target_is_dead: target_is_dead(target),
    })
}

fn target_is_within_interaction_range(player: &crate::entity::Player, target: &Entity) -> bool {
    let player_position = player.position();
    let target_position = target.position();
    let target_bounding_box = target.bounding_box();
    let maximum_distance = player.attribute_value(Attribute::ENTITY_INTERACTION_RANGE) + 1.0;
    let minimum_x = target_position.x() + target_bounding_box.minimum_x();
    let maximum_x = target_position.x() + target_bounding_box.maximum_x();
    let minimum_y = target_position.y() + target_bounding_box.minimum_y();
    let maximum_y = target_position.y() + target_bounding_box.maximum_y();
    let minimum_z = target_position.z() + target_bounding_box.minimum_z();
    let maximum_z = target_position.z() + target_bounding_box.maximum_z();
    let player_eye_x = player_position.x();
    let player_eye_y = player_position.y() + player.eye_height();
    let player_eye_z = player_position.z();
    let distance_x = player_eye_x - player_eye_x.clamp(minimum_x, maximum_x);
    let distance_y = player_eye_y - player_eye_y.clamp(minimum_y, maximum_y);
    let distance_z = player_eye_z - player_eye_z.clamp(minimum_z, maximum_z);
    let distance_squared =
        distance_x * distance_x + distance_y * distance_y + distance_z * distance_z;

    distance_squared <= maximum_distance * maximum_distance
}

fn target_is_viewable_by_player(target: &Entity, player_id: EntityId) -> bool {
    match target {
        Entity::Creature(entity) => entity.is_viewer(player_id),
        Entity::ExperienceOrb(entity) => entity.is_viewer(player_id),
        Entity::Generic(entity) => entity.is_viewer(player_id),
        Entity::Item(entity) => entity.is_viewer(player_id),
        Entity::Player(_) => true,
        Entity::Projectile(entity) => entity.is_viewer(player_id),
    }
}

fn target_is_dead(target: &Entity) -> bool {
    match target {
        Entity::Creature(entity) => entity.is_dead(),
        Entity::ExperienceOrb(entity) => entity.is_dead(),
        Entity::Generic(entity) => entity.is_dead(),
        Entity::Item(_) => false,
        Entity::Player(_) => false,
        Entity::Projectile(_) => false,
    }
}
