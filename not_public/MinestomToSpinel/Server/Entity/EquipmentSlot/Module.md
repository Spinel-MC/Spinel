# EquipmentSlot Module

## Module Boundary

Owns Minestom `net.minestom.server.entity.EquipmentSlot`; it does not own living storage or handler behavior.

## Reviewed Source Coverage

- Minestom: `src/main/java/net/minestom/server/entity/EquipmentSlot.java`.
- Spinel: `spinel-server/src/entity/equipment_slot.rs`.

## Owned Documents

- [Equipment slot](Slot.md)

## Dependency Documents

- [Living entity equipment](../LivingEntity/Equipment.md)
- [Equipment handler contract](../../Inventory/EquipmentHandler/Contract.md)

## Surrounding Modules Or Domains That Block Completion

Network and data codec ownership must be established for this semantic enum.

## Completion Order

1. Complete `Slot.md`.
2. Integrate the living and handler documents.

## Current Module State

Unfinished. Values and basic accessors exist; codecs, collection, and invalid legacy conversion are unproven.

## Module-Level Orchestration Task Tree

1. [ ] Complete [Equipment slot](Slot.md).
2. [ ] Complete reciprocal integration with [Living entity equipment](../LivingEntity/Equipment.md).
3. [ ] Complete reciprocal integration with [Equipment handler contract](../../Inventory/EquipmentHandler/Contract.md).

## Verification Gate

All enum variants and selected codec/error branches require proof.
