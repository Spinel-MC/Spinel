use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct PacketField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Deserialize, Clone)]
pub struct PacketEntry {
    pub id: String,
    pub fields: Option<Vec<PacketField>>,
}

#[derive(Deserialize)]
pub struct PacketsJson {
    pub serverbound: HashMap<String, HashMap<String, PacketEntry>>,
    pub clientbound: HashMap<String, HashMap<String, PacketEntry>>,
}

pub fn resolve_packet_entry(
    name: &str,
    state: Option<&str>,
    recipient: &str,
) -> Option<PacketEntry> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let packets_path =
        Path::new(&manifest_dir).join("../spinel-registry/build_assets/packets.json");

    if !packets_path.exists() {
        panic!("packets.json not found at {:?}", packets_path);
    }

    let file = std::fs::File::open(packets_path).expect("Failed to open packets.json");
    let reader = std::io::BufReader::new(file);
    let packets: PacketsJson =
        serde_json::from_reader(reader).expect("Failed to parse packets.json");

    let direction = match recipient {
        "Client" => &packets.clientbound,
        "Server" => &packets.serverbound,
        _ => panic!("Invalid recipient: {}", recipient),
    };

    if let Some(state_name) = state {
        let protocol_key = match state_name {
            "Handshaking" => "handshake",
            "Status" => "status",
            "Login" => "login",
            "Configuration" => "config",
            "Play" => "play",
            _ => return None,
        };

        if let Some(protocol) = direction.get(protocol_key) {
            if let Some(entry) = protocol.get(name) {
                return Some(entry.clone());
            }
        }
        return None;
    }

    None
}

pub fn extract_state_string(state_expr: &Option<syn::Expr>) -> Option<String> {
    state_expr.as_ref().and_then(|expr| {
        if let syn::Expr::Path(expr_path) = expr {
            expr_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident.to_string())
        } else {
            None
        }
    })
}

pub fn extract_recipient_string(recipient_expr: &Option<syn::Expr>) -> Option<String> {
    recipient_expr.as_ref().and_then(|expr| {
        if let syn::Expr::Path(expr_path) = expr {
            expr_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident.to_string())
        } else {
            None
        }
    })
}
