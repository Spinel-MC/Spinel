use spinel::network::Recipient;
use spinel::{
    client::{MinecraftClient, events::network::packet::PacketEvent},
    macros::event_listener,
};

use crate::events::info::packet_filter::packet_is_filtered;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

static RECEIVED_CHUNK_COUNT: AtomicUsize = AtomicUsize::new(0);
static RECEIVED_CHUNK_BYTES: AtomicUsize = AtomicUsize::new(0);
static FIRST_RECEIVED_CHUNK_AT: OnceLock<Mutex<Option<Instant>>> = OnceLock::new();

#[event_listener]
fn on_inbound_packet(event: &mut PacketEvent, _client: &mut MinecraftClient) {
    if event.recipient == Recipient::Client && event.packet_name == "level_chunk_with_light" {
        RECEIVED_CHUNK_COUNT.fetch_add(1, Ordering::Relaxed);
        RECEIVED_CHUNK_BYTES.fetch_add(event.payload_size, Ordering::Relaxed);
        if let Ok(mut first_received_chunk_at) = FIRST_RECEIVED_CHUNK_AT
            .get_or_init(|| Mutex::new(None))
            .lock()
        {
            first_received_chunk_at.get_or_insert_with(Instant::now);
        }
    }
    if event.recipient != Recipient::Client || packet_is_filtered(&event.packet_name) {
        return;
    }

    println!(
        "[Clientbound]: State={:?}, ID=\"{}\"",
        event.state, event.packet_name,
    );
}

pub fn received_chunk_metrics() -> (usize, usize, Duration) {
    let received_chunk_count = RECEIVED_CHUNK_COUNT.load(Ordering::Relaxed);
    let received_chunk_bytes = RECEIVED_CHUNK_BYTES.load(Ordering::Relaxed);
    let elapsed = FIRST_RECEIVED_CHUNK_AT
        .get_or_init(|| Mutex::new(None))
        .lock()
        .ok()
        .and_then(|first_received_chunk_at| *first_received_chunk_at)
        .map(|first_received_chunk_at| first_received_chunk_at.elapsed())
        .unwrap_or_default();
    (received_chunk_count, received_chunk_bytes, elapsed)
}
