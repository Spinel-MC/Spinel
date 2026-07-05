impl World {
    pub fn explosion_supplier(&self) -> Option<&dyn ExplosionSupplier> {
        self.explosion_supplier.as_deref()
    }

    pub fn set_explosion_supplier(&mut self, supplier: impl ExplosionSupplier + 'static) {
        self.explosion_supplier = Some(Box::new(supplier));
    }

    pub fn clear_explosion_supplier(&mut self) {
        self.explosion_supplier = None;
    }

    pub fn explode(&mut self, center: EntityPosition, strength: f32) -> Result<Vec<BlockPosition>> {
        self.explode_with_data(center, strength, None)
    }

    pub fn explode_with_data(
        &mut self,
        center: EntityPosition,
        strength: f32,
        additional_data: Option<&NbtCompound>,
    ) -> Result<Vec<BlockPosition>> {
        let Some(explosion_supplier) = self.explosion_supplier() else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No explosion supplier was set",
            ));
        };
        let explosion =
            explosion_supplier.create_explosion_with_data(center, strength, additional_data);
        explosion.apply(self)
    }
}
