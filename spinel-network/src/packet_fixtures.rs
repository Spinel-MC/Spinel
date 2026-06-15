use crate::{ConnectionState, PacketCodecRegistry, PacketNameRegistry, Recipient};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::Path;

const PROBE_SIZE: usize = 1_024;
const PROBE_MUTATION_LIMIT: usize = 512;
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
            let fields = packet_fields(&packet_definitions, direction, state, &resource_id);
            let payloads = generate_payloads(codec.decode_then_encode).unwrap_or_default();

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
