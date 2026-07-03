use crate::events::chunk_loader_error::ChunkLoaderErrorEvent;
use crate::server::MinecraftServer;
use crate::world::{Chunk, ChunkLoader, ChunkLoaderOperation, ChunkPosition, World};
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use std::io;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

static CHUNK_LOADER_ERROR_TEST_LOCK: Mutex<()> = Mutex::new(());
static CHUNK_LOADER_ERROR_EVENTS: Mutex<
    Vec<(ChunkLoaderOperation, Option<ChunkPosition>, String)>,
> = Mutex::new(Vec::new());

#[event_listener]
fn record_chunk_loader_error(event: &mut ChunkLoaderErrorEvent, _server: &mut MinecraftServer) {
    if event.world().name() != &Identifier::minecraft("chunk_loader_error_test") {
        return;
    }
    CHUNK_LOADER_ERROR_EVENTS.lock().unwrap().push((
        event.operation,
        event.chunk_position,
        event.message.clone(),
    ));
}

struct FailingParallelChunkLoader;

impl ChunkLoader for FailingParallelChunkLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        Err(io::Error::other("intentional asynchronous load failure"))
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }

    fn supports_parallel_loading(&self) -> bool {
        true
    }
}

#[test]
fn asynchronous_chunk_loader_failure_dispatches_world_event() {
    let _test_lock = CHUNK_LOADER_ERROR_TEST_LOCK.lock().unwrap();
    CHUNK_LOADER_ERROR_EVENTS.lock().unwrap().clear();
    let mut server = MinecraftServer::new();
    let mut world = World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("chunk_loader_error_test"),
    );
    world.set_event_dispatcher(&mut server as *mut MinecraftServer as usize);
    world.set_chunk_loader(FailingParallelChunkLoader);
    let position = ChunkPosition::new(7, -4);
    let ticket = world.load_chunk_future(position).unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);

    loop {
        match world.complete_chunk_load(&ticket) {
            Err(error) => {
                assert_eq!(error.to_string(), "intentional asynchronous load failure");
                break;
            }
            Ok(true) => panic!("failed chunk load must not complete successfully"),
            Ok(false) if Instant::now() < deadline => thread::yield_now(),
            Ok(false) => panic!("asynchronous chunk load did not finish before deadline"),
        }
    }

    let events = CHUNK_LOADER_ERROR_EVENTS.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].0, ChunkLoaderOperation::LoadChunk);
    assert_eq!(events[0].1, Some(position));
    assert_eq!(events[0].2, "intentional asynchronous load failure");
}
