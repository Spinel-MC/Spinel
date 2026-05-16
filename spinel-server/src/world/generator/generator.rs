use crate::world::generator::GenerationUnit;

pub trait Generator {
    fn generate(&self, unit: &mut GenerationUnit);
}

impl<F> Generator for F
where
    F: Fn(&mut GenerationUnit),
{
    fn generate(&self, unit: &mut GenerationUnit) {
        self(unit);
    }
}
