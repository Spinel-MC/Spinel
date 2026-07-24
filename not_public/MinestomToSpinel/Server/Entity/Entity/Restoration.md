# Entity restoration and passenger composition

## Scope and owner

Owner: the selected public `Entity` construction, mutable state, and passenger APIs, plus direct subtype constructors that preserve their existing required construction inputs while accepting a persisted UUID. Player construction already receives UUID and connection inputs; player persistence remains outside this document.

## Reviewed source coverage

- Minestom `Entity(EntityType, UUID)`, `setInstance(Instance, Pos)`, `setInstance(Instance, Point)`, `getPosition`, `getVelocity`, `setVelocity`, `addPassenger`, and `removePassenger` in `src/main/java/net/minestom/server/entity/Entity.java`.
- Minestom `EntityCreature(EntityType, UUID)` in `src/main/java/net/minestom/server/entity/EntityCreature.java`.
- Spinel `Entity`, `GenericEntity`, `LivingEntity`, `EntityCreature`, `ItemEntity`, `ExperienceOrb`, `ProjectileEntity`, and `Player` owners named in Module.md.

## Public API mappings

```java
public Entity(EntityType entityType, UUID uuid)
public void addPassenger(Entity entity)
public void removePassenger(Entity entity)
```

```rust
pub fn with_uuid(entity_type: EntityType, uuid: Uuid) -> Self
pub fn add_passenger(&mut self, passenger: &mut Entity) -> Result<bool, Error>
pub fn remove_passenger(&mut self, passenger: &mut Entity) -> Result<bool, Error>
```rust
pub fn add_passenger(&mut self, vehicle_id: EntityId, passenger_id: EntityId) -> Result<bool, Error>
pub fn remove_passenger(&mut self, vehicle_id: EntityId, passenger_id: EntityId) -> Result<bool, Error>
```
```

Rust has no inheritance constructor dispatch. `GenericEntity`, `LivingEntity`, `EntityCreature`, `ItemEntity`, `ExperienceOrb`, and `ProjectileEntity` each expose `with_uuid` on their direct owner. Each retains its existing required non-UUID constructor inputs. `Player::new` already accepts UUID with its required protocol-session inputs.

The `Entity` methods remain the direct Minestom-shaped passenger owner. `World` additionally exposes the same operations by entity ID because an external restoration codec cannot safely borrow two stored `Entity` values mutably at once. The world boundary obtains both values with the safe distinct-entity split, invokes the direct owner operation, updates tracker and visibility state, and sends the passenger and position packets. This is an unavoidable Rust representation boundary; it does not move passenger ownership from `Entity`.

```java
public CompletableFuture<Void> setInstance(Instance instance, Pos spawnPosition)
public Pos getPosition()
public Vec getVelocity()
public void setVelocity(Vec velocity)
```

```rust
pub fn set_world_at(self, world: &mut World, position: EntityPosition) -> bool
pub fn get_position(&self) -> EntityPosition
pub fn get_velocity(&self) -> Velocity
pub fn set_position(&mut self, position: EntityPosition)
pub fn set_velocity(&mut self, velocity: Velocity)
```

`set_world_at` is the existing Spinel synchronous representation of Minestom instance placement. Public universal mutable state stays on `Entity` and dispatches every enum variant rather than widening a generic-only owner.

## Dependencies

- Core: direct living ownership must remain in `LivingEntity`.
- Core: world entity relationship dispatch performs packet and visibility effects for mounted entities.
- Soft: ExampleServer owns persistence codecs and invokes these public APIs.

## Edge behavior inventory

- UUID construction must preserve the supplied UUID without changing subtype-required initial state.
- Position and velocity restoration must work through `Entity` for creature, experience orb, generic, item, living, player, and projectile variants.
- Adding a passenger transfers it from a prior vehicle, aligns worlds, updates passenger position, and produces the relationship packet in world-managed integration.
- Removing a passenger clears its vehicle relation and produces the relationship packet in world-managed integration.
- Invalid passenger relationships return the shared `entity::Error` contract.

## Actionable task tree

1. [x] Add direct UUID constructors for every non-player runtime subtype lacking one.
2. [x] Preserve Player UUID construction through its existing required session constructor.
3. [x] Expose universal position and velocity restoration methods on `Entity`.
4. [x] Use the shared entity error contract for public passenger composition.
5. [x] Verify every enum variant accepts restored position and velocity.

## Verification checklist

- [x] direct UUID constructors preserve supplied UUID for generic, living, creature, experience orb, item, projectile, and player construction;
- [x] `Entity` restores position and velocity for all runtime enum variants;
- [x] mounted lifecycle packet and visibility integration remains covered by world passenger tests.

## Completion gate

This prerequisite is complete only when the focused owner tests and world passenger integration tests pass. Entity serialization remains ExampleServer-owned and is intentionally outside this Minestom capability document.