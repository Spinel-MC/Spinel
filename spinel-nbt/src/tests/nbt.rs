use crate::{Nbt, NbtCompound, Tag, TagHandler, TagReadable, TagWritable};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use uuid::Uuid;

#[test]
fn all_named_and_unnamed_nbt_tags_round_trip() {
    let mut compound = NbtCompound::new();
    compound.insert("byte".to_string(), Nbt::Byte(1));
    compound.insert("short".to_string(), Nbt::Short(2));
    compound.insert("int".to_string(), Nbt::Int(3));
    compound.insert("long".to_string(), Nbt::Long(4));
    compound.insert("float".to_string(), Nbt::Float(5.0));
    compound.insert("double".to_string(), Nbt::Double(6.0));
    compound.insert("bytes".to_string(), Nbt::ByteArray(Box::new([7, 8])));
    compound.insert("string".to_string(), Nbt::String("value".to_string()));
    compound.insert("list".to_string(), Nbt::List(Box::new([Nbt::Int(9)])));
    compound.insert("ints".to_string(), Nbt::IntArray(Box::new([10, 11])));
    compound.insert("longs".to_string(), Nbt::LongArray(Box::new([12, 13])));

    let tag = Nbt::Compound(compound.clone());
    let mut named = Vec::new();
    tag.write("root", &mut named).unwrap();
    let mut named_slice = named.as_slice();
    assert_eq!(
        Nbt::read_from_stream(&mut named_slice).unwrap(),
        ("root".to_string(), tag.clone())
    );

    let mut unnamed = Vec::new();
    tag.write_unnamed(&mut unnamed).unwrap();
    let mut unnamed_slice = unnamed.as_slice();
    assert_eq!(Nbt::read_unnamed(&mut unnamed_slice).unwrap(), tag);
}

#[test]
fn invalid_nbt_data_returns_errors() {
    assert!(Nbt::read_unnamed(&mut [99u8].as_slice()).is_err());
    assert!(Nbt::read_unnamed(&mut [8u8, 0, 4, 0xff].as_slice()).is_err());
    assert!(Nbt::read_unnamed(&mut [9u8, 3, 255, 255, 255, 255].as_slice()).is_err());
    assert!(
        Nbt::List(Box::new([Nbt::Int(1), Nbt::String("bad".to_string())]))
            .write_unnamed(&mut Vec::new())
            .is_err()
    );
}

#[test]
fn tag_handler_supports_minestom_style_operations() {
    let mut handler = TagHandler::new_handler();
    let score = Tag::<i32>::integer("score").default_value(5);
    let nested = Tag::<String>::string("name").path(["profile"]);
    let transient = Tag::<i32>::integer("temporary").transient();
    let uuid = Uuid::from_u128(42);
    let uuid_tag = Tag::<Uuid>::uuid("uuid");

    assert_eq!(handler.get_tag(&score), Some(5));
    handler.set_tag(&score, Some(10));
    handler.set_tag(&nested, Some("Wayne".to_string()));
    handler.set_tag(&uuid_tag, Some(uuid));
    handler.set_tag(&transient, Some(99));

    assert_eq!(handler.get_and_set_tag(&score, Some(11)), Some(10));
    handler.update_tag(&score, |value| value.map(|value| value + 1));

    assert_eq!(handler.get_tag(&score), Some(12));
    assert_eq!(handler.get_tag(&nested), Some("Wayne".to_string()));
    assert_eq!(handler.get_tag(&uuid_tag), Some(uuid));
    assert_eq!(handler.get_tag(&transient), None);
}

#[test]
fn equal_nbt_values_have_equal_hashes() {
    let first_nan = Nbt::List(vec![Nbt::Float(f32::NAN), Nbt::Double(f64::NAN)].into_boxed_slice());
    let second_nan = Nbt::List(
        vec![
            Nbt::Float(f32::from_bits(0x7fc0_0001)),
            Nbt::Double(f64::from_bits(0x7ff8_0000_0000_0001)),
        ]
        .into_boxed_slice(),
    );

    assert_eq!(first_nan, second_nan);
    assert_eq!(hash(&first_nan), hash(&second_nan));
    assert_ne!(Nbt::Float(0.0), Nbt::Float(-0.0));
    assert_ne!(hash(&Nbt::Int(1)), hash(&Nbt::Long(1)));
}

fn hash(value: &Nbt) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
