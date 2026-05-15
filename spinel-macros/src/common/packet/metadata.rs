use serde::Deserialize;
use serde_json::Value;
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
    #[allow(dead_code)]
    pub version: Option<u32>,
    #[allow(dead_code)]
    pub types: Option<HashMap<String, Value>>,
    pub packets: Option<PacketDirections>,
    pub serverbound: Option<HashMap<String, HashMap<String, PacketEntry>>>,
    pub clientbound: Option<HashMap<String, HashMap<String, PacketEntry>>>,
}

#[derive(Deserialize)]
pub struct PacketDirections {
    pub serverbound: HashMap<String, HashMap<String, PacketEntry>>,
    pub clientbound: HashMap<String, HashMap<String, PacketEntry>>,
}

impl PacketsJson {
    fn serverbound(&self) -> Option<&HashMap<String, HashMap<String, PacketEntry>>> {
        self.packets
            .as_ref()
            .map(|packets| &packets.serverbound)
            .or(self.serverbound.as_ref())
    }

    fn clientbound(&self) -> Option<&HashMap<String, HashMap<String, PacketEntry>>> {
        self.packets
            .as_ref()
            .map(|packets| &packets.clientbound)
            .or(self.clientbound.as_ref())
    }
}

pub struct PacketMetadataResolver;

impl PacketMetadataResolver {
    pub fn extract_recipient_string(recipient_expr: &Option<syn::Expr>) -> Option<String> {
        Self::extract_path_segment(recipient_expr.as_ref())
    }

    pub fn extract_state_string(state_expr: &Option<syn::Expr>) -> Option<String> {
        Self::extract_path_segment(state_expr.as_ref())
    }

    pub fn resolve_packet_entry(
        name: &str,
        state: Option<&str>,
        recipient: &str,
    ) -> Option<PacketEntry> {
        let packet_catalog = Self::read_packet_catalog()?;
        let direction = Self::select_direction(&packet_catalog, recipient)?;
        let protocol_key = Self::protocol_key(state?)?;
        let protocol = Self::select_protocol(direction, protocol_key)?;
        protocol.get(name).cloned()
    }

    fn extract_path_segment(expr: Option<&syn::Expr>) -> Option<String> {
        let syn::Expr::Path(expr_path) = expr? else {
            return None;
        };

        expr_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
    }

    fn protocol_key(state_name: &str) -> Option<&'static str> {
        match state_name {
            "Handshaking" => Some("handshake"),
            "Status" => Some("status"),
            "Login" => Some("login"),
            "Configuration" => Some("configuration"),
            "Play" => Some("play"),
            _ => None,
        }
    }

    fn read_packet_catalog() -> Option<PacketsJson> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok()?;
        let packets_path =
            Path::new(&manifest_dir).join("../spinel-registry/build_assets/packets.json");
        let file = std::fs::File::open(packets_path).ok()?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).ok()
    }

    fn select_direction<'a>(
        packets_json: &'a PacketsJson,
        recipient: &str,
    ) -> Option<&'a HashMap<String, HashMap<String, PacketEntry>>> {
        match recipient {
            "Client" => packets_json.clientbound(),
            "Server" => packets_json.serverbound(),
            _ => None,
        }
    }

    fn select_protocol<'a>(
        direction: &'a HashMap<String, HashMap<String, PacketEntry>>,
        protocol_key: &str,
    ) -> Option<&'a HashMap<String, PacketEntry>> {
        direction.get(protocol_key).or_else(|| {
            if protocol_key == "configuration" {
                return direction.get("config");
            }

            None
        })
    }
}
