use crate::world::{
    Block, BlockComparison, BlockHandler, BlockHandlerDestroy, BlockHandlerHandle,
    BlockHandlerPlacement, BlockHandlerRegistry, BlockHandlerTick, BlockInstance,
    BlockLookupCondition, BlockPosition, Chunk, ChunkPosition, StoredBlockInstance,
};
use spinel_nbt::{Nbt, NbtCompound, Tag, TagReadable};
use spinel_network::types::Identifier;
use std::collections::HashSet;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[test]
fn block_instance_preserves_nbt_and_handler_through_property_changes() {
    let handler = BlockHandlerHandle::new(CountingHandler::new(
        "property",
        Arc::new(AtomicUsize::new(0)),
    ));
    let mut nbt = NbtCompound::new();
    nbt.insert("custom".to_string(), Nbt::Int(4));
    let block_instance = BlockInstance::from(Block::OAK_LOG)
        .with_nbt(Some(nbt.clone()))
        .with_handler(Some(handler.clone()))
        .with_property("axis", "x")
        .unwrap();

    assert_eq!(block_instance.block_state().property("axis"), Some("x"));
    assert_eq!(block_instance.nbt(), Some(&nbt));
    assert!(
        block_instance
            .handler()
            .is_some_and(|stored_handler| stored_handler.is_same_handler(&handler))
    );

    let default_state = block_instance.default_state();
    assert_eq!(default_state.block_state(), Block::OAK_LOG.default_state());
    assert!(!default_state.has_nbt());
    assert!(default_state.handler().is_none());
}

#[test]
fn chunk_conditional_lookup_preserves_full_cached_block_instance() {
    let handler = BlockHandlerHandle::new(CountingHandler::new(
        "cached",
        Arc::new(AtomicUsize::new(0)),
    ));
    let mut nbt = NbtCompound::new();
    nbt.insert("custom".to_string(), Nbt::String("value".to_string()));
    let block_instance = BlockInstance::from(Block::CHEST)
        .with_nbt(Some(nbt.clone()))
        .with_handler(Some(handler.clone()));
    let position = BlockPosition::new(1, 64, 1);
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

    assert!(chunk.set_block_instance(position, block_instance.clone()));

    let cached = chunk
        .block_instance_with_condition(position, BlockLookupCondition::Cached)
        .unwrap();
    let typed = chunk
        .block_instance_with_condition(position, BlockLookupCondition::Type)
        .unwrap();

    assert_eq!(cached, block_instance);
    assert_eq!(cached.nbt(), Some(&nbt));
    assert!(cached.handler().is_some());
    assert_eq!(typed.block(), Block::CHEST);
    assert!(!typed.has_nbt());
    assert!(typed.handler().is_none());
    assert_eq!(
        chunk
            .copy_for_position(ChunkPosition::new(4, 5))
            .block_instance_with_condition(position, BlockLookupCondition::Cached),
        Some(cached)
    );
}

#[test]
fn same_type_block_instances_keep_distinct_handlers_and_callback_order() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let first_ticks = Arc::new(AtomicUsize::new(0));
    let second_ticks = Arc::new(AtomicUsize::new(0));
    let first = BlockInstance::from(Block::CHEST).with_new_handler(RecordingHandler::new(
        "first",
        Arc::clone(&events),
        Arc::clone(&first_ticks),
    ));
    let second = BlockInstance::from(Block::CHEST).with_new_handler(RecordingHandler::new(
        "second",
        Arc::clone(&events),
        Arc::clone(&second_ticks),
    ));
    let first_position = BlockPosition::new(1, 64, 1);
    let second_position = BlockPosition::new(2, 64, 1);
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));

    assert!(chunk.set_block_instance(first_position, first));
    assert!(chunk.set_block_instance(second_position, second.clone()));
    assert!(chunk.set_block_instance(first_position, second));
    assert_eq!(
        *events.lock().unwrap(),
        vec![
            "first:place",
            "second:place",
            "first:destroy",
            "second:place"
        ]
    );

    assert_eq!(
        chunk.tick(uuid::Uuid::nil(), &BlockHandlerRegistry::default(), 1),
        2
    );
    assert_eq!(first_ticks.load(Ordering::SeqCst), 0);
    assert_eq!(second_ticks.load(Ordering::SeqCst), 2);
}

#[test]
fn handler_registry_suppliers_and_dummy_handlers_preserve_namespace_identity() {
    let key = Identifier::minecraft("supplier");
    let missing_key = Identifier::minecraft("missing");
    let mut registry = BlockHandlerRegistry::default();
    registry.register_supplier(key.clone(), || {
        CountingHandler::new("supplier", Arc::new(AtomicUsize::new(0)))
    });

    let first = registry.handler(&key).unwrap();
    let second = registry.handler(&key).unwrap();
    let first_dummy = registry.handler_or_dummy(&missing_key);
    let second_dummy = registry.handler_or_dummy(&missing_key);

    assert_eq!(first.key(), &key);
    assert_eq!(second.key(), &key);
    assert!(!first.is_same_handler(&second));
    assert_eq!(first_dummy.key(), &missing_key);
    assert!(first_dummy.is_same_handler(&second_dummy));
}

#[test]
fn block_instance_exposes_immutable_block_registry_and_state_surface() {
    let custom_tag = Tag::integer("custom");
    let block_instance = BlockInstance::from(Block::OAK_STAIRS)
        .with_properties([("facing", "east"), ("half", "top")])
        .unwrap()
        .with_tag(&custom_tag, Some(7));
    let state = block_instance.state();
    let parsed = BlockInstance::from_state(&state).unwrap();
    let cloned = block_instance.clone();

    assert_eq!(parsed.block_state(), block_instance.block_state());
    assert_eq!(block_instance.property("facing"), Some("east"));
    assert_eq!(block_instance.get_tag(&custom_tag), Some(7));
    assert!(
        block_instance
            .clone()
            .with_tag(&custom_tag, None)
            .get_tag(&custom_tag)
            .is_none()
    );
    assert_eq!(
        BlockInstance::from_key("minecraft:oak_stairs").map(|block| block.id()),
        Some(Block::OAK_STAIRS.id())
    );
    assert_eq!(
        BlockInstance::from_id(Block::OAK_STAIRS.id()).map(|block| block.block()),
        Some(Block::OAK_STAIRS)
    );
    assert_eq!(
        BlockInstance::from_state_id(block_instance.state_id()).map(|block| block.block_state()),
        Some(block_instance.block_state())
    );
    assert!(
        block_instance
            .possible_states()
            .into_iter()
            .all(|possible_state| {
                possible_state.block() == Block::OAK_STAIRS
                    && !possible_state.has_nbt()
                    && possible_state.handler().is_none()
            })
    );
    assert!(block_instance.compare(&block_instance, BlockComparison::Identity));
    assert!(!block_instance.compare(&cloned, BlockComparison::Identity));
    assert!(block_instance.compare(&cloned, BlockComparison::BlockType));
    assert!(block_instance.compare(&cloned, BlockComparison::BlockState));
    assert!(HashSet::from([block_instance.clone()]).contains(&cloned));
    assert!(BlockInstance::from(Block::AIR).is_air());
    assert!(BlockInstance::from(Block::STONE).is_solid());
    assert!(BlockInstance::from(Block::WATER).is_liquid());
    assert_eq!(block_instance.key(), Identifier::minecraft("oak_stairs"));
}

#[test]
fn stored_block_instance_round_trip_restores_registered_and_unknown_handlers() {
    let registered_key = Identifier::minecraft("registered");
    let unknown_key = Identifier::minecraft("unknown");
    let mut handlers = BlockHandlerRegistry::default();
    handlers.register_supplier(registered_key.clone(), || {
        CountingHandler::new("registered", Arc::new(AtomicUsize::new(0)))
    });
    let mut registered_nbt = NbtCompound::new();
    registered_nbt.insert("custom".to_string(), Nbt::Int(3));
    let registered = BlockInstance::from(Block::CHEST)
        .with_handler(handlers.handler(&registered_key))
        .with_nbt(Some(registered_nbt));
    let unknown_handler = handlers.handler_or_dummy(&unknown_key);
    let unknown = BlockInstance::from(Block::BARREL).with_handler(Some(unknown_handler.clone()));

    let stored_registered = StoredBlockInstance::from(&registered);
    let stored_unknown = StoredBlockInstance::from(&unknown);
    let restored_registered = stored_registered.clone().restore(&mut handlers);
    let restored_unknown = stored_unknown.clone().restore(&mut handlers);

    assert_eq!(stored_registered.handler_key(), Some(&registered_key));
    assert_eq!(stored_unknown.handler_key(), Some(&unknown_key));
    assert_eq!(restored_registered.block_state(), registered.block_state());
    assert_eq!(restored_registered.nbt(), registered.nbt());
    assert_eq!(
        restored_registered.handler().map(|handler| handler.key()),
        Some(&registered_key)
    );
    assert!(
        restored_unknown
            .handler()
            .is_some_and(|handler| handler.is_same_handler(&unknown_handler))
    );
}

struct CountingHandler {
    key: Identifier,
    ticks: Arc<AtomicUsize>,
}

impl CountingHandler {
    fn new(key: &str, ticks: Arc<AtomicUsize>) -> Self {
        Self {
            key: Identifier::minecraft(key),
            ticks,
        }
    }
}

impl BlockHandler for CountingHandler {
    fn key(&self) -> &Identifier {
        &self.key
    }

    fn tick(&self, _tick: BlockHandlerTick) {
        self.ticks.fetch_add(1, Ordering::SeqCst);
    }

    fn is_tickable(&self) -> bool {
        true
    }
}

struct RecordingHandler {
    key: Identifier,
    name: &'static str,
    events: Arc<Mutex<Vec<&'static str>>>,
    ticks: Arc<AtomicUsize>,
}

impl RecordingHandler {
    fn new(
        name: &'static str,
        events: Arc<Mutex<Vec<&'static str>>>,
        ticks: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            key: Identifier::minecraft(name),
            name,
            events,
            ticks,
        }
    }

    fn event(&self, action: &'static str) -> &'static str {
        match (self.name, action) {
            ("first", "place") => "first:place",
            ("first", "destroy") => "first:destroy",
            ("second", "place") => "second:place",
            ("second", "destroy") => "second:destroy",
            _ => "unknown",
        }
    }
}

impl BlockHandler for RecordingHandler {
    fn key(&self) -> &Identifier {
        &self.key
    }

    fn on_place(&self, _placement: BlockHandlerPlacement) {
        let event = self.event("place");
        self.events.lock().unwrap().push(event);
    }

    fn on_destroy(&self, _destroy: BlockHandlerDestroy) {
        let event = self.event("destroy");
        self.events.lock().unwrap().push(event);
    }

    fn tick(&self, _tick: BlockHandlerTick) {
        self.ticks.fetch_add(1, Ordering::SeqCst);
    }

    fn is_tickable(&self) -> bool {
        true
    }
}
