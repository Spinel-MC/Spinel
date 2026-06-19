use super::super::award_stats::{AwardStatValue, AwardStatsPacket};
use spinel_network::data_type::DataType;
use spinel_network::types::Statistic;

#[test]
fn award_stats_packet_roundtrips() {
    let packet = AwardStatsPacket {
        stats: vec![
            AwardStatValue {
                statistic: Statistic {
                    stat_type_id: 0,
                    stat_value_id: 42,
                },
                value: 7,
            },
            AwardStatValue {
                statistic: Statistic {
                    stat_type_id: 8,
                    stat_value_id: 5,
                },
                value: 99,
            },
        ],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = AwardStatsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
