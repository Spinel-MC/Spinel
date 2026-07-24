# LivingEntity Module

## Module Boundary

Owns the selected Minestom `LivingEntity` equipment and hand behavior. Equipment-slot, slot-group, and interface-contract documents remain separate owners.

## Reviewed Source Coverage

- Minestom: `src/main/java/net/minestom/server/entity/LivingEntity.java`.
- Spinel: `spinel-server/src/entity/living/state.rs`, `living/equipment.rs`, `generic_entity.rs`, `entity.rs`, `entity_creature.rs`, `entity/player/instance/inventory_state.rs`, `entity/player/item_use.rs`.

## Owned Documents

- [Living entity equipment](Equipment.md)

## Dependency Documents

- [Equipment slot](../EquipmentSlot/Slot.md)
- [Equipment-slot group](../EquipmentSlotGroup/Group.md)
- [Equipment handler contract](../../Inventory/EquipmentHandler/Contract.md)

## Surrounding Modules Or Domains That Block Completion

Viewer dispatch, equip events, attributes, and player held-slot inventory require integration under the direct living owner.

## Completion Order

1. Complete slot and handler contract dependencies.
2. Complete `Equipment.md` owner and behavior work.
3. Integrate player held-slot and nonplayer viewer paths.

## Current Module State

Unfinished. State and packets exist, but ownership is widened to `GenericEntity` and the common living contract is absent.

## Module-Level Orchestration Task Tree

1. [ ] Complete [Equipment slot](../EquipmentSlot/Slot.md).
2. [ ] Complete [Equipment-slot group](../EquipmentSlotGroup/Group.md).
3. [ ] Complete [Equipment handler contract](../../Inventory/EquipmentHandler/Contract.md).
4. [ ] Complete [Living entity equipment](Equipment.md).

## Verification Gate

Completion requires public receiver-boundary, event, attribute, viewer-packet, and player-inventory integration proof.
