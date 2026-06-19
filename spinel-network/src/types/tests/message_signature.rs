use crate::data_type::DataType;
use crate::types::{MessageSignature, PackedMessageSignature};

#[test]
fn packed_message_signature_with_full_signature_roundtrips() {
    let packed_signature = PackedMessageSignature {
        signature_cache_id: -1,
        full_signature: Some(MessageSignature { bytes: [7; 256] }),
    };
    let mut payload = Vec::new();

    packed_signature.encode(&mut payload).unwrap();
    let decoded_signature = PackedMessageSignature::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_signature, packed_signature);
}

#[test]
fn packed_message_signature_with_cache_id_roundtrips() {
    let packed_signature = PackedMessageSignature {
        signature_cache_id: 12,
        full_signature: None,
    };
    let mut payload = Vec::new();

    packed_signature.encode(&mut payload).unwrap();
    let decoded_signature = PackedMessageSignature::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_signature, packed_signature);
}
