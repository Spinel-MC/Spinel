pub mod target;
pub mod tracker;

pub use target::{BlockHitTarget, HitTarget};
pub use tracker::{ClientHitTargetTracker, TrackedEntity};
#[cfg(test)]
mod tests;
