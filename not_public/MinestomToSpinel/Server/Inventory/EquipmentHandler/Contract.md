# Equipment handler contract

## Scope

Owner: Minestom `EquipmentHandler` interface, including its two abstract operations and all inherited public default operations. Concrete event/attribute behavior remains in [Living entity equipment](../../Entity/LivingEntity/Equipment.md).

## Reviewed Source Coverage

- Minestom: `inventory/EquipmentHandler.java`.
- Spinel: `entity/living/state.rs`, `living/equipment.rs`, `generic_entity.rs`, `player/instance/inventory_state.rs`, `entity/equipment_slot.rs`.

## Current Spinel State

No shared public contract was found. Player offers hand/equipment methods; `GenericEntity` offers generic equipment; `LivingState` owns storage; packet construction is elsewhere.

## Dependency Classification

| Dependency | Type | Minestom evidence | Block | Task branch |
| --- | --- | --- | --- | --- |
| [Equipment slot](../../Entity/EquipmentSlot/Slot.md) | Core | every default routes via a slot | Yes | 1 |
| [Living entity equipment](../../Entity/LivingEntity/Equipment.md) | Core | concrete `LivingEntity` implementation | Yes | 2 |
| entity viewer packet dispatch | Core | sync and full packet defaults require `Entity` | Yes | 3 |

## Actionable Task Tree

1. [ ] Expose abstract `getEquipment` and `setEquipment` on the shared contract.
2. [ ] Expose hand and armor defaults through the same contract.
   2.1 [ ] `get/setItemInMainHand`, `get/setItemInOffHand`, `get/setItemInHand`.
   2.2 [ ] `get/setHelmet`, `get/setChestplate`, `get/setLeggings`, `get/setBoots`, `get/setBodyEquipment`.
   2.3 [ ] `hasEquipment` with air-only false behavior.
3. [ ] Expose synchronization defaults with Entity-only behavior.
   3.1 [ ] `syncEquipment(slot)` delegates to the selected stack form.
   3.2 [ ] `syncEquipment(slot, stack)` sends one viewer packet.
   3.3 [ ] `getEquipmentsPacket()` includes every `EquipmentSlot` value.
   3.4 [ ] Preserve non-Entity state failure rather than silently succeeding.

## Relevant Classes And Ownership Notes

This is the shared Minestom contract; `LivingEntity` is the concrete receiver. Player must inherit/delegate rather than define a competing hand API. Packet structs remain transport values, not the owner.

## Implementation Strategy Against Agent.md And DesignDecisionRules.md

Use the narrow `EquipmentHandler` Rust trait as the shared contract and implement/delegate it through the direct living owner. Do not duplicate defaults across player and creature types.

## Dependency-Aware Implementation Order

1. 1.
2. 2.1-2.3.
3. 3.1-3.4 with living viewer integration.

## Verification Checklist

- [ ] every default routes to the exact source slot;
- [ ] air/non-air `hasEquipment`;
- [ ] selected and all-slot packet composition;
- [ ] Entity-only invalid receiver path;
- [ ] player and nonplayer living receiver availability.

## Public API Coverage

| Minestom declaration | Spinel owner | Mapping status | Proof |
| --- | --- | --- | --- |
| abstract `getEquipment`, `setEquipment` | `EquipmentHandler` trait | `get_equipment` returns `ItemStack`; `set_equipment` returns `Result<(), SetEquipmentError>` | compile/API |
| six hand default operations | `EquipmentHandler` trait defaults | setters return `Result<(), SetEquipmentError>` | unit |
| ten armor/body defaults and `hasEquipment` | `EquipmentHandler` trait defaults | setters return `Result<(), SetEquipmentError>` | unit |
| `syncEquipment` overloads | `EquipmentHandler` trait | `Result<(), SyncEquipmentError>` | packet |
| `getEquipmentsPacket` | `EquipmentHandler` trait | `Result<SetEquipmentPacket, GetEquipmentsPacketError>` | packet |

### Required side-by-side mappings

```java
public default ItemStack getItemInHand(PlayerHand hand)
public default void setItemInHand(PlayerHand hand, ItemStack stack)
```

```rust
pub fn get_item_in_hand(&self, hand: PlayerHand) -> ItemStack
pub fn set_item_in_hand(
    &mut self,
    hand: PlayerHand,
    item_stack: ItemStack,
) -> Result<(), SetEquipmentError>
```

```java
public default EntityEquipmentPacket getEquipmentsPacket()
```

```rust
pub fn get_equipments_packet(&self) -> Result<SetEquipmentPacket, GetEquipmentsPacketError>
```

## Edge Behavior Coverage

| Source behavior | Spinel state | Proof |
| --- | --- | --- |
| `getItemInHand` switches exactly MAIN/OFF | player-only partial | unit |
| body defaults exist; saddle has no named convenience | body absent; saddle remains generic only | unit |
| full packet loops every enum value | current visible entries omit saddle | packet |
| non-Entity handler fails state check | no equivalent | unit |

## Completion Gate

Unfinished until every abstract/default operation, receiver boundary, packet side effect, and invalid-receiver branch is mapped, implemented, and verified.
