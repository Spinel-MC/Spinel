# EquipmentHandler Module

## Module Boundary

Owns Minestom `net.minestom.server.inventory.EquipmentHandler` shared contract. `LivingEntity` owns concrete behavior; `EquipmentSlot` owns enum semantics.

## Reviewed Source Coverage

- Minestom: `src/main/java/net/minestom/server/inventory/EquipmentHandler.java`, `entity/LivingEntity.java`.
- Spinel: `spinel-server/src/entity/living/state.rs`, `living/equipment.rs`, `generic_entity.rs`, `player/instance/inventory_state.rs`.

## Owned Documents

- [Equipment handler contract](Contract.md)

## Dependency Documents

- [Living entity equipment](../../Entity/LivingEntity/Equipment.md)
- [Equipment slot](../../Entity/EquipmentSlot/Slot.md)

## Surrounding Modules Or Domains That Block Completion

The shared Java interface maps to a narrow `EquipmentHandler` Rust trait; Entity-only packet operations use named Result errors.

## Completion Order

1. Complete `Contract.md`.
2. Integrate one contract into the direct living owner.

## Current Module State

Unfinished. Operations are split across state, generic entity, and player inventory rather than one shared public contract.

## Module-Level Orchestration Task Tree

1. [ ] Complete [Equipment handler contract](Contract.md).
2. [ ] Complete reciprocal integration with [Living entity equipment](../../Entity/LivingEntity/Equipment.md).
3. [ ] Complete reciprocal integration with [Equipment slot](../../Entity/EquipmentSlot/Slot.md).

## Verification Gate

Every inherited default operation must be exposed on each intended living receiver and packet behavior must retain Entity-only restriction.
