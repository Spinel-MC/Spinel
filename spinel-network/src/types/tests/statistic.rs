use crate::data_type::DataType;
use crate::types::Statistic;

#[test]
fn statistic_roundtrips() {
    let statistic = Statistic {
        stat_type_id: 3,
        stat_value_id: 17,
    };
    let mut payload = Vec::new();

    statistic.encode(&mut payload).unwrap();
    let decoded_statistic = Statistic::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_statistic, statistic);
}
