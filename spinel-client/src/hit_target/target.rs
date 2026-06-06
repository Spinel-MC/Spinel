use spinel_network::types::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HitTarget {
    Entity { entity_id: i32 },
    Block(BlockHitTarget),
    Miss,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockHitTarget {
    pub position: Position,
    pub face: i8,
}
