# Equipment slot

## Scope

Owner: Minestom `EquipmentSlot` enum, its eight variants, codecs, collection, accessors, and legacy-ID invalid path.

## Reviewed Source Coverage

- Minestom: `entity/EquipmentSlot.java`.
- Spinel: `spinel-server/src/entity/equipment_slot.rs`; protocol dependency `spinel-core` set-equipment slot type.

## Current Spinel State

All eight values and ID/NBT/hand/armor accessors are present. No selected `NETWORK_TYPE`, `CODEC`, `armors`, or `fromLegacyProtocolId` operation was found.

## Dependency Classification

| Dependency | Type | Minestom evidence | Block | Task branch |
| --- | --- | --- | --- | --- |
| protocol enum codec | Core | `NETWORK_TYPE` | Yes | 1.2 |
| NBT string codec | Core | `CODEC` | Yes | 1.3 |
| living and handler documents | Soft | slots are consumed there | No | 2 |

## Actionable Task Tree

1. [ ] Preserve enum data contract.
   1.1 [ ] Verify all eight variants and their protocol ID, legacy ID, NBT name, armor flag, and armor inventory slot.
   1.2 [ ] Map the modern VAR_INT protocol codec.
   1.3 [ ] Map the NBT-name string codec.
   1.4 [ ] Map `armors()` fixed collection order.
   1.5 [ ] Map invalid `fromLegacyProtocolId` state failure.
2. [ ] Integrate this single enum with [Living entity equipment](../LivingEntity/Equipment.md) and [Equipment handler contract](../../Inventory/EquipmentHandler/Contract.md).

## Relevant Classes And Ownership Notes

`EquipmentSlot` remains an entity-domain enum. Packet serializers consume it but must not own it.

## Implementation Strategy Against Agent.md And DesignDecisionRules.md

Keep slot semantics on `EquipmentSlot`; do not create packet-specific duplicate enums. The Rust failure representation for invalid legacy IDs is unresolved.

## Dependency-Aware Implementation Order

1. 1.1.
2. 1.2-1.5.
3. 2.

## Verification Checklist

- [ ] variant table;
- [ ] modern protocol codec;
- [ ] NBT codec;
- [ ] armor collection ordering;
- [ ] invalid legacy-ID behavior.

## Public API Coverage

| Minestom declaration | Spinel owner | Mapping status | Proof |
| --- | --- | --- | --- |
| eight enum variants | `EquipmentSlot` | Present | unit |
| `NETWORK_TYPE`, `CODEC` | none found | Missing | codec |
| `armors()` | none found | Missing | unit |
| `fromLegacyProtocolId` | none found | Missing/error-shape unresolved | unit |
| remaining accessors | `get_*`, `is_*` | Rust getter naming accepted; behavior unverified | unit |

### Required side-by-side mappings

```java
public static List<EquipmentSlot> armors()
```

```rust
// Unresolved: no current Spinel equivalent.
```

```java
public static EquipmentSlot fromLegacyProtocolId(int legacyProtocolId)
```

```rust
// Unresolved: no current Spinel equivalent; invalid-value error representation is undecided.
```

## Edge Behavior Coverage

| Source behavior | Spinel state | Proof |
| --- | --- | --- |
| invalid legacy ID throws | missing | unit |
| armor collection has boots, leggings, chestplate, helmet order | missing | unit |
| modern and legacy IDs intentionally differ for off hand and armor | accessor values present | unit |

## Completion Gate

Unfinished until every enum surface row and codec/error behavior is mapped, implemented, and verified.
