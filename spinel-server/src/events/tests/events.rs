use crate::entity::player::PlayerSkin;
use crate::entity::{Damage, Entity, EntityId, GenericEntity, Player};
use crate::events::entity_damage::EntityDamageEvent;
use crate::events::entity_death::EntityDeathEvent;
use crate::events::entity_fire_extinguish::EntityFireExtinguishEvent;
use crate::events::entity_set_fire::EntitySetFireEvent;
use crate::events::login::AsyncPlayerPreLoginEvent;
use crate::events::outgoing_transfer::OutgoingTransferEvent;
use crate::events::pickup_item::PickupItemEvent;
use crate::events::player_command::PlayerCommandEvent;
use crate::events::player_configuration::AsyncPlayerConfigurationEvent;
use crate::events::player_packet::PlayerPacketEvent;
use crate::events::player_packet_out::PlayerPacketOutEvent;
use crate::events::player_skin_init::PlayerSkinInitEvent;
use spinel_network::types::Identifier;
use spinel_network::types::game_profile::{GameProfile, GameProfileProperty};
use spinel_registry::EntityType;
use spinel_registry::damage_type::DamageType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn living_event_surface_matches_minestom_cancellation_shape() {
    let living_entity_id = EntityId::from_raw(1);
    let item_entity_id = EntityId::from_raw(2);
    let mut entity = Entity::Generic(GenericEntity::new(EntityType::ZOMBIE));
    let entity_id = entity.entity_id();
    let entity = &mut entity as *mut Entity;
    let mut set_fire = EntitySetFireEvent::new(entity, 20);
    set_fire.set_fire_ticks(-5);
    set_fire.set_cancelled(true);

    let mut extinguish = EntityFireExtinguishEvent::new(entity, true);
    extinguish.set_cancelled(true);

    let mut damage = EntityDamageEvent::new(
        entity,
        Damage::new(DamageType::GENERIC, 5.0).without_animation(),
    );
    damage.set_damage(-10.0);
    damage.set_should_animate(true);
    damage.set_cancelled(true);

    let death = EntityDeathEvent::new(entity);

    let mut pickup = PickupItemEvent::new(living_entity_id, item_entity_id);
    pickup.set_cancelled(true);

    assert_eq!(set_fire.entity_id(), entity_id);
    assert_eq!(set_fire.fire_ticks(), 0);
    assert!(set_fire.is_cancelled());
    assert!(extinguish.is_natural());
    assert!(extinguish.is_cancelled());
    assert_eq!(damage.damage_source(), "generic");
    assert_eq!(damage.damage().amount(), -10.0);
    assert!(damage.should_animate());
    assert!(damage.is_cancelled());
    assert_eq!(death.entity_id(), entity_id);
    assert_eq!(pickup.living_entity_id().value(), 1);
    assert_eq!(pickup.item_entity_id().value(), 2);
    assert!(pickup.is_cancelled());
}

#[test]
fn player_connection_events_expose_minestom_mutation_and_cancellation_surface() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let player_pointer = &mut player as *mut Player;
    let mut command = PlayerCommandEvent::new(player_pointer, "first");
    command.set_command("second");
    command.set_cancelled(true);
    let mut transfer = OutgoingTransferEvent::new(player_pointer, "first.example", 25565);
    transfer.set_host("second.example");
    transfer.set_port(25566);
    transfer.set_cancelled(true);
    let mut inbound = PlayerPacketEvent::new(player_pointer, 1, "packet", vec![1, 2]);
    inbound.set_cancelled(true);
    let mut outbound = PlayerPacketOutEvent::new(player_pointer, 2, "packet_out", 3);
    outbound.set_cancelled(true);
    let skin = PlayerSkin::new("texture", Some("signature".to_string()));
    let mut skin_init = PlayerSkinInitEvent::new(player_pointer, None);
    skin_init.set_skin(Some(skin.clone()));

    assert_eq!(command.command(), "second");
    assert!(command.is_cancelled());
    assert_eq!(transfer.host(), "second.example");
    assert_eq!(transfer.port(), 25566);
    assert!(transfer.is_cancelled());
    assert_eq!(inbound.payload(), [1, 2]);
    assert!(inbound.is_cancelled());
    assert_eq!(outbound.payload_size(), 3);
    assert!(outbound.is_cancelled());
    assert_eq!(skin_init.skin(), Some(&skin));
}

#[test]
fn login_and_configuration_events_preserve_profile_and_feature_choices() {
    let profile = GameProfile {
        uuid: Uuid::nil(),
        username: "ProfileName".to_string(),
        properties: vec![GameProfileProperty {
            name: "textures".to_string(),
            value: "texture".to_string(),
            signature: None,
        }],
    };
    let mut pre_login = AsyncPlayerPreLoginEvent::new("Initial".to_string(), Uuid::max(), false);
    pre_login.set_game_profile(profile.clone());
    let player = Player::new(
        profile.uuid,
        profile.username.clone(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let mut configuration = AsyncPlayerConfigurationEvent::new(player, true);
    let vanilla = Identifier::minecraft("vanilla");
    let trade_rebalance = Identifier::minecraft("trade_rebalance");
    configuration.add_feature_flag(trade_rebalance.clone());
    configuration.set_should_clear_chat(true);
    configuration.set_should_send_registry_data(false);
    configuration.set_spawning_world(None);

    assert_eq!(pre_login.game_profile(), &profile);
    assert!(configuration.feature_flags().contains(&vanilla));
    assert!(configuration.feature_flags().contains(&trade_rebalance));
    assert!(configuration.remove_feature_flag(&trade_rebalance));
    assert!(configuration.should_clear_chat());
    assert!(!configuration.should_send_registry_data());
    assert_eq!(configuration.spawning_world(), None);
}
