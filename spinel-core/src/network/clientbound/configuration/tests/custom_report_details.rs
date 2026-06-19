use super::super::custom_report_details::CustomReportDetailsPacket;
use spinel_network::DataType;
use std::collections::HashMap;

#[test]
fn custom_report_details_packet_roundtrips_bounded_detail_entries() {
    let packet = CustomReportDetailsPacket {
        details: HashMap::from([
            ("environment".to_string(), "production".to_string()),
            ("build".to_string(), "1.21.11".to_string()),
        ]),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = CustomReportDetailsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(
        decoded_packet.details.get("environment"),
        Some(&"production".to_string())
    );
    assert_eq!(
        decoded_packet.details.get("build"),
        Some(&"1.21.11".to_string())
    );
}
