use crate::data_type::DataType;
use crate::types::ExactDataComponentPredicate;

#[test]
fn exact_data_component_predicate_roundtrips() {
    let predicate = ExactDataComponentPredicate::default();
    let mut payload = Vec::new();

    predicate.encode(&mut payload).unwrap();
    let decoded_predicate = ExactDataComponentPredicate::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_predicate, predicate);
}
