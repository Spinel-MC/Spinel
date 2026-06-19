use spinel_macros::packet;

#[packet(id: "bundle_delimiter", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct BundleDelimiterPacket;
