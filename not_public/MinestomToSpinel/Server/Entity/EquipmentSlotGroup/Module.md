# EquipmentSlotGroup Module

## Module Boundary

Owns Minestom `net.minestom.server.entity.EquipmentSlotGroup`, including named group membership and predicate/codec behavior. It is consumed by enchantment definitions and equipment attribute handling.

## Reviewed Source Coverage

- Minestom: `src/main/java/net/minestom/server/entity/EquipmentSlotGroup.java`.
- Spinel: `spinel-registry/src/data_components/attribute_list.rs`; dependent `spinel-registry/src/registry_values/enchantment.rs`, `spinel-server/src/entity/living/state.rs`.

## Owned Documents

- [Equipment-slot group](Group.md)

## Dependency Documents

- [Enchantment registry definition and read model](../../Registry/Enchantment.md)
- [Living entity equipment](../LivingEntity/Equipment.md)
- [Equipment slot](../EquipmentSlot/Slot.md)

## Surrounding Modules Or Domains That Block Completion

The exact current Spinel codec and all public group operations require source-to-source mapping before implementation.

## Completion Order

1. Complete `Group.md` membership and codec contract.
2. Integrate one group model with registry and living-attribute consumers.

## Current Module State

Unfinished. A Spinel group enum exists in data components, but full selected Minestom surface has not been established as present.

## Module-Level Orchestration Task Tree

1. [ ] Complete [Equipment-slot group](Group.md).
2. [ ] Complete reciprocal integration with [Enchantment registry definition and read model](../../Registry/Enchantment.md).
3. [ ] Complete reciprocal integration with [Living entity equipment](../LivingEntity/Equipment.md).

## Verification Gate

Completion requires group membership, predicate, and both codec paths for every variant.
