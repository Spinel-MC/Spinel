use crate::{Nbt, parse_snbt_compound};

#[test]
fn snbt_parser_reads_nested_summon_shape() {
    let compound = parse_snbt_compound(
        r#"{Pos:[1.5d,64.0d,-2.25d],Rotation:[90.0f,15.0f],CustomName:'{"text":"Bot"}',Flags:{Silent:true,NoGravity:1b},UUID:[I;1,2,3,4]}"#,
    )
    .unwrap();

    assert!(matches!(compound.get("Pos"), Some(Nbt::List(values)) if values.len() == 3));
    assert!(matches!(compound.get("Rotation"), Some(Nbt::List(values)) if values.len() == 2));
    assert_eq!(
        compound.get("CustomName"),
        Some(&Nbt::String(r#"{"text":"Bot"}"#.to_owned()))
    );
    assert!(matches!(compound.get("Flags"), Some(Nbt::Compound(_))));
    assert_eq!(
        compound.get("UUID"),
        Some(&Nbt::IntArray(Box::new([1, 2, 3, 4])))
    );
}

#[test]
fn snbt_parser_rejects_mixed_list_types_and_trailing_input() {
    assert!(parse_snbt_compound("{Values:[1,2.0d]}").is_err());
    assert!(parse_snbt_compound("{Value:1} trailing").is_err());
}
