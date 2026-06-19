use super::super::set_score::SetScorePacket;
use spinel_network::data_type::DataType;
use spinel_network::types::ScoreNumberFormat;
use spinel_utils::component::text::TextComponent;

#[test]
fn set_score_packet_roundtrips() {
    let packet = SetScorePacket {
        owner: "Player".to_owned(),
        objective_name: "kills".to_owned(),
        score: 42,
        display: Some(TextComponent::literal("Shown")),
        number_format: Some(ScoreNumberFormat::Fixed {
            value: TextComponent::literal("42"),
        }),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SetScorePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
