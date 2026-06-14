use spinel_macros::packet;
use spinel_network::types::{Array, Identifier};

#[packet(
    id: "update_enabled_features",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client,
    generate_fields: false
)]
pub struct FeaturesPacket {
    pub features: Array<Identifier>,
}

impl FeaturesPacket {
    pub fn new(features: Vec<String>) -> Result<Self, String> {
        let feature_identifiers = features
            .into_iter()
            .map(|feature_identifier| feature_identifier.parse())
            .collect::<Result<Vec<Identifier>, String>>()?;

        Ok(Self {
            features: Array(feature_identifiers),
        })
    }

    pub fn vanilla() -> Self {
        Self {
            features: Array(vec![Identifier::minecraft("vanilla")]),
        }
    }
}
