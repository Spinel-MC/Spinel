# EnchantmentList Module

## Module Boundary

Owns the Minestom `net.minestom.server.item.component.EnchantmentList` value surface. Registry definition lookup remains owned by [Server Registry Enchantment](../../../../Server/Registry/Enchantment.md).

## Reviewed Source Coverage

- Minestom: `src/main/java/net/minestom/server/item/component/EnchantmentList.java`.
- Spinel: `spinel-registry/src/data_components/enchantment_list.rs`.

## Owned Documents

- [Enchantment list value](Value.md)

## Dependency Documents

- [Enchantment registry definition and read model](../../../../Server/Registry/Enchantment.md)

## Surrounding Modules Or Domains That Block Completion

Protocol network-buffer codec support is not yet established for this value.

## Completion Order

1. Complete `Value.md` immutable value and codec parity.
2. Integrate registry-key resolution with the registry document.

## Current Module State

Unfinished. Item component NBT behavior exists; the selected Minestom network codec and bound are unproven.

## Module-Level Orchestration Task Tree

1. [ ] Complete [Enchantment list value](Value.md).
2. [ ] Complete reciprocal integration with [Enchantment registry definition and read model](../../../../Server/Registry/Enchantment.md).

## Verification Gate

Completion requires value semantics and both selected serialization forms.
