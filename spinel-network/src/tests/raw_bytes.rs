use crate::{DataType, RawBytes};
use std::io::{Cursor, Read};

#[test]
fn raw_bytes_decode_consumes_remaining_reader_bytes() {
    let payload = vec![1, 2, 3, 4];
    let mut reader = Cursor::new(payload.clone());
    let decoded = RawBytes::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();

    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(decoded, RawBytes(payload));
    assert!(remaining.is_empty());
}

#[test]
fn raw_bytes_encode_writes_payload_without_length_prefix() {
    let mut encoded = Vec::new();
    RawBytes(vec![0xAA, 0xBB, 0xCC])
        .encode(&mut encoded)
        .unwrap();
    assert_eq!(encoded, vec![0xAA, 0xBB, 0xCC]);
}
