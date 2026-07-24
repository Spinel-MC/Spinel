# Equipment-slot group

## Scope

Owner: Minestom `EquipmentSlotGroup` enum. It owns group variants `ANY`, `MAIN_HAND`, `OFF_HAND`, `HAND`, `FEET`, `LEGS`, `CHEST`, `HEAD`, `ARMOR`, `BODY`, and `SADDLE`, plus membership/predicate and serialization behavior.

## Reviewed Source Coverage

- Minestom: `entity/EquipmentSlotGroup.java`.
- Spinel: `spinel-registry/src/data_components/attribute_list.rs`, `registry_values/enchantment.rs`, `spinel-server/src/entity/living/attributes.rs` and `living/state.rs`.

## Current Spinel State

`spinel_registry::EquipmentSlotGroup` exists for attribute data. This pass has not established a complete public codec, NBT-name, membership-list, `contains`, or predicate counterpart for the selected Minestom surface.

## Dependency Classification

| Dependency | Type | Minestom evidence | Block | Task branch |
| --- | --- | --- | --- | --- |
| [Equipment slot](../EquipmentSlot/Slot.md) | Core | groups contain slots | Yes | 1 |
| [Enchantment registry definition and read model](../../Registry/Enchantment.md) | Cross | enchantment `slots` uses groups | No | 2.1 |
| [Living entity equipment](../LivingEntity/Equipment.md) | Cross | modifier application calls membership semantics | No | 2.2 |

## Actionable Task Tree

1. [ ] Preserve the complete group contract.
   1.1 [ ] Verify each named group has the source slot membership and ordering.
   1.2 [ ] Map `equipmentSlots()` immutable list capability.
   1.3 [ ] Map `nbtName()`, `NETWORK_TYPE`, and string `CODEC`.
   1.4 [ ] Map `contains(EquipmentSlot)` and `Predicate.test(EquipmentSlot)` as one shared behavior.
2. [ ] Integrate the single group owner.
   2.1 [ ] Route enchantment `slots` through this owner.
   2.2 [ ] Route equipment attribute filtering through this owner.

## Relevant Classes And Ownership Notes

Group semantics belong to the entity equipment domain even when used by registry components. The registry and living systems must consume one group type, not recreate membership switches.

## Implementation Strategy Against Agent.md And DesignDecisionRules.md

Retain a fixed semantic enum and expose named `contains` behavior on the group owner. Rust has no need to duplicate Java `Predicate.test`; map it through `contains` as the same capability.

## Dependency-Aware Implementation Order

1. 1.1.
2. 1.2-1.4.
3. 2.1-2.2.

## Verification Checklist

- [ ] membership and order for all eleven groups;
- [ ] NBT-name and protocol codec round trips;
- [ ] `contains` and predicate parity;
- [ ] enchantment slot and attribute-modifier integration.

## Public API Coverage

| Minestom declaration | Spinel owner | Mapping status | Proof |
| --- | --- | --- | --- |
| eleven enum variants | `spinel_registry::EquipmentSlotGroup` | Present type; exact variants unverified | unit |
| `NETWORK_TYPE`, `CODEC` | no source-proven equivalent | Missing | codec |
| `equipmentSlots`, `nbtName` | unverified | Unresolved | unit |
| `contains`, `test` | `EquipmentSlotGroup::contains` | `test` maps to the same named Rust predicate capability | unit |

### Required side-by-side mappings

```java
public boolean contains(EquipmentSlot equipmentSlot)
@Override public boolean test(EquipmentSlot equipmentSlot)
```

```rust
pub fn contains(&self, equipment_slot: EquipmentSlot) -> bool
```

## Edge Behavior Coverage

| Source behavior | Spinel state | Proof |
| --- | --- | --- |
| `ANY` contains every EquipmentSlot value | unverified | unit |
| `HAND` contains exactly main/off | unverified | unit |
| `ARMOR` ordering is chest, legs, boots, helmet | unverified | unit |
| string decoding uses named group map | unverified | codec |

## Completion Gate

Unfinished until every variant, list, codec, and predicate branch is mapped, implemented, and verified by both registry and living consumers.
