use super::super::world_manager::WorldManager;
use crate::entity::{Entity, EntityPosition, GenericEntity};
use crate::world::{Chunk, ChunkLoader, ChunkPosition};
use spinel_network::types::Identifier;
use spinel_registry::EntityType;
use spinel_registry::dimension_type::DimensionType;
use std::io;

struct ManagerTestChunkLoader;

impl ChunkLoader for ManagerTestChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn world_manager_create_and_register_worlds_match_minestom_world_manager_surface() {
    let mut worlds = WorldManager::new();
    let first_world = worlds.create_world(Identifier::minecraft("overworld"));
    let second_world =
        worlds.create_world_with_loader(Identifier::minecraft("custom"), ManagerTestChunkLoader);
    let nether_world = worlds.create_world_with_dimension(
        DimensionType::THE_NETHER,
        DimensionType::builder()
            .vertical_bounds(-32, 256, 128)
            .build(),
    );
    let end_world = worlds.create_world_with_dimension_and_loader(
        DimensionType::THE_END,
        DimensionType::default(),
        ManagerTestChunkLoader,
    );

    assert_eq!(worlds.worlds().len(), 4);
    assert!(
        worlds
            .world(first_world)
            .is_some_and(|world| world.is_registered())
    );
    assert!(
        worlds
            .world(second_world)
            .is_some_and(|world| world.is_registered())
    );
    assert_eq!(
        worlds
            .world(nether_world)
            .map(|world| world.get_dimension_type().clone()),
        Some(DimensionType::THE_NETHER)
    );
    assert_eq!(
        worlds
            .world(end_world)
            .map(|world| world.dimension_name().clone()),
        Some(Identifier::minecraft("the_end"))
    );
}

#[test]
fn world_manager_add_passenger_moves_passenger_to_vehicle_world_first() {
    let mut worlds = WorldManager::new();
    let vehicle_world = worlds.create_world(Identifier::minecraft("overworld"));
    let passenger_world = worlds.create_world(Identifier::minecraft("the_nether"));
    let mut vehicle = GenericEntity::new(EntityType::PIG);
    vehicle.set_position(EntityPosition::new(12.0, 70.0, 8.0, 0.0, 0.0));
    let vehicle_id = vehicle.get_entity_id();
    let mut passenger = GenericEntity::new(EntityType::ZOMBIE);
    passenger.set_position(EntityPosition::new(-20.0, 40.0, 30.0, 0.0, 0.0));
    let passenger_id = passenger.get_entity_id();
    worlds
        .world_mut(vehicle_world)
        .unwrap()
        .add_entity(Entity::Generic(vehicle));
    worlds
        .world_mut(passenger_world)
        .unwrap()
        .add_entity(Entity::Generic(passenger));

    assert!(worlds.add_passenger(vehicle_id, passenger_id).unwrap());

    assert!(
        worlds
            .world(passenger_world)
            .unwrap()
            .get_entity(passenger_id)
            .is_none()
    );
    let vehicle_world = worlds.world(vehicle_world).unwrap();
    assert!(
        vehicle_world
            .get_entity(vehicle_id)
            .unwrap()
            .get_passengers()
            .contains(&passenger_id)
    );
    let passenger = vehicle_world.get_entity(passenger_id).unwrap();
    assert_eq!(passenger.get_vehicle(), Some(vehicle_id));
    assert_eq!(
        passenger.get_position(),
        EntityPosition::new(
            12.0,
            70.0 + EntityType::PIG.get_height() * 0.75,
            8.0,
            0.0,
            0.0,
        )
    );
}
