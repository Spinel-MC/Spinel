use ::spinel_macros::packet;
use ::spinel_network::types::Array;

#[packet(
    id: "update_enabled_features",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client,
    generate_fields: true
)]
pub struct FeaturesPacket {
    pub features: Array<Identifier>,
}

impl FeaturesPacket {
    pub fn new(features: Vec<String>) -> Self {
        Self {
            features: Array(
                features
                    .into_iter()
                    .map(|s| s.parse().expect("Invalid feature identifier"))
                    .collect(),
            ),
        }
    }

    pub fn vanilla() -> Self {
        Self::new(vec!["minecraft:vanilla".to_string()])
    }
}
