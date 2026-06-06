use std::collections::HashMap;

use spinel_core::raycast::RaycastBoundingBox;
use spinel_network::types::{Position, Vector3d, chunk::ChunkData};
use spinel_registry::EntityType;

use crate::hit_target::{BlockHitTarget, HitTarget};

const BLOCK_INTERACTION_RANGE: f64 = 4.5;
const ENTITY_INTERACTION_RANGE: f64 = 3.0;
const DEFAULT_MIN_SECTION_Y: i32 = -4;

#[derive(Debug, Clone)]
pub struct TrackedEntity {
    entity_id: i32,
    position: Vector3d,
    width: f64,
    height: f64,
}

#[derive(Debug, Default, Clone)]
pub struct ClientHitTargetTracker {
    entities: HashMap<i32, TrackedEntity>,
    chunks: HashMap<(i32, i32), ChunkData>,
    block_state_overrides: HashMap<Position, i32>,
}

impl TrackedEntity {
    pub fn new(entity_id: i32, entity_type_id: i32, position: Vector3d) -> Self {
        let entity_type = EntityType::from_id(entity_type_id).unwrap_or(EntityType::PLAYER);
        Self {
            entity_id,
            position,
            width: entity_type.width(),
            height: entity_type.height(),
        }
    }

    pub const fn entity_id(&self) -> i32 {
        self.entity_id
    }

    pub const fn position(&self) -> Vector3d {
        self.position
    }

    pub fn set_position(&mut self, position: Vector3d) {
        self.position = position;
    }

    pub fn move_by_protocol_delta(&mut self, delta_x: i16, delta_y: i16, delta_z: i16) {
        self.position.x += protocol_delta(delta_x);
        self.position.y += protocol_delta(delta_y);
        self.position.z += protocol_delta(delta_z);
    }

    fn bounding_box(&self) -> RaycastBoundingBox {
        RaycastBoundingBox::from_center_dimensions(
            self.position,
            self.width,
            self.height,
            self.width,
        )
    }
}

impl ClientHitTargetTracker {
    pub fn track_entity(&mut self, entity: TrackedEntity) {
        self.entities.insert(entity.entity_id(), entity);
    }

    pub fn remove_entities(&mut self, entity_ids: impl IntoIterator<Item = i32>) {
        entity_ids.into_iter().for_each(|entity_id| {
            self.entities.remove(&entity_id);
        });
    }

    pub fn move_entity_by_protocol_delta(
        &mut self,
        entity_id: i32,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
    ) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.move_by_protocol_delta(delta_x, delta_y, delta_z);
        }
    }

    pub fn set_entity_position(&mut self, entity_id: i32, position: Vector3d) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.set_position(position);
        }
    }

    pub fn set_block_state(&mut self, position: Position, block_state: i32) {
        self.block_state_overrides.insert(position, block_state);
    }

    pub fn track_chunk(&mut self, chunk_x: i32, chunk_z: i32, chunk_data: ChunkData) {
        self.remove_block_state_overrides_in_chunk(chunk_x, chunk_z);
        self.chunks.insert((chunk_x, chunk_z), chunk_data);
    }

    pub fn remove_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        self.chunks.remove(&(chunk_x, chunk_z));
        self.remove_block_state_overrides_in_chunk(chunk_x, chunk_z);
    }

    fn remove_block_state_overrides_in_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        self.block_state_overrides.retain(|position, _| {
            position.x.div_euclid(16) != chunk_x || position.z.div_euclid(16) != chunk_z
        });
    }

    fn block_is_solid(&self, position: Position) -> bool {
        if let Some(block_state) = self.block_state_overrides.get(&position) {
            return *block_state != 0;
        }

        let chunk_x = position.x.div_euclid(16);
        let chunk_z = position.z.div_euclid(16);
        let section_index = position.y.div_euclid(16) - DEFAULT_MIN_SECTION_Y;
        let Some(section) = self
            .chunks
            .get(&(chunk_x, chunk_z))
            .and_then(|chunk| chunk.sections.get(section_index as usize))
        else {
            return false;
        };

        section
            .block_state_at(
                position.x.rem_euclid(16),
                position.y.rem_euclid(16),
                position.z.rem_euclid(16),
            )
            .is_some_and(|block_state| block_state != 0)
    }

    pub fn hit_target(&self, eye_position: Vector3d, direction: Vector3d) -> HitTarget {
        let entity_hit = self.entity_hit_target(eye_position, direction);
        let block_hit = self.block_hit_target(eye_position, direction);

        match (entity_hit, block_hit) {
            (Some(entity), Some(block)) if entity.distance_ratio <= block.distance_ratio => {
                HitTarget::Entity {
                    entity_id: entity.entity_id,
                }
            }
            (Some(_), Some(block)) => HitTarget::Block(block.target),
            (Some(entity), None) => HitTarget::Entity {
                entity_id: entity.entity_id,
            },
            (None, Some(block)) => HitTarget::Block(block.target),
            (None, None) => HitTarget::Miss,
        }
    }

    fn entity_hit_target(
        &self,
        eye_position: Vector3d,
        direction: Vector3d,
    ) -> Option<EntityHitTarget> {
        let ray = scaled_direction(direction, ENTITY_INTERACTION_RANGE);
        self.entities
            .values()
            .filter_map(|entity| {
                entity
                    .bounding_box()
                    .ray_intersection(eye_position, ray)
                    .map(|intersection| EntityHitTarget {
                        entity_id: entity.entity_id(),
                        distance_ratio: intersection.ratio,
                    })
            })
            .min_by(|first, second| first.distance_ratio.total_cmp(&second.distance_ratio))
    }

    fn block_hit_target(&self, eye_position: Vector3d, direction: Vector3d) -> Option<BlockHit> {
        grid_raycast(
            eye_position,
            direction,
            BLOCK_INTERACTION_RANGE,
            |position| self.block_is_solid(position),
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct EntityHitTarget {
    entity_id: i32,
    distance_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
struct BlockHit {
    target: BlockHitTarget,
    distance_ratio: f64,
}

fn protocol_delta(delta: i16) -> f64 {
    delta as f64 / 4096.0
}

fn scaled_direction(direction: Vector3d, range: f64) -> Vector3d {
    Vector3d {
        x: direction.x * range,
        y: direction.y * range,
        z: direction.z * range,
    }
}

fn grid_raycast(
    eye_position: Vector3d,
    direction: Vector3d,
    range: f64,
    block_is_solid: impl Fn(Position) -> bool,
) -> Option<BlockHit> {
    let mut current_position = Position {
        x: eye_position.x.floor() as i32,
        y: eye_position.y.floor() as i32,
        z: eye_position.z.floor() as i32,
    };

    if block_is_solid(current_position) {
        return Some(BlockHit {
            distance_ratio: 0.0,
            target: BlockHitTarget {
                position: current_position,
                face: 1,
            },
        });
    }

    let mut traversal = GridTraversal::new(eye_position, direction);

    while traversal.distance <= range {
        let face = traversal.step(&mut current_position);
        if block_is_solid(current_position) {
            return Some(BlockHit {
                distance_ratio: traversal.distance / range,
                target: BlockHitTarget {
                    position: current_position,
                    face,
                },
            });
        }
    }

    None
}

struct GridTraversal {
    step_x: i32,
    step_y: i32,
    step_z: i32,
    next_x: f64,
    next_y: f64,
    next_z: f64,
    delta_x: f64,
    delta_y: f64,
    delta_z: f64,
    distance: f64,
}

impl GridTraversal {
    fn new(eye_position: Vector3d, direction: Vector3d) -> Self {
        Self {
            step_x: axis_step(direction.x),
            step_y: axis_step(direction.y),
            step_z: axis_step(direction.z),
            next_x: axis_next_distance(eye_position.x, direction.x),
            next_y: axis_next_distance(eye_position.y, direction.y),
            next_z: axis_next_distance(eye_position.z, direction.z),
            delta_x: axis_delta(direction.x),
            delta_y: axis_delta(direction.y),
            delta_z: axis_delta(direction.z),
            distance: 0.0,
        }
    }

    fn step(&mut self, current_position: &mut Position) -> i8 {
        if self.next_x <= self.next_y && self.next_x <= self.next_z {
            current_position.x += self.step_x;
            self.distance = self.next_x;
            self.next_x += self.delta_x;
            return if self.step_x > 0 { 4 } else { 5 };
        }

        if self.next_y <= self.next_z {
            current_position.y += self.step_y;
            self.distance = self.next_y;
            self.next_y += self.delta_y;
            return if self.step_y > 0 { 0 } else { 1 };
        }

        current_position.z += self.step_z;
        self.distance = self.next_z;
        self.next_z += self.delta_z;
        if self.step_z > 0 { 2 } else { 3 }
    }
}

fn axis_step(direction: f64) -> i32 {
    if direction > 0.0 {
        return 1;
    }

    if direction < 0.0 {
        return -1;
    }

    0
}

fn axis_next_distance(position: f64, direction: f64) -> f64 {
    if direction > 0.0 {
        return (position.floor() + 1.0 - position) / direction;
    }

    if direction < 0.0 {
        return (position - position.floor()) / -direction;
    }

    f64::INFINITY
}

fn axis_delta(direction: f64) -> f64 {
    if direction == 0.0 {
        return f64::INFINITY;
    }

    1.0 / direction.abs()
}

#[cfg(test)]
mod tests {
    use spinel_network::types::{
        Position, Vector3d,
        chunk::{ChunkData, ChunkSection, PalettedContainer},
    };
    use spinel_registry::EntityType;

    use crate::hit_target::{BlockHitTarget, ClientHitTargetTracker, HitTarget, TrackedEntity};

    #[test]
    fn hit_target_selects_entity_in_view_ray() {
        let mut tracker = ClientHitTargetTracker::default();
        tracker.track_entity(TrackedEntity::new(
            7,
            EntityType::ZOMBIE.id(),
            Vector3d {
                x: 0.0,
                y: 64.0,
                z: 2.5,
            },
        ));

        let target = tracker.hit_target(
            Vector3d {
                x: 0.0,
                y: 65.62,
                z: 0.0,
            },
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );

        assert_eq!(target, HitTarget::Entity { entity_id: 7 });
    }

    #[test]
    fn hit_target_selects_nearest_block_in_view_ray() {
        let mut tracker = ClientHitTargetTracker::default();
        tracker.set_block_state(Position { x: 0, y: 65, z: 2 }, 1);

        let target = tracker.hit_target(
            Vector3d {
                x: 0.5,
                y: 65.5,
                z: 0.5,
            },
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );

        assert_eq!(
            target,
            HitTarget::Block(BlockHitTarget {
                position: Position { x: 0, y: 65, z: 2 },
                face: 2,
            })
        );
    }

    #[test]
    fn hit_target_selects_block_from_tracked_chunk() {
        let mut tracker = ClientHitTargetTracker::default();
        tracker.track_chunk(0, 0, chunk_with_single_block(0, 0, 2));

        let target = tracker.hit_target(
            Vector3d {
                x: 0.5,
                y: 64.5,
                z: 0.5,
            },
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );

        assert_eq!(
            target,
            HitTarget::Block(BlockHitTarget {
                position: Position { x: 0, y: 64, z: 2 },
                face: 2,
            })
        );
    }

    #[test]
    fn block_update_overrides_tracked_chunk_without_expansion() {
        let mut tracker = ClientHitTargetTracker::default();
        let block_position = Position { x: 0, y: 64, z: 2 };
        tracker.track_chunk(0, 0, chunk_with_single_block(0, 0, 2));
        tracker.set_block_state(block_position, 0);

        let target = tracker.hit_target(
            Vector3d {
                x: 0.5,
                y: 64.5,
                z: 0.5,
            },
            Vector3d {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );

        assert_eq!(target, HitTarget::Miss);
    }

    fn chunk_with_single_block(local_x: i32, local_y: i32, local_z: i32) -> ChunkData {
        let mut sections = (0..8)
            .map(|_| air_section())
            .chain(std::iter::once(section_with_single_block(
                local_x, local_y, local_z,
            )))
            .collect::<Vec<_>>();
        sections.extend((0..15).map(|_| air_section()));

        ChunkData {
            heightmaps: Vec::new(),
            sections,
            block_entities: Vec::new(),
        }
    }

    fn air_section() -> ChunkSection {
        ChunkSection {
            block_count: 0,
            block_states: PalettedContainer {
                bits_per_entry: 0,
                palette: Some(vec![0]),
                data: Vec::new(),
            },
            biomes: PalettedContainer {
                bits_per_entry: 0,
                palette: Some(vec![0]),
                data: Vec::new(),
            },
        }
    }

    fn section_with_single_block(local_x: i32, local_y: i32, local_z: i32) -> ChunkSection {
        let block_index = ((local_y as usize) << 8) | ((local_z as usize) << 4) | local_x as usize;
        let mut data = vec![0u64; 256];
        data[block_index / 16] = 1u64 << ((block_index % 16) * 4);

        ChunkSection {
            block_count: 1,
            block_states: PalettedContainer {
                bits_per_entry: 4,
                palette: Some(vec![0, 1]),
                data,
            },
            biomes: PalettedContainer {
                bits_per_entry: 0,
                palette: Some(vec![0]),
                data: Vec::new(),
            },
        }
    }
}
