use crate::entity::{Damage, EntityId};
use crate::events::entity_damage::EntityDamageEvent;
use crate::events::entity_death::EntityDeathEvent;
use crate::events::entity_fire_extinguish::EntityFireExtinguishEvent;
use crate::events::entity_set_fire::EntitySetFireEvent;
use crate::events::pickup_item::PickupItemEvent;
use spinel_registry::damage_type::DamageType;

#[test]
fn living_event_surface_matches_minestom_cancellation_shape() {
    let living_entity_id = EntityId::from_raw(1);
    let item_entity_id = EntityId::from_raw(2);
    let mut set_fire = EntitySetFireEvent::new(living_entity_id, 20);
    set_fire.set_fire_ticks(-5);
    set_fire.set_cancelled(true);

    let mut extinguish = EntityFireExtinguishEvent::new(living_entity_id, true);
    extinguish.set_cancelled(true);

    let mut damage = EntityDamageEvent::new(
        living_entity_id,
        Damage::new(DamageType::GENERIC, 5.0).without_animation(),
    );
    damage.set_damage(-10.0);
    damage.set_should_animate(true);
    damage.set_cancelled(true);

    let death = EntityDeathEvent::new(living_entity_id);

    let mut pickup = PickupItemEvent::new(living_entity_id, item_entity_id);
    pickup.set_cancelled(true);

    assert_eq!(set_fire.entity_id().value(), 1);
    assert_eq!(set_fire.fire_ticks(), 0);
    assert!(set_fire.is_cancelled());
    assert!(extinguish.is_natural());
    assert!(extinguish.is_cancelled());
    assert_eq!(damage.damage_source(), "generic");
    assert_eq!(damage.damage().amount(), 0.0);
    assert!(damage.should_animate());
    assert!(damage.is_cancelled());
    assert_eq!(death.entity_id().value(), 1);
    assert_eq!(pickup.living_entity_id().value(), 1);
    assert_eq!(pickup.item_entity_id().value(), 2);
    assert!(pickup.is_cancelled());
}
