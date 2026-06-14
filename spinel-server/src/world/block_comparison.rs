#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlockComparison {
    Identity,
    BlockType,
    BlockState,
}
