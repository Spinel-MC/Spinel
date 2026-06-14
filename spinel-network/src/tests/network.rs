use crate::types::{BlockHitResult, Position};
use crate::{DataType, VarIntWrapper};

#[test]
fn block_hit_result_matches_minestom_use_item_on_shape() {
    let block_hit = BlockHitResult {
        position: Position { x: 1, y: 2, z: 3 },
        direction: 1,
        cursor_x: 0.25,
        cursor_y: 0.5,
        cursor_z: 0.75,
        is_inside: false,
        hit_world_border: true,
    };
    let mut bytes = Vec::new();
    block_hit.encode(&mut bytes).unwrap();
    VarIntWrapper(12).encode(&mut bytes).unwrap();

    let mut reader = bytes.as_slice();
    let decoded = BlockHitResult::decode(&mut reader).unwrap();
    let sequence = VarIntWrapper::decode(&mut reader).unwrap().0;

    assert_eq!(decoded, block_hit);
    assert_eq!(sequence, 12);
    assert!(reader.is_empty());
}
