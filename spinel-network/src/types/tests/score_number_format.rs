use crate::data_type::DataType;
use crate::types::ScoreNumberFormat;
use spinel_utils::component::text::TextComponent;

#[test]
fn fixed_score_number_format_roundtrips() {
    let number_format = ScoreNumberFormat::Fixed {
        value: TextComponent::literal("Score"),
    };
    let mut payload = Vec::new();

    number_format.encode(&mut payload).unwrap();
    let decoded_number_format = ScoreNumberFormat::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_number_format, number_format);
}

#[test]
fn blank_score_number_format_roundtrips() {
    let number_format = ScoreNumberFormat::Blank;
    let mut payload = Vec::new();

    number_format.encode(&mut payload).unwrap();
    let decoded_number_format = ScoreNumberFormat::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_number_format, number_format);
}
