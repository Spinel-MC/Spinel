use crate::color::{AlphaColor, Color, ColorComponent, DyeColor};

#[test]
fn color_converts_to_and_from_rgb_int() {
    let color = Color::from_rgb_int(0x12_34_56);

    assert_eq!(color.red(), 0x12);
    assert_eq!(color.green(), 0x34);
    assert_eq!(color.blue(), 0x56);
    assert_eq!(color.as_rgb(), 0x12_34_56);
}

#[test]
fn color_rejects_out_of_range_components() {
    let error = Color::try_from_rgb(0, 300, 0).unwrap_err();

    assert_eq!(error.component, ColorComponent::Green);
    assert_eq!(error.value, 300);
}

#[test]
fn color_hex_and_alpha_helpers_match_minestom_shape() {
    let color = Color::from_hex_string("#123456").unwrap();

    assert_eq!(color.as_rgb(), 0x12_34_56);
    assert_eq!(color.as_hex_string(), "#123456");
    assert_eq!(color.with_red(0xaa).as_rgb(), 0xaa_34_56);
    assert_eq!(color.with_alpha(0x80).as_argb(), 0x80_12_34_56);
}

#[test]
fn alpha_color_converts_argb_and_rgba() {
    let color = AlphaColor::from_argb(0x12_34_56_78);

    assert_eq!(color.alpha(), 0x12);
    assert_eq!(color.as_argb(), 0x12_34_56_78);
    assert_eq!(color.as_rgba(), 0x34_56_78_12);
    assert_eq!(AlphaColor::from_rgba_hex_string("#34567812"), Some(color));
    assert_eq!(AlphaColor::from_argb_hex_string("#12345678"), Some(color));
}

#[test]
fn dye_colors_include_all_vanilla_entries() {
    assert_eq!(DyeColor::ALL.len(), 16);
    assert_eq!(DyeColor::White.color().as_rgb(), 0xf9fffe);
    assert_eq!(DyeColor::Black.text_color().as_rgb(), 0x000000);
    assert_eq!(DyeColor::Red.firework_color().as_rgb(), 0xb3312c);
    assert_eq!(DyeColor::Orange.map_color_id(), 15);
}
