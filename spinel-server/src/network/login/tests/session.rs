use crate::network::login::session::signed_hex_digest;

#[test]
fn minecraft_session_hash_formats_positive_big_integer_without_leading_zeroes() {
    let hash = [0x00, 0x00, 0x12, 0x34];

    assert_eq!(signed_hex_digest(&hash), "1234");
}

#[test]
fn minecraft_session_hash_formats_negative_big_integer_like_java() {
    let hash = [0xff, 0xff, 0xff, 0xfe];

    assert_eq!(signed_hex_digest(&hash), "-2");
}
