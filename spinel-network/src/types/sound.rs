#[derive(Debug, Clone)]
pub enum SoundEvent {
    Id(i32),
    Named {
        name: String,
        fixed_range: Option<f32>,
    },
}
