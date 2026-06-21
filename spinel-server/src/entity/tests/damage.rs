use crate::entity::{Damage, EntityId};
use spinel_network::types::Vector3d;
use spinel_registry::damage_type::DamageType;

#[test]
fn damage_amount_matches_minestom_without_clamping() {
    let mut damage = Damage::new(DamageType::GENERIC, -2.0);

    assert_eq!(damage.get_amount(), -2.0);

    damage.set_amount(-4.0);

    assert_eq!(damage.get_amount(), -4.0);
}

#[test]
fn entity_damage_factories_preserve_direct_source_and_attacker() {
    let source = EntityId::from_raw(12);
    let from_entity = Damage::from_entity(source, 3.0);
    let from_player = Damage::from_player(source, 4.0);

    assert_eq!(from_entity.damage_type(), &DamageType::MOB_ATTACK);
    assert_eq!(from_entity.get_source(), source);
    assert_eq!(from_entity.get_attacker(), source);
    assert_eq!(from_player.get_source(), source);

    let damage: Damage = from_entity.into();
    assert_eq!(damage.get_source(), Some(source));
    assert_eq!(damage.get_attacker(), Some(source));
}

#[test]
fn projectile_and_positional_damage_factories_preserve_context() {
    let shooter = EntityId::from_raw(4);
    let projectile = EntityId::from_raw(8);
    let projectile_damage = Damage::from_projectile(Some(shooter), projectile, 5.0);

    assert_eq!(projectile_damage.damage_type(), &DamageType::MOB_PROJECTILE);
    assert_eq!(projectile_damage.get_projectile(), projectile);
    assert_eq!(projectile_damage.get_shooter(), Some(shooter));

    let position = Vector3d {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let positional_damage = Damage::from_position(DamageType::EXPLOSION, position, 6.0);

    assert_eq!(positional_damage.damage_type(), &DamageType::EXPLOSION);
    assert_eq!(positional_damage.get_source_position(), Some(position));
    assert_eq!(positional_damage.get_source(), None);
    assert_eq!(positional_damage.get_attacker(), None);
}
