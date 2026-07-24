# Enchantment list value

## Scope

Owner: Minestom `net.minestom.server.item.component.EnchantmentList`. This value owns enchantment keys and integer levels, not registry definition lookup.

## Reviewed Source Coverage

- Minestom: `item/component/EnchantmentList.java`.
- Spinel: `spinel-registry/src/data_components/enchantment_list.rs`; dependency `spinel-registry/src/registry_values/enchantment.rs`.

## Current Spinel State

Spinel has an immutable-by-return `HashMap<RegistryKey<Enchantment>, i32>` value with `has`, `level`, `with`, and `remove`, plus component NBT conversion. No selected Minestom network codec or `Short.MAX_VALUE` collection bound was found.

## Dependency Classification

| Dependency | Type | Minestom evidence | Block | Task branch |
| --- | --- | --- | --- | --- |
| [Enchantment registry definition and read model](../../../../Server/Registry/Enchantment.md) | Soft | map keys are `RegistryKey<Enchantment>` | No | 3.1 |
| network buffer collection codecs | Core | `NETWORK_TYPE` | Yes | 1.2 |

## Actionable Task Tree

1. [ ] Preserve the selected data contract.
   1.1 [ ] Preserve immutable copied map construction and `EMPTY` semantics.
   1.2 [ ] Implement or explicitly map `NETWORK_TYPE`: registry key plus VAR_INT level with `Short.MAX_VALUE` entry bound.
   1.3 [ ] Implement or explicitly map `CODEC`: registry key plus integer level with the same entry bound.
2. [ ] Preserve immutable operations.
   2.1 [ ] Map single-key constructor.
   2.2 [ ] Verify `has`, absent `level == 0`, replacement `with`, and `remove` each return/source values correctly.
3. [ ] Integrate keys with [Enchantment registry definition and read model](../../../../Server/Registry/Enchantment.md) without duplicating definition reads.

## Relevant Classes And Ownership Notes

The item component owns levels. `Registries` owns resolution to definition data. No entity or ExampleServer owner should absorb either responsibility.

## Implementation Strategy Against Agent.md And DesignDecisionRules.md

Keep a single immutable value type. Map the Java single-enchantment constructor to `new`; use getter naming for the record reader.

## Dependency-Aware Implementation Order

1. 1.1 and 2.2.
2. 1.2 and 1.3.
3. 2.1 and 3.1.

## Verification Checklist

- [ ] Empty, singleton, multiple, replacement, remove, and absent-level paths.
- [ ] NBT component round trip.
- [ ] Network and data codec round trips, including maximum and oversized entry count.
- [ ] Registry keys resolve only through the registry dependency.

## Public API Coverage

| Minestom declaration | Spinel owner | Mapping status | Proof |
| --- | --- | --- | --- |
| record reader `enchantments()` and `EMPTY` | `get_enchantments`, `EMPTY` | Map directly with Rust getter naming | unit |
| `NETWORK_TYPE`, `CODEC` | no discovered equivalent | Missing | codec |
| `(RegistryKey<Enchantment>, int)` | `new` | Static Java construction maps naturally to `new` | unit |
| `has`, `level`, `with`, `remove` | same-named operations | Map directly; `with` is an immutable update, not a constructor | unit |

### Required side-by-side mappings

```java
public EnchantmentList(RegistryKey<Enchantment> enchantment, int level)
```

```rust
// Current Spinel
pub fn new(enchantment: RegistryKey<Enchantment>, level: i32) -> Self

```

```java
public int level(RegistryKey<Enchantment> enchantment)
```

```rust
pub fn level(&self, enchantment: &RegistryKey<Enchantment>) -> i32
```

The borrowed key avoids cloning and preserves the one-key lookup operation; Rust borrowing is the only representation difference.

## Edge Behavior Coverage

| Source behavior | Spinel state | Proof |
| --- | --- | --- |
| Map is copied on construction | Spinel takes ownership in `new`; alias behavior needs explicit test | unit |
| Missing key has level zero | present | unit |
| collection bound is `Short.MAX_VALUE` | missing | codec |
| `with` replaces an existing key | present | unit |

## Completion Gate

Unfinished until every selected constructor, value operation, NBT/data codec, network codec, collection bound, and dependency integration is mapped, implemented, and verified.
