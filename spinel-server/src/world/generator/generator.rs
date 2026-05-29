use crate::world::generator::GenerationUnit;

pub trait Generator {
    fn generate(&self, unit: &mut GenerationUnit) -> Result<(), GenerateChunkError>;
}

impl<F> Generator for F
where
    F: Fn(&mut GenerationUnit),
{
    fn generate(&self, unit: &mut GenerationUnit) -> Result<(), GenerateChunkError> {
        self(unit);
        Ok(())
    }
}

pub struct FallibleGenerator<F> {
    generator: F,
}

impl<F> FallibleGenerator<F> {
    pub fn new(generator: F) -> Self {
        Self { generator }
    }
}

impl<F> Generator for FallibleGenerator<F>
where
    F: Fn(&mut GenerationUnit) -> Result<(), GenerateChunkError>,
{
    fn generate(&self, unit: &mut GenerationUnit) -> Result<(), GenerateChunkError> {
        (self.generator)(unit)
    }
}

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GenerateChunkError {
    GeneratorFailed { reason: String },
}

impl Display for GenerateChunkError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GeneratorFailed { reason } => {
                write!(formatter, "chunk generator failed: {reason}")
            }
        }
    }
}

impl std::error::Error for GenerateChunkError {}
