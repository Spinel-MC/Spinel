use crate::entity::{Entity, EntityId, EntityPosition};
use crate::world::{CHUNK_SIZE_X, CHUNK_SIZE_Z, ChunkPosition};
use spinel_registry::EntityType;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityTrackerTarget {
    Entities,
    Players,
    Items,
    ExperienceOrbs,
}

#[derive(Clone, Copy)]
struct EntityTrackerEntry {
    uuid: Uuid,
    position: EntityPosition,
}

#[derive(Clone, Default)]
struct EntityTrackerTargetEntry {
    entities: HashSet<EntityId>,
    chunk_entities: HashMap<ChunkPosition, HashSet<EntityId>>,
}

#[derive(Clone, Default)]
pub struct EntityTracker {
    entries_by_entity_id: HashMap<EntityId, EntityTrackerEntry>,
    entries_by_entity_uuid: HashMap<Uuid, EntityId>,
    target_entries: HashMap<EntityTrackerTarget, EntityTrackerTargetEntry>,
    chunk_partitions: HashSet<ChunkPosition>,
}

impl EntityTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entity: &Entity) {
        let entity_id = entity.entity_id();
        let position = entity.position();
        self.entries_by_entity_id.insert(
            entity_id,
            EntityTrackerEntry {
                uuid: entity.uuid(),
                position,
            },
        );
        self.entries_by_entity_uuid.insert(entity.uuid(), entity_id);
        self.targets_for_entity(entity)
            .into_iter()
            .for_each(|target| self.register_target(entity_id, position, target));
    }

    pub fn unregister(&mut self, entity_id: EntityId) -> bool {
        let Some(entry) = self.entries_by_entity_id.remove(&entity_id) else {
            return false;
        };
        self.entries_by_entity_uuid.remove(&entry.uuid);
        self.target_entries
            .values_mut()
            .for_each(|target_entry| target_entry.unregister(entity_id, entry.position));
        true
    }

    pub fn move_entity(&mut self, entity_id: EntityId, position: EntityPosition) -> bool {
        let Some(entry) = self.entries_by_entity_id.get_mut(&entity_id) else {
            return false;
        };
        let previous_position = entry.position;
        entry.position = position;
        if entity_chunk_position(previous_position) == entity_chunk_position(position) {
            return true;
        }
        self.target_entries
            .values_mut()
            .filter(|target_entry| target_entry.entities.contains(&entity_id))
            .for_each(|target_entry| {
                target_entry.move_entity(entity_id, previous_position, position)
            });
        true
    }

    pub fn entity_by_id(&self, entity_id: EntityId) -> Option<EntityId> {
        self.entries_by_entity_id
            .contains_key(&entity_id)
            .then_some(entity_id)
    }

    pub fn entity_by_uuid(&self, entity_uuid: Uuid) -> Option<EntityId> {
        self.entries_by_entity_uuid.get(&entity_uuid).copied()
    }

    pub fn entities(&self, target: EntityTrackerTarget) -> Vec<EntityId> {
        self.target_entry(target)
            .map(EntityTrackerTargetEntry::entities)
            .unwrap_or_default()
    }

    pub fn chunk_entities(
        &self,
        position: ChunkPosition,
        target: EntityTrackerTarget,
    ) -> Vec<EntityId> {
        self.target_entry(target)
            .map(|target_entry| target_entry.chunk_entities(position))
            .unwrap_or_default()
    }

    pub fn nearby_entities_by_chunk_range(
        &self,
        position: EntityPosition,
        chunk_range: i32,
        target: EntityTrackerTarget,
    ) -> Vec<EntityId> {
        let center = entity_chunk_position(position);
        let target_entry = match self.target_entry(target) {
            Some(target_entry) => target_entry,
            None => return Vec::new(),
        };
        (-chunk_range..=chunk_range)
            .flat_map(|chunk_x_offset| {
                (-chunk_range..=chunk_range).flat_map(move |chunk_z_offset| {
                    target_entry.chunk_entities(ChunkPosition::new(
                        center.x + chunk_x_offset,
                        center.z + chunk_z_offset,
                    ))
                })
            })
            .collect()
    }

    pub fn nearby_entities(
        &self,
        position: EntityPosition,
        range: f64,
        target: EntityTrackerTarget,
    ) -> Vec<EntityId> {
        let chunk_range = (range / f64::from(CHUNK_SIZE_X)).floor() as i32 + 1;
        let squared_range = range * range;
        self.nearby_entities_by_chunk_range(position, chunk_range, target)
            .into_iter()
            .filter(|entity_id| {
                self.entries_by_entity_id
                    .get(entity_id)
                    .is_some_and(|entry| {
                        entity_distance_squared(position, entry.position) <= squared_range
                    })
            })
            .collect()
    }

    pub fn viewable(&self, position: ChunkPosition, chunk_range: i32) -> Vec<EntityId> {
        let point = EntityPosition::new(
            f64::from(position.x * CHUNK_SIZE_X),
            0.0,
            f64::from(position.z * CHUNK_SIZE_Z),
            0.0,
            0.0,
        );
        self.nearby_entities_by_chunk_range(point, chunk_range, EntityTrackerTarget::Players)
    }

    pub fn create_chunk_partition(&mut self, position: ChunkPosition) -> bool {
        self.chunk_partitions.insert(position)
    }

    pub fn delete_chunk_partition(&mut self, position: ChunkPosition) -> bool {
        self.chunk_partitions.remove(&position)
    }

    pub fn has_chunk_partition(&self, position: ChunkPosition) -> bool {
        self.chunk_partitions.contains(&position)
    }

    fn targets_for_entity(&self, entity: &Entity) -> Vec<EntityTrackerTarget> {
        match entity {
            Entity::Player(_) => vec![EntityTrackerTarget::Entities, EntityTrackerTarget::Players],
            Entity::Generic(entity) if entity.entity_type() == EntityType::ITEM => {
                vec![EntityTrackerTarget::Entities, EntityTrackerTarget::Items]
            }
            Entity::Generic(entity) if entity.entity_type() == EntityType::EXPERIENCE_ORB => {
                vec![
                    EntityTrackerTarget::Entities,
                    EntityTrackerTarget::ExperienceOrbs,
                ]
            }
            Entity::Generic(_) => vec![EntityTrackerTarget::Entities],
        }
    }

    fn register_target(
        &mut self,
        entity_id: EntityId,
        position: EntityPosition,
        target: EntityTrackerTarget,
    ) {
        self.target_entries
            .entry(target)
            .or_default()
            .register(entity_id, position);
    }

    fn target_entry(&self, target: EntityTrackerTarget) -> Option<&EntityTrackerTargetEntry> {
        self.target_entries.get(&target)
    }
}

impl EntityTrackerTargetEntry {
    fn register(&mut self, entity_id: EntityId, position: EntityPosition) {
        self.entities.insert(entity_id);
        self.chunk_entities
            .entry(entity_chunk_position(position))
            .or_default()
            .insert(entity_id);
    }

    fn unregister(&mut self, entity_id: EntityId, position: EntityPosition) {
        self.entities.remove(&entity_id);
        self.remove_from_chunk(entity_id, entity_chunk_position(position));
    }

    fn move_entity(
        &mut self,
        entity_id: EntityId,
        previous_position: EntityPosition,
        position: EntityPosition,
    ) {
        self.remove_from_chunk(entity_id, entity_chunk_position(previous_position));
        self.chunk_entities
            .entry(entity_chunk_position(position))
            .or_default()
            .insert(entity_id);
    }

    fn entities(&self) -> Vec<EntityId> {
        self.entities.iter().copied().collect()
    }

    fn chunk_entities(&self, position: ChunkPosition) -> Vec<EntityId> {
        self.chunk_entities
            .get(&position)
            .map(|entities| entities.iter().copied().collect())
            .unwrap_or_default()
    }

    fn remove_from_chunk(&mut self, entity_id: EntityId, position: ChunkPosition) {
        let Some(chunk_entities) = self.chunk_entities.get_mut(&position) else {
            return;
        };
        chunk_entities.remove(&entity_id);
        if chunk_entities.is_empty() {
            self.chunk_entities.remove(&position);
        }
    }
}

fn entity_chunk_position(position: EntityPosition) -> ChunkPosition {
    ChunkPosition::new(
        (position.x().floor() as i32).div_euclid(CHUNK_SIZE_X),
        (position.z().floor() as i32).div_euclid(CHUNK_SIZE_Z),
    )
}

fn entity_distance_squared(first: EntityPosition, second: EntityPosition) -> f64 {
    let x_distance = first.x() - second.x();
    let y_distance = first.y() - second.y();
    let z_distance = first.z() - second.z();
    x_distance * x_distance + y_distance * y_distance + z_distance * z_distance
}
