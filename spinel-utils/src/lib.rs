pub mod component;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn to_order(&self) -> u8 {
        match self {
            Priority::High => 2,
            Priority::Medium => 1,
            Priority::Low => 0,
        }
    }
}