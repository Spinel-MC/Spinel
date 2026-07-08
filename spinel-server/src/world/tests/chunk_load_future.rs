use super::super::instance::World;
use crate::world::{Chunk, ChunkLoader, ChunkPosition};
use spinel_network::types::Identifier;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;

struct ParallelTrackingLoader {
    load_count: Arc<AtomicUsize>,
    save_count: Arc<AtomicUsize>,
    load_thread: Arc<Mutex<Option<ThreadId>>>,
    save_thread: Arc<Mutex<Option<ThreadId>>>,
    should_fail_load: bool,
}

struct SynchronousTrackingLoader {
    load_thread: Arc<Mutex<Option<ThreadId>>>,
    save_thread: Arc<Mutex<Option<ThreadId>>>,
}

impl ChunkLoader for ParallelTrackingLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        *self.load_thread.lock().unwrap() = Some(std::thread::current().id());
        self.load_count.fetch_add(1, Ordering::SeqCst);
        if self.should_fail_load {
            return Err(io::Error::new(io::ErrorKind::Other, "load failed"));
        }
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        *self.save_thread.lock().unwrap() = Some(std::thread::current().id());
        self.save_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }

    fn supports_parallel_loading(&self) -> bool {
        true
    }

    fn supports_parallel_saving(&self) -> bool {
        true
    }
}

impl ChunkLoader for SynchronousTrackingLoader {
    fn load_chunk(&self, _position: ChunkPosition) -> io::Result<Option<Chunk>> {
        *self.load_thread.lock().unwrap() = Some(std::thread::current().id());
        Ok(None)
    }

    fn save_chunk(&self, _chunk: &Chunk) -> io::Result<()> {
        *self.save_thread.lock().unwrap() = Some(std::thread::current().id());
        Ok(())
    }

    fn unload_chunk(&self, _chunk: &mut Chunk) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn future_chunk_loads_share_in_flight_ticket_and_remove_it_after_completion() {
    let load_count = Arc::new(AtomicUsize::new(0));
    let mut world = test_world();
    let chunk_position = ChunkPosition::new(2, 3);
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: load_count.clone(),
        save_count: Arc::new(AtomicUsize::new(0)),
        load_thread: Arc::new(Mutex::new(None)),
        save_thread: Arc::new(Mutex::new(None)),
        should_fail_load: false,
    });

    let first_ticket = world.load_chunk_future(chunk_position).unwrap();
    let second_ticket = world.load_chunk_future(chunk_position).unwrap();

    assert_eq!(first_ticket, second_ticket);
    assert!(world.chunk_load_in_progress(chunk_position));
    while !world.complete_chunk_load(&first_ticket).unwrap() {}
    assert!(!world.chunk_load_in_progress(chunk_position));
    assert!(world.chunk(chunk_position).is_some());
    assert_eq!(load_count.load(Ordering::SeqCst), 1);
}

#[test]
fn future_chunk_load_does_not_block_while_generator_runs() {
    let mut world = test_world();
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: Arc::new(AtomicUsize::new(0)),
        save_count: Arc::new(AtomicUsize::new(0)),
        load_thread: Arc::new(Mutex::new(None)),
        save_thread: Arc::new(Mutex::new(None)),
        should_fail_load: false,
    });
    world.set_generator(|_| {
        std::thread::sleep(std::time::Duration::from_millis(200));
    });

    let load_started = std::time::Instant::now();
    let ticket = world.load_chunk_future(ChunkPosition::new(2, 3)).unwrap();

    assert!(load_started.elapsed() < std::time::Duration::from_millis(100));
    assert!(!world.complete_chunk_load(&ticket).unwrap());
    while !world.complete_chunk_load(&ticket).unwrap() {
        std::thread::yield_now();
    }
}

#[test]
fn future_chunk_load_failure_removes_in_flight_entry_and_propagates_error() {
    let mut world = test_world();
    let chunk_position = ChunkPosition::new(2, 3);
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: Arc::new(AtomicUsize::new(0)),
        save_count: Arc::new(AtomicUsize::new(0)),
        load_thread: Arc::new(Mutex::new(None)),
        save_thread: Arc::new(Mutex::new(None)),
        should_fail_load: true,
    });

    let ticket = world.load_chunk_future(chunk_position).unwrap();
    let load_error = loop {
        match world.complete_chunk_load(&ticket) {
            Ok(false) => continue,
            Ok(true) => panic!("failed load should not complete successfully"),
            Err(error) => break error,
        }
    };

    assert_eq!(load_error.kind(), io::ErrorKind::Other);
    assert!(!world.chunk_load_in_progress(chunk_position));
}

#[test]
fn optional_future_chunk_load_respects_auto_chunk_load_disabled() {
    let mut world = test_world();

    world.enable_auto_chunk_load(false);

    assert!(
        world
            .load_optional_chunk_future(ChunkPosition::new(2, 3))
            .unwrap()
            .is_none()
    );
}

#[test]
fn future_chunk_load_and_save_follow_loader_parallel_flags() {
    let load_thread = Arc::new(Mutex::new(None));
    let save_thread = Arc::new(Mutex::new(None));
    let save_count = Arc::new(AtomicUsize::new(0));
    let mut world = test_world();
    let caller_thread = std::thread::current().id();
    world.set_chunk_loader(ParallelTrackingLoader {
        load_count: Arc::new(AtomicUsize::new(0)),
        save_count: save_count.clone(),
        load_thread: load_thread.clone(),
        save_thread: save_thread.clone(),
        should_fail_load: false,
    });

    let ticket = world.load_chunk_future(ChunkPosition::new(0, 0)).unwrap();
    while !world.complete_chunk_load(&ticket).unwrap() {}
    world.save_chunks_future().join().unwrap();

    assert_ne!(load_thread.lock().unwrap().unwrap(), caller_thread);
    assert_ne!(save_thread.lock().unwrap().unwrap(), caller_thread);
    assert_eq!(save_count.load(Ordering::SeqCst), 1);
}

#[test]
fn non_parallel_loader_runs_on_caller_before_async_chunk_completion() {
    let load_thread = Arc::new(Mutex::new(None));
    let save_thread = Arc::new(Mutex::new(None));
    let mut world = test_world();
    let caller_thread = std::thread::current().id();
    world.set_chunk_loader(SynchronousTrackingLoader {
        load_thread: load_thread.clone(),
        save_thread: save_thread.clone(),
    });

    let ticket = world.load_chunk_future(ChunkPosition::new(0, 0)).unwrap();
    assert_eq!(load_thread.lock().unwrap().unwrap(), caller_thread);
    while !world.complete_chunk_load(&ticket).unwrap() {
        std::thread::yield_now();
    }
    world
        .save_chunk_future(ChunkPosition::new(0, 0))
        .join()
        .unwrap();
    assert_eq!(save_thread.lock().unwrap().unwrap(), caller_thread);
}

fn test_world() -> World {
    World::new_with_dimension_name(
        uuid::Uuid::new_v4(),
        spinel_registry::dimension_type::DimensionType::OVERWORLD,
        Identifier::minecraft("overworld"),
    )
}
