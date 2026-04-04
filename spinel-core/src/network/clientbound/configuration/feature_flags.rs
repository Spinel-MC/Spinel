use spinel_macros::packet;

#[packet(id: "update_enabled_features", generate_fields: true, state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct FeatureFlagsPacket;
