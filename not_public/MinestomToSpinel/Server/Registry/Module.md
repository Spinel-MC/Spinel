# Registry Module

## Module Boundary

Owns the selected Minestom `net.minestom.server.registry` enchantment-registry entry point. It does not own item component values, equipment slots, entity behavior, or ExampleServer policy.

## Reviewed Source Coverage

- Minestom: `src/main/java/net/minestom/server/registry/Registries.java`, `VanillaRegistries.java`.
- Spinel: `spinel-registry/src/registry/collection/registries.rs`, `vanilla.rs`, `generated/vanilla_enchantments.rs`.

## Owned Documents

- [Enchantment definition and read model](Enchantment.md)

## Dependency Documents

- [Enchantment list value](../../../Common/Item/Component/EnchantmentList/Value.md)
- [Equipment-slot group](../../Entity/EquipmentSlotGroup/Group.md)

## Surrounding Modules Or Domains That Block Completion

Typed effect component decoding and the extractor's emitted enchantment payload schema are core dependencies recorded in `Enchantment.md`.

## Completion Order

1. Complete `Enchantment.md` typed read and registry access work.
2. Integrate the item-component and equipment-slot-group dependency documents.

## Current Module State

Unfinished. Vanilla registrations preserve raw NBT and `Registries::get_enchantments()` is documented as the required missing public reader.

## Module-Level Orchestration Task Tree

1. [ ] Complete [Enchantment definition and read model](Enchantment.md).
2. [ ] Complete reciprocal integration with [Enchantment list value](../../../Common/Item/Component/EnchantmentList/Value.md).
3. [ ] Complete reciprocal integration with [Equipment-slot group](../../Entity/EquipmentSlotGroup/Group.md).

## Verification Gate

The module cannot complete before `Enchantment.md` proves real extracted vanilla entries are typed-readable through the public registry owner.
