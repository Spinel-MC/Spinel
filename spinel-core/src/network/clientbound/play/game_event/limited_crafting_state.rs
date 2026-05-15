#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitedCraftingState {
    Disabled,
    Enabled,
}

impl LimitedCraftingState {
    pub(crate) const fn value(self) -> f32 {
        match self {
            Self::Disabled => 0.0,
            Self::Enabled => 1.0,
        }
    }
}
