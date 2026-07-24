# Enchantment registry definition and read model

## Scope

Owner: Minestom `net.minestom.server.item.enchant.Enchantment`, its concrete `EnchantmentImpl`, and the registry access path declared in `Registries` / implemented in `VanillaRegistries`. This document owns definition decoding and read access, not enchantment effect execution or ExampleServer command policy.

## Reviewed Source Coverage

- Minestom: `item/enchant/Enchantment.java`, `item/enchant/EnchantmentImpl.java`, `registry/Registries.java`, `registry/VanillaRegistries.java`.
- Spinel: `spinel-registry/src/registry_values/enchantment.rs`, `registry/collection/registries.rs`, `registry/collection/vanilla.rs`, `generated/vanilla_enchantments.rs`.
- Missing intended Spinel owner path: no separate owner is yet established for typed reading of generated enchantment registry NBT.

## Current Spinel State

`Enchantment` stores equivalent typed fields and can serialize them, but generated vanilla registration calls `Enchantment::raw`; raw values preserve NBT and leave all typed reader fields at defaults. `Registries` owns the dynamic registry but no public `enchantment()` reader was found.

## Dependency Classification

| Dependency | Type | Minestom evidence | Block | Task branch |
| --- | --- | --- | --- | --- |
| Extracted vanilla enchantment payload | Core | `Enchantment.createDefaultRegistry` loads vanilla resources | Yes | 1.1 |
| Tag, component, text, cost, and slot-group readers | Core | `REGISTRY_CODEC` reads each field | Yes | 1.2-1.6 |
| Typed effect-component reader | Core | `effects` is part of `REGISTRY_CODEC` | Yes | 1.7 |
| Public dynamic-registry access | Core | `Registries.enchantment()` | Yes | 2.1 |
| [Enchantment list value](../../../Common/Item/Component/EnchantmentList/Value.md) | Soft | keys identify entries stored on items | No | 3.1 |
| [Equipment-slot group](../../Entity/EquipmentSlotGroup/Group.md) | Cross | `slots()` returns groups | No | 3.2 |

## Actionable Task Tree

1. [ ] Decode the selected `Enchantment` registry-definition surface from extracted data.
   1.1 [ ] Preserve `description` component data from each vanilla entry.
   1.2 [ ] Preserve `exclusive_set` as an enchantment registry tag, including omitted-field default empty tag.
   1.3 [ ] Preserve required `supported_items` material tag.
   1.4 [ ] Preserve nullable `primary_items` without converting absent into an empty primary tag.
   1.5 [ ] Preserve `weight`, `max_level`, `min_cost`, `max_cost`, and `anvil_cost`.
   1.6 [ ] Preserve ordered `slots` group values.
   1.7 [ ] Preserve typed `effects` component map; stop on any effect-component branch not source-traced.
2. [ ] Expose the Minestom registry read operation on the direct Spinel registries owner.
   2.1 [ ] Provide `Registries::get_enchantments()` as the direct Rust getter for the dynamic enchantment registry.
   2.2 [ ] Retain dynamic registration ordering: effect helper registries before vanilla enchantment definitions.
3. [ ] Integrate dependent consumers without transferring registry ownership.
   3.1 [ ] Ensure [Enchantment list value](../../../Common/Item/Component/EnchantmentList/Value.md) keys resolve through the registry owner.
   3.2 [ ] Ensure `slots` consumes [Equipment-slot group](../../Entity/EquipmentSlotGroup/Group.md) values without a duplicate group model.

## Relevant Classes And Ownership Notes

`Enchantment` owns typed definition fields; `Registries` owns lookup. Generated registration remains a registry concern. Item stacks only own keyed levels; ExampleServer only consumes the completed capability.

## Implementation Strategy Against Agent.md And DesignDecisionRules.md

Keep typed entry decoding on the registry value and lookup on `Registries`. Do not introduce an ExampleServer registry adapter or raw-NBT reader.

## Dependency-Aware Implementation Order

1. Tasks 1.1-1.7.
2. Task 2.2.
3. Task 2.1.
4. Tasks 3.1-3.2.

## Verification Checklist

- [ ] `minecraft:sharpness` and another entry prove typed non-default scalar, tag, cost, slot, and effect values.
- [ ] Omitted `primary_items` and `exclusive_set` preserve their distinct source semantics.
- [ ] Public registry lookup returns the generated vanilla definition by key.
- [ ] Generated-data path uses SpinelExtractor output only.
- [ ] No ExampleServer owner parses raw enchantment registry NBT.

## Public API Coverage

| Minestom declaration | Current / intended Spinel owner | Mapping status | Required proof |
| --- | --- | --- | --- |
| `Enchantment.NETWORK_TYPE`, `CODEC`, `REGISTRY_CODEC` | `spinel_registry::Enchantment` | Map to the typed registry-key/network/data codec boundary; current raw registration lacks typed decode/read linkage | codec and real-entry tests |
| `static builder()` | `Enchantment::builder()` | Present construction surface; raw registration bypasses it | builder/default test |
| `description`, `exclusiveSet`, `supportedItems`, `primaryItems`, `weight`, `maxLevel`, `minCost`, `maxCost`, `anvilCost`, `slots`, `effects` | `Enchantment` getters | Structurally present; vanilla runtime values missing | real-entry reader test per field |
| nested `Target`, `Effect`, `Cost` | `EnchantmentTarget`, effect types, `EnchantmentCost` | Cost maps directly; target/effect decode linkage remains implementation work | decode each selected branch |
| `Builder` public setters and `build` | `EnchantmentBuilder` | Setter shape resolved below | builder and data tests |
| `Registries.enchantment()` / `VanillaRegistries.enchantment()` | `Registries::get_enchantments()` | Missing implementation | lookup test |

### Required side-by-side mappings

```java
// Minestom: net.minestom.server.registry.Registries
DynamicRegistry<Enchantment> enchantment();
```

```rust
// Spinel: spinel_registry::Registries
pub fn get_enchantments(&self) -> &DynamicRegistry<Enchantment>
```

```java
// Minestom: net.minestom.server.item.enchant.Enchantment
int maxLevel();
```

```rust
// Spinel: spinel_registry::Enchantment
pub const fn get_max_level(&self) -> i32
```

Getter naming is a permitted Rust convention; actual vanilla registered value parity remains unfinished.

```java
public <T> Builder effect(DataComponent<T> component, T value)
public Builder effects(DataComponentMap effects)
```

```rust
// Spinel: spinel_registry::EnchantmentBuilder
pub fn set_effect<T: DataComponentValue>(self, component: DataComponentType<T>, value: T) -> Self
pub fn set_effects(self, effects: DataComponentMap) -> Self
```

The two Java effect inputs remain two builder setters. Rust generic bounds express the component/value relation without a caller-visible capability loss.

## Edge Behavior Coverage

| Source behavior | Spinel state | Proof |
| --- | --- | --- |
| `exclusive_set` omitted defaults to empty tag | raw NBT currently only | unit |
| `primary_items` is nullable | getter can be `None`, raw path unproven | unit |
| helper registries initialize before enchantments | generated registration ordering differs in representation | integration |
| `slots` preserves list order | typed raw decode absent | unit |
| effect map defaults empty when omitted | typed raw decode absent | unit |

## Completion Gate

Unfinished until all task leaves, typed field reads, registry lookup, generated-data wiring, and reciprocal dependency integrations are implemented and verified.
