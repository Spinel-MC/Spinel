use super::super::features::FeaturesPacket;
use spinel_network::DataType;
use spinel_network::types::{Array, Identifier};

#[test]
fn features_packet_roundtrips_enabled_feature_identifiers() {
    let packet = FeaturesPacket {
        features: Array(vec![
            Identifier::minecraft("vanilla"),
            Identifier::minecraft("trade_rebalance"),
        ]),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = FeaturesPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.features.0.len(), 2);
    assert!(
        decoded_packet
            .features
            .0
            .contains(&Identifier::minecraft("vanilla"))
    );
    assert!(
        decoded_packet
            .features
            .0
            .contains(&Identifier::minecraft("trade_rebalance"))
    );
}
