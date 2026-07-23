# PhysicsResult post-movement observation

## Scope and owner

Minestom owns physics simulation on `PhysicsUtils` and the resolved result value on experimental `PhysicsResult`. `Entity` retains the most recently calculated result privately. Spinel already computes and retains the direct counterpart `EntityPhysicsResult`; this document exposes that retained record through every runtime entity receiver so external policy can observe resolved movement without adding `fall_distance` state to Spinel.

## Reviewed source coverage

- Minestom `PhysicsResult` record components and constructors in `src/main/java/net/minestom/server/collision/PhysicsResult.java`.
- Minestom `PhysicsUtils.simulateMovement(...)` and `updateVelocity(...)` in `src/main/java/net/minestom/server/collision/PhysicsUtils.java`.
- Minestom `Entity.movementTick()` and private `previousPhysicsResult` storage in `src/main/java/net/minestom/server/entity/Entity.java`.
- Spinel `EntityPhysicsResult`, `EntitySweepResult`, `simulate_movement`, `GenericEntity::movement_tick`, `Player::movement_tick`, and `Entity` variant dispatch.

## Public API mappings

```java
public record PhysicsResult(
    Pos newPosition,
    Vec newVelocity,
    boolean isOnGround,
    boolean collisionX,
    boolean collisionY,
    boolean collisionZ,
    Vec originalDelta,
    Point[] collisionPoints,
    Shape[] collisionShapes,
    Point[] collisionShapePositions,
    boolean hasCollision,
    SweepResult res,
    boolean cached
)
```

```rust
pub struct EntityPhysicsResult

pub const fn get_new_position(self) -> EntityPosition
pub const fn get_new_velocity_per_tick(self) -> Velocity
pub const fn is_on_ground(self) -> bool
pub const fn has_collision_x(self) -> bool
pub const fn has_collision_y(self) -> bool
pub const fn has_collision_z(self) -> bool
pub const fn get_original_delta(self) -> Velocity
pub const fn get_collision_points(self) -> [Option<Vector3d>; 3]
pub const fn get_collision_shapes(self) -> [Option<&'static [BlockShapeBox]>; 3]
pub const fn get_collision_shape_positions(self) -> [Option<BlockPosition>; 3]
pub const fn has_collision(self) -> bool
pub const fn get_sweep(self) -> EntitySweepResult
pub const fn is_cached(self) -> bool
```

```java
PhysicsResult physicsResult = PhysicsUtils.simulateMovement(...);
this.previousPhysicsResult = physicsResult;
```

```rust
let physics_result = simulate_movement(...);
self.previous_physics_result = Some(physics_result);
```

```rust
pub const fn get_last_physics_result(&self) -> Option<EntityPhysicsResult>
```

The final getter is the explicitly requested narrow Spinel exposure of Minestom's privately retained post-move result. Rust has no inherited `Entity` base object in this representation, so `GenericEntity` and `Player` expose their direct stored result while `Entity::get_last_physics_result` performs exhaustive variant dispatch. `LivingEntity`, `EntityCreature`, and projectile owners inherit generic movement through their direct embedded owner and therefore retain the same receiver capability.

## Dependencies

- Core: `EntityPhysicsResult` retains resolved collision and support evidence produced by `simulate_movement`.
- Core: Generic entity and player movement retain a record before loaded-chunk acceptance, matching Minestom's assignment order.
- Soft: ExampleServer's `example-entity` uses the record to own Vanilla fall-distance accumulation and reset policy.

## Edge behavior inventory

- Before the first movement simulation, `get_last_physics_result` returns `None`.
- The result exposes applied position, post-drag velocity, pre-resolution delta, per-axis collision flags, contact evidence, support block positions, sweep evidence, ground state, and cache state.
- Player movement preserves client-authoritative position while retaining the simulation result and post-physics velocity.
- Generic, living, creature, item, experience-orb, and projectile variants all resolve through generic movement; player resolves through its direct movement owner.
- The capability does not add, derive, persist, or reset `fall_distance`.

## Actionable task tree

1. [x] Map all experimental `PhysicsResult` components to existing `EntityPhysicsResult` accessors.
2. [x] Expose the retained result on direct generic and player movement owners.
3. [x] Add exhaustive `Entity` dispatch for all runtime variants.
4. [x] Verify collision-support evidence and player result retention in focused physics tests.

## Verification checklist

- [x] Generic collision movement retains on-ground, Y-axis collision, collision presence, and supporting block position.
- [x] Player movement retains its resolved result while preserving client position behavior.
- [x] `Entity` dispatch returns the stored result for a simulated generic variant.

## Completion gate

This capability is complete when focused entity physics tests prove direct and universal access to the same resolved post-movement record. Vanilla fall-distance policy remains owned externally by `example-entity`.