use crate::data_type::DataType;
use crate::types::entity_metadata::MetadataValue;
use crate::{ConnectionState, PacketCodecRegistry, PacketNameRegistry, Recipient};
use serde::Serialize;
use serde_json::Value;
use spinel_utils::component::text::TextComponent;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::Path;

const PROBE_SIZE: usize = 1_024;
const PROBE_MUTATION_LIMIT: usize = 512;
const PROBE_COMBINATION_MUTATION_LIMIT: usize = 64;
const PAYLOAD_CANDIDATE_LIMIT: usize = 32;

#[derive(Debug, Serialize)]
pub struct PacketFixtureCatalog {
    pub version: u32,
    pub packets: BTreeMap<&'static str, BTreeMap<&'static str, BTreeMap<String, PacketFixture>>>,
}

#[derive(Debug, Serialize)]
pub struct PacketFixture {
    pub id: String,
    pub fields: Value,
    pub payloads: Vec<Vec<u8>>,
}

impl PacketFixtureCatalog {
    pub fn generate() -> Self {
        let packet_definitions = packet_definitions();
        let mut packets = BTreeMap::new();
        for codec in PacketCodecRegistry::codecs() {
            let direction = direction_name(codec.recipient);
            let state = state_name((codec.state)());
            let packet_id = (codec.id)();
            let resource_id = packet_resource_id(
                codec.recipient,
                (codec.state)(),
                packet_id,
                codec.packet_type,
            );
            if vanilla_protocol_excludes_packet(direction, state, &resource_id) {
                continue;
            }
            let fields = packet_fields(&packet_definitions, direction, state, &resource_id);
            let payloads =
                fixture_payloads(direction, state, &resource_id, codec.decode_then_encode)
                    .unwrap_or_default();

            packets
                .entry(direction)
                .or_insert_with(BTreeMap::new)
                .entry(state)
                .or_insert_with(BTreeMap::new)
                .insert(
                    resource_id,
                    PacketFixture {
                        id: format!("0x{packet_id:02X}"),
                        fields,
                        payloads,
                    },
                );
        }

        Self {
            version: protocol_version(&packet_definitions),
            packets,
        }
    }

    pub fn write(&self, path: &Path) -> io::Result<()> {
        let serialized_catalog = serde_json::to_vec_pretty(self)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        fs::write(path, serialized_catalog)
    }

    pub fn generation_failures(&self) -> Vec<String> {
        self.packets
            .iter()
            .flat_map(|(direction, states)| {
                states.iter().flat_map(move |(state, packets)| {
                    packets.iter().filter_map(move |(resource_id, fixture)| {
                        fixture
                            .payloads
                            .is_empty()
                            .then(|| format!("{direction}/{state}/{resource_id}"))
                    })
                })
            })
            .collect()
    }
}

fn generate_payloads(
    decode_then_encode: fn(&[u8]) -> io::Result<Vec<u8>>,
) -> io::Result<Vec<Vec<u8>>> {
    let mut probe = vec![0; PROBE_SIZE];
    let mut payloads = Vec::new();
    let mut unique_payloads = HashSet::new();
    collect_encoded_packet(
        &probe,
        decode_then_encode,
        &mut payloads,
        &mut unique_payloads,
    );

    let common_encoded_values = [
        length_prefixed_bytes(b"minecraft:overworld"),
        length_prefixed_bytes(b"{}"),
        length_prefixed_bytes(br#"{"text":""}"#),
        length_prefixed_bytes(b"minecraft:air"),
        vec![10, 0],
    ];
    for encoded_value in &common_encoded_values {
        for offset in 0..PROBE_MUTATION_LIMIT {
            if offset + encoded_value.len() > probe.len() {
                continue;
            }

            probe[offset..offset + encoded_value.len()].copy_from_slice(encoded_value);
            collect_encoded_packet(
                &probe,
                decode_then_encode,
                &mut payloads,
                &mut unique_payloads,
            );
            probe[offset..offset + encoded_value.len()].fill(0);

            if payloads.len() >= PAYLOAD_CANDIDATE_LIMIT {
                return Ok(payloads);
            }
        }
    }

    for first_encoded_value in &common_encoded_values {
        for first_offset in 0..PROBE_COMBINATION_MUTATION_LIMIT {
            if first_offset + first_encoded_value.len() > probe.len() {
                continue;
            }

            probe[first_offset..first_offset + first_encoded_value.len()]
                .copy_from_slice(first_encoded_value);

            for second_encoded_value in &common_encoded_values {
                for second_offset in 0..PROBE_COMBINATION_MUTATION_LIMIT {
                    if ranges_overlap(
                        first_offset,
                        first_offset + first_encoded_value.len(),
                        second_offset,
                        second_offset + second_encoded_value.len(),
                    ) {
                        continue;
                    }
                    if second_offset + second_encoded_value.len() > probe.len() {
                        continue;
                    }

                    probe[second_offset..second_offset + second_encoded_value.len()]
                        .copy_from_slice(second_encoded_value);
                    collect_encoded_packet(
                        &probe,
                        decode_then_encode,
                        &mut payloads,
                        &mut unique_payloads,
                    );
                    probe[second_offset..second_offset + second_encoded_value.len()].fill(0);

                    if payloads.len() >= PAYLOAD_CANDIDATE_LIMIT {
                        return Ok(payloads);
                    }
                }
            }

            probe[first_offset..first_offset + first_encoded_value.len()].fill(0);
        }
    }

    for offset in 0..PROBE_MUTATION_LIMIT {
        for value in 1..=255 {
            probe[offset] = value;
            collect_encoded_packet(
                &probe,
                decode_then_encode,
                &mut payloads,
                &mut unique_payloads,
            );
            if payloads.len() >= PAYLOAD_CANDIDATE_LIMIT {
                return Ok(payloads);
            }
        }
        probe[offset] = 0;
    }

    if payloads.is_empty() {
        return Err(decode_then_encode(&probe)
            .err()
            .unwrap_or_else(|| io::Error::other("codec produced no payload candidate")));
    }

    Ok(payloads)
}

fn fixture_payloads(
    direction: &str,
    state: &str,
    resource_id: &str,
    decode_then_encode: fn(&[u8]) -> io::Result<Vec<u8>>,
) -> io::Result<Vec<Vec<u8>>> {
    let canonical_payloads = canonical_fixture_payloads(direction, state, resource_id);
    if !canonical_payloads.is_empty() {
        return Ok(canonical_payloads);
    }

    generate_payloads(decode_then_encode)
}

fn canonical_fixture_payloads(direction: &str, state: &str, resource_id: &str) -> Vec<Vec<u8>> {
    match (direction, state, resource_id) {
        ("clientbound", "configuration", "show_dialog") => {
            vec![configuration_show_dialog_payload()]
        }
        ("clientbound", "play", "debug/block_value") => vec![debug_block_value_payload()],
        ("clientbound", "play", "debug/chunk_value") => vec![debug_chunk_value_payload()],
        ("clientbound", "play", "debug/event") => vec![debug_event_payload()],
        ("clientbound", "play", "disguised_chat") => vec![disguised_chat_payload()],
        ("clientbound", "play", "explode") => vec![explode_payload()],
        ("clientbound", "play", "mount_screen_open") => vec![mount_screen_open_payload()],
        ("clientbound", "play", "place_ghost_recipe") => vec![place_ghost_recipe_payload()],
        ("clientbound", "play", "player_chat") => vec![player_chat_payload()],
        ("clientbound", "play", "set_entity_data") => vec![set_entity_data_payload()],
        ("serverbound", "play", "chat_session_update") => vec![chat_session_update_payload()],
        ("serverbound", "play", "container_button_click") => {
            vec![container_button_click_payload()]
        }
        ("serverbound", "play", "container_slot_state_changed") => {
            vec![container_slot_state_changed_payload()]
        }
        ("serverbound", "play", "set_command_block") => vec![set_command_block_payload()],
        ("serverbound", "play", "set_structure_block") => vec![set_structure_block_payload()],
        _ => Vec::new(),
    }
}

fn vanilla_protocol_excludes_packet(direction: &str, state: &str, resource_id: &str) -> bool {
    direction == "serverbound" && state == "handshake" && resource_id == "legacy_server_list_ping"
}

fn collect_encoded_packet(
    probe: &[u8],
    decode_then_encode: fn(&[u8]) -> io::Result<Vec<u8>>,
    payloads: &mut Vec<Vec<u8>>,
    unique_payloads: &mut HashSet<Vec<u8>>,
) {
    let Ok(payload) = decode_then_encode(probe) else {
        return;
    };
    if unique_payloads.insert(payload.clone()) {
        payloads.push(payload);
    }
}

fn ranges_overlap(start_a: usize, end_a: usize, start_b: usize, end_b: usize) -> bool {
    start_a < end_b && start_b < end_a
}

fn configuration_show_dialog_payload() -> Vec<u8> {
    vec![
        10, 8, 0, 5, 116, 105, 116, 108, 101, 0, 5, 84, 105, 116, 108, 101, 8, 0, 4, 116, 121, 112,
        101, 0, 16, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 110, 111, 116, 105, 99, 101, 0,
    ]
}

fn debug_block_value_payload() -> Vec<u8> {
    let mut payload = vec![0; 8];
    payload.extend_from_slice(&debug_subscription_update_payload());
    payload
}

fn debug_chunk_value_payload() -> Vec<u8> {
    let mut payload = vec![0; 8];
    payload.extend_from_slice(&debug_subscription_update_payload());
    payload
}

fn debug_event_payload() -> Vec<u8> {
    encode_varint(10)
}

fn debug_subscription_update_payload() -> Vec<u8> {
    let mut payload = encode_varint(10);
    payload.push(0);
    payload
}

fn disguised_chat_payload() -> Vec<u8> {
    vec![
        8, 0, 6, 77, 97, 115, 107, 101, 100, 0, 14, 99, 104, 97, 116, 46, 116, 121, 112, 101, 46,
        116, 101, 120, 116, 2, 0, 2, 10, 0, 22, 99, 104, 97, 116, 46, 116, 121, 112, 101, 46, 116,
        101, 120, 116, 46, 110, 97, 114, 114, 97, 116, 101, 2, 0, 2, 10, 0, 8, 0, 6, 83, 101, 114,
        118, 101, 114, 1, 8, 0, 6, 84, 97, 114, 103, 101, 116,
    ]
}

fn place_ghost_recipe_payload() -> Vec<u8> {
    vec![0, 3, 0, 0, 0]
}

fn explode_payload() -> Vec<u8> {
    let mut payload = vec![0; 24];
    payload.extend_from_slice(&0.0f32.to_be_bytes());
    payload.extend_from_slice(&0i32.to_be_bytes());
    payload.push(0);
    payload.extend_from_slice(&encode_varint(23));
    payload.extend_from_slice(&encode_varint(669));
    payload.extend_from_slice(&encode_varint(0));
    payload
}

fn mount_screen_open_payload() -> Vec<u8> {
    let mut payload = encode_varint(0);
    payload.extend_from_slice(&encode_varint(0));
    payload.extend_from_slice(&0i32.to_be_bytes());
    payload
}

fn container_button_click_payload() -> Vec<u8> {
    let mut payload = encode_varint(0);
    payload.extend_from_slice(&encode_varint(0));
    payload
}

fn container_slot_state_changed_payload() -> Vec<u8> {
    let mut payload = encode_varint(0);
    payload.extend_from_slice(&encode_varint(0));
    payload.push(0);
    payload
}

fn set_entity_data_payload() -> Vec<u8> {
    let mut payload = encode_varint(7);
    payload.push(2);
    if MetadataValue::OptionalText(Some(TextComponent::literal("Minestom Physics")))
        .encode(&mut payload)
        .is_err()
    {
        return Vec::new();
    }
    payload.push(3);
    if MetadataValue::Boolean(true).encode(&mut payload).is_err() {
        return Vec::new();
    }
    payload.push(0xFF);
    payload
}

fn player_chat_payload() -> Vec<u8> {
    let mut payload = encode_varint(0);
    payload.extend_from_slice(&[0; 16]);
    payload.extend_from_slice(&encode_varint(0));
    payload.push(0);
    payload.extend_from_slice(&encode_varint(0));
    payload.extend_from_slice(&[0; 8]);
    payload.extend_from_slice(&[0; 8]);
    payload.extend_from_slice(&encode_varint(0));
    payload.push(0);
    payload.extend_from_slice(&encode_varint(0));
    payload.extend_from_slice(&chat_type_payload());
    payload
}

fn chat_session_update_payload() -> Vec<u8> {
    let mut payload = vec![0; 16];
    payload.extend_from_slice(&[0, 0, 1, 149, 52, 107, 116, 0]);
    payload.extend_from_slice(&length_prefixed_varint_bytes(&[
        48, 129, 159, 48, 13, 6, 9, 42, 134, 72, 134, 247, 13, 1, 1, 1, 5, 0, 3, 129, 141, 0, 48,
        129, 137, 2, 129, 129, 0, 187, 21, 253, 162, 3, 35, 13, 156, 17, 117, 4, 250, 124, 165, 40,
        129, 119, 122, 135, 217, 7, 43, 143, 218, 98, 154, 137, 195, 110, 244, 65, 17, 175, 176,
        166, 210, 66, 210, 5, 43, 100, 56, 216, 23, 119, 62, 70, 218, 218, 102, 104, 136, 169, 84,
        170, 210, 167, 15, 160, 252, 168, 173, 60, 24, 63, 144, 198, 206, 81, 133, 85, 199, 63, 34,
        31, 58, 124, 235, 56, 175, 103, 166, 100, 44, 212, 193, 121, 133, 73, 38, 250, 95, 213,
        152, 215, 61, 225, 26, 186, 92, 147, 103, 255, 187, 199, 128, 61, 225, 173, 248, 94, 190,
        111, 172, 57, 21, 179, 172, 115, 170, 158, 30, 2, 170, 220, 156, 48, 183, 2, 3, 1, 0, 1,
    ]));
    payload.push(0);
    payload
}

fn set_command_block_payload() -> Vec<u8> {
    let mut payload = vec![0; 8];
    payload.push(0);
    payload.push(0);
    payload.push(0);
    payload
}

fn set_structure_block_payload() -> Vec<u8> {
    let mut payload = vec![0; 8];
    payload.push(0);
    payload.push(0);
    payload.push(0);
    payload.extend_from_slice(&[0, 0, 0]);
    payload.extend_from_slice(&[0, 0, 0]);
    payload.push(0);
    payload.push(0);
    payload.push(0);
    payload.extend_from_slice(&0.0f32.to_be_bytes());
    payload.push(0);
    payload.push(0);
    payload
}

fn chat_type_payload() -> Vec<u8> {
    vec![
        0, 14, 99, 104, 97, 116, 46, 116, 121, 112, 101, 46, 116, 101, 120, 116, 2, 0, 2, 10, 0,
        22, 99, 104, 97, 116, 46, 116, 121, 112, 101, 46, 116, 101, 120, 116, 46, 110, 97, 114,
        114, 97, 116, 101, 2, 0, 2, 10, 0, 8, 0, 6, 83, 101, 114, 118, 101, 114, 1, 8, 0, 6, 84,
        97, 114, 103, 101, 116,
    ]
}

fn encode_varint(value: i32) -> Vec<u8> {
    let mut remaining_bits = value as u32;
    let mut encoded = Vec::new();

    loop {
        if remaining_bits & !0x7F == 0 {
            encoded.push(remaining_bits as u8);
            return encoded;
        }

        encoded.push(((remaining_bits & 0x7F) as u8) | 0x80);
        remaining_bits >>= 7;
    }
}

fn length_prefixed_varint_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = encode_varint(bytes.len() as i32);
    encoded.extend_from_slice(bytes);
    encoded
}

fn length_prefixed_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(bytes.len() + 1);
    encoded.push(bytes.len() as u8);
    encoded.extend_from_slice(bytes);
    encoded
}

fn direction_name(recipient: Recipient) -> &'static str {
    match recipient {
        Recipient::Server => "serverbound",
        Recipient::Client => "clientbound",
    }
}

fn state_name(state: ConnectionState) -> &'static str {
    match state {
        ConnectionState::Handshaking => "handshake",
        ConnectionState::Status => "status",
        ConnectionState::Login => "login",
        ConnectionState::Configuration => "configuration",
        ConnectionState::Play => "play",
    }
}

fn packet_name(recipient: Recipient, state: ConnectionState, packet_id: i32) -> String {
    match recipient {
        Recipient::Server => PacketNameRegistry::get_serverbound_packet_name(state, packet_id),
        Recipient::Client => PacketNameRegistry::get_clientbound_packet_name(state, packet_id),
    }
}

fn packet_resource_id(
    recipient: Recipient,
    state: ConnectionState,
    packet_id: i32,
    packet_type: &str,
) -> String {
    let packet_name = packet_name(recipient, state, packet_id);
    if packet_name != "Unknown" {
        return packet_name;
    }

    packet_type
        .strip_suffix("Packet")
        .unwrap_or(packet_type)
        .chars()
        .enumerate()
        .fold(String::new(), |mut resource_id, (index, character)| {
            if character.is_ascii_uppercase() && index > 0 {
                resource_id.push('_');
            }
            resource_id.push(character.to_ascii_lowercase());
            resource_id
        })
}

fn packet_definitions() -> Value {
    serde_json::from_str(include_str!("../../spinel-registry/assets/packets.json"))
        .unwrap_or(Value::Null)
}

fn protocol_version(packet_definitions: &Value) -> u32 {
    packet_definitions
        .get("version")
        .and_then(Value::as_u64)
        .unwrap_or_default() as u32
}

fn packet_fields(
    packet_definitions: &Value,
    direction: &str,
    state: &str,
    resource_id: &str,
) -> Value {
    packet_definitions
        .pointer(&format!(
            "/packets/{direction}/{state}/{resource_id}/fields"
        ))
        .cloned()
        .unwrap_or_else(|| Value::Array(Vec::new()))
}
