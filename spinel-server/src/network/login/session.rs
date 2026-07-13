use serde::Deserialize;
use sha1::{Digest, Sha1};
use spinel_network::types::game_profile::{GameProfile, GameProfileProperty};
use uuid::Uuid;

const SESSION_SERVER_HAS_JOINED_URL: &str =
    "https://sessionserver.mojang.com/session/minecraft/hasJoined";

#[derive(Deserialize)]
struct MojangSessionProfile {
    id: String,
    name: String,
    #[serde(default)]
    properties: Vec<MojangSessionProperty>,
}

#[derive(Deserialize)]
struct MojangSessionProperty {
    name: String,
    value: String,
    signature: Option<String>,
}

pub(crate) struct MojangSessionVerifier;

impl MojangSessionVerifier {
    pub(crate) fn verify_joined_profile(
        username: &str,
        shared_secret: &[u8],
        public_key_der: &[u8],
    ) -> Option<GameProfile> {
        let server_hash = minecraft_session_hash(shared_secret, public_key_der);
        let session_profile = reqwest::blocking::Client::new()
            .get(SESSION_SERVER_HAS_JOINED_URL)
            .query(&[("username", username), ("serverId", server_hash.as_str())])
            .send()
            .ok()?;
        if !session_profile.status().is_success() {
            return None;
        }
        let session_profile = session_profile.json::<MojangSessionProfile>().ok()?;
        let profile_uuid = uuid_from_undashed_hex(&session_profile.id)?;
        let mut profile_properties = Vec::new();
        for property in session_profile.properties {
            profile_properties.push(GameProfileProperty {
                name: property.name,
                value: property.value,
                signature: property.signature,
            });
        }
        Some(GameProfile {
            uuid: profile_uuid,
            username: session_profile.name,
            properties: profile_properties,
        })
    }
}

fn minecraft_session_hash(shared_secret: &[u8], public_key_der: &[u8]) -> String {
    let mut digest = Sha1::new();
    digest.update([]);
    digest.update(shared_secret);
    digest.update(public_key_der);
    let hash = digest.finalize();
    signed_hex_digest(&hash)
}

pub(crate) fn signed_hex_digest(hash: &[u8]) -> String {
    if hash.iter().all(|byte| *byte == 0) {
        return "0".to_owned();
    }
    let hash_is_negative = hash
        .first()
        .is_some_and(|first_byte| first_byte & 0x80 != 0);
    if !hash_is_negative {
        return unsigned_hex_without_leading_zeroes(hash);
    }
    let magnitude = twos_complement_magnitude(hash);
    format!("-{}", unsigned_hex_without_leading_zeroes(&magnitude))
}

fn unsigned_hex_without_leading_zeroes(bytes: &[u8]) -> String {
    let first_non_zero_index = bytes
        .iter()
        .position(|byte| *byte != 0)
        .unwrap_or(bytes.len().saturating_sub(1));
    let mut hex = String::new();
    for byte in &bytes[first_non_zero_index..] {
        hex.push_str(&format!("{byte:02x}"));
    }
    hex.trim_start_matches('0').to_owned()
}

fn twos_complement_magnitude(hash: &[u8]) -> Vec<u8> {
    let mut magnitude = Vec::with_capacity(hash.len());
    for byte in hash {
        magnitude.push(!byte);
    }
    for byte in magnitude.iter_mut().rev() {
        let new_byte = byte.wrapping_add(1);
        *byte = new_byte;
        if new_byte != 0 {
            break;
        }
    }
    magnitude
}

fn uuid_from_undashed_hex(uuid_hex: &str) -> Option<Uuid> {
    if uuid_hex.len() != 32 {
        return None;
    }
    let uuid_text = format!(
        "{}-{}-{}-{}-{}",
        &uuid_hex[0..8],
        &uuid_hex[8..12],
        &uuid_hex[12..16],
        &uuid_hex[16..20],
        &uuid_hex[20..32]
    );
    Uuid::parse_str(&uuid_text).ok()
}
