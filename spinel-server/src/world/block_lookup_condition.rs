#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockLookupCondition {
    None,
    Cached,
    Type,
}
