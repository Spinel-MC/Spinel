use crate::data_type::DataType;
use crate::types::{
    BitSet, FilterMask, FilterMaskType, MessageSignature, PackedLastSeenMessages,
    PackedMessageSignature, SignedMessageBodyPacked,
};

#[test]
fn filter_mask_roundtrips() {
    let filter_mask = FilterMask {
        filter_mask_type: FilterMaskType::PartiallyFiltered,
        mask: BitSet(vec![0b0000_0011, 0, 0, 0, 0, 0, 0, 0]),
    };
    let mut payload = Vec::new();

    filter_mask.encode(&mut payload).unwrap();
    let decoded_filter_mask = FilterMask::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_filter_mask, filter_mask);
}

#[test]
fn signed_message_body_packed_roundtrips() {
    let body = SignedMessageBodyPacked {
        content: "hello".to_owned(),
        timestamp_millis: 1_718_000_000_123,
        salt: 77,
        last_seen: PackedLastSeenMessages {
            entries: vec![
                PackedMessageSignature {
                    signature_cache_id: 5,
                    full_signature: None,
                },
                PackedMessageSignature {
                    signature_cache_id: -1,
                    full_signature: Some(MessageSignature { bytes: [9; 256] }),
                },
            ],
        },
    };
    let mut payload = Vec::new();

    body.encode(&mut payload).unwrap();
    let decoded_body = SignedMessageBodyPacked::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_body, body);
}
