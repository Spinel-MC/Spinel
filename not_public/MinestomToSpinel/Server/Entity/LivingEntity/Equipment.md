# Living entity equipment and hands

## Scope

Owner: selected public `LivingEntity` equipment/hand API and its `EquipmentHandler` inheritance. Excludes unrelated `LivingEntity` health and damage APIs.

## Reviewed Source Coverage

- Minestom: `entity/LivingEntity.java`, `inventory/EquipmentHandler.java`.
- Spinel: `entity/living/state.rs`, `living/equipment.rs`, `generic_entity.rs`, `entity.rs`, `entity_creature.rs`, `player/instance/inventory_state.rs`, `player/item_use.rs`, `server/instance.rs`.
- Missing intended Spinel owner path: a direct living-entity public owner/contract separate from arbitrary `GenericEntity`.

## Error Owner

No existing shared error owner covers this selected entity-equipment surface. The mapped shared owner is `spinel_server::entity::Error`, implemented at the intended path `spinel-server/src/entity/error.rs`; all no-value operations use `Result<(), Error>` and value operations use `Result<T, Error>`.

## Current Spinel State

`LivingState` stores all eight slots and updates attributes. `GenericEntity` exposes equipment regardless of living type; player inventory owns separate hand/equipment methods returning `bool`. Nonplayer mutation lacks Minestom's event and selected-slot viewer dispatch. Full visible entries omit saddle.

## Dependency Classification

| Dependency | Type | Minestom evidence | Block | Task branch |
| --- | --- | --- | --- | --- |
| [Equipment handler contract](../../Inventory/EquipmentHandler/Contract.md) | Core | `LivingEntity implements EquipmentHandler` | Yes | 1 |
| [Equipment slot](../EquipmentSlot/Slot.md) | Core | all read/write routing | Yes | 2 |
| [Equipment-slot group](../EquipmentSlotGroup/Group.md) | Cross | attribute modifier filtering | No | 3.2 |
| equip event infrastructure | Core | `EntityEquipEvent` precedes state storage | Yes | 3.1 |
| viewer packets/new-viewer path | Core | `syncEquipment`, `updateNewViewer` | Yes | 4 |
| player held-slot inventory | Cross | Player overrides generic equipment | No | 5 |

## Actionable Task Tree

1. [ ] Establish the direct living receiver that implements the shared handler contract; keep nonliving receivers outside this surface.
2. [ ] Initialize every selected slot to air and route generic `getEquipment` / `setEquipment` across main, off, boots, leggings, chestplate, helmet, body, and saddle.
3. [ ] Preserve mutation order.
   3.1 [ ] Fire `EntityEquipEvent`, use its replacement stack, then store it.
   3.2 [ ] Remove old and add new attribute modifiers only when each modifier's slot group contains the changed slot.
4. [ ] Preserve packet side effects.
   4.1 [ ] Send selected changed equipment to viewers.
   4.2 [ ] Send all equipment entries to a new viewer before the conditional attributes packet.
   4.3 [ ] Include saddle whenever Minestom's all-slot packet includes it.
5. [ ] Preserve hand animation and active-hand metadata behavior.
   5.1 [ ] Server-origin main/off swing reaches viewers and self.
   5.2 [ ] client-origin main/off swing reaches viewers only.
   5.3 [ ] `refreshActiveHand` coalesces metadata changes, maps off-hand status, updates riptide pose, and restores notifications.
6. [ ] Integrate player held-slot inventory without replacing the common living contract or changing source mutation semantics.

## Relevant Classes And Ownership Notes

Minestom declares the behavior on `LivingEntity`, inherited from `EquipmentHandler`. Spinel's future public surface must remain on the direct living counterpart; `Entity`, server, packet, and ExampleServer owners may dispatch/consume but must not become the API owner.

## Implementation Strategy Against Agent.md And DesignDecisionRules.md

Create a narrow `EquipmentHandler` shared contract and implement/delegate it through the direct living owner. The explicitly approved Rust representation passes `&mut World` to equipment mutation so the direct receiver can reach the source-required event and viewer transport without global state. Do not retain `GenericEntity`-wide equipment merely because it is convenient.

## Dependency-Aware Implementation Order

1. Tasks 1-2 after handler and slot documents.
2. Task 3.
3. Task 4.
4. Tasks 5-6.

## Verification Checklist

- [ ] all eight slots begin as air and are independently addressable on intended living receivers;
- [ ] nonliving receiver cannot use the living surface;
- [ ] equip event replacement, old/new modifier transitions, and group filtering;
- [ ] individual change, new-viewer full equipment, and recipient sets for all swing paths;
- [ ] player selected-hotbar main hand and off-hand synchronization;
- [ ] active-hand/riptide metadata and pose behavior.

## Public API Coverage

| Minestom declaration | Spinel owner | Mapping status | Proof |
| --- | --- | --- | --- |
| `LivingEntity(EntityType, UUID)`, `LivingEntity(EntityType)` | no direct living type | Missing | construction |
| `getEquipment`, `setEquipment` | direct living owner through `EquipmentHandler` | `get_equipment` returns `ItemStack`; `set_equipment` returns `Result<(), Error>` | unit/integration |
| `updateEquipmentAttributes` | `LivingState` / `LivingAttributes` | behavior partial; visibility/owner unresolved | unit |
| `updateNewViewer` equipment branch | Generic packet builder | integration missing | packet capture |
| `swingMainHand`, `swingOffHand`, internal client forms | Generic packet constructors | dispatch/receiver behavior unresolved | packet capture |
| `refreshActiveHand` | direct living owner | `Result<(), Error>` when dispatch/metadata work fails | metadata test |

### Required side-by-side mappings

```java
// Minestom: net.minestom.server.entity.LivingEntity
public ItemStack getEquipment(EquipmentSlot slot)
public void setEquipment(EquipmentSlot slot, ItemStack itemStack)
```

```rust
// Current Spinel split across GenericEntity and Player
pub fn get_equipment(&self, equipment_slot: EquipmentSlot) -> ItemStack
pub fn set_equipment(
    &mut self,
    equipment_slot: EquipmentSlot,
    item_stack: ItemStack,
) -> Result<(), Error>

```

```java
public void refreshActiveHand(boolean isHandActive, boolean offHand, boolean riptideSpinAttack)
```

```rust
pub fn refresh_active_hand(
    &mut self,
    is_hand_active: bool,
    off_hand: bool,
    riptide_spin_attack: bool,
) -> Result<(), Error>
```

## Edge Behavior Coverage

| Source behavior | Spinel state | Proof |
| --- | --- | --- |
| Event executes before storage and may replace stack | absent for nonplayer | integration |
| changed slot alone is synced | no common dispatch | packet |
| new viewer gets every enum slot then conditional attributes | saddle absent from visible entries | packet |
| client swing excludes self | packet creation only | packet |
| metadata update temporarily suppresses notifications | player-only path | unit |

## Completion Gate

Unfinished until all selected methods, inherited contract operations, event ordering, packets, metadata, attributes, receiver boundaries, and player integration are implemented and verified.
