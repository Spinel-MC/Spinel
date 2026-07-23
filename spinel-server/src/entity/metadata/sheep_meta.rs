use crate::entity::metadata::{AnimalMeta, LivingEntityMeta, definitions};
use spinel_registry::EntityType;
use spinel_utils::color::DyeColor;
use std::ops::{Deref, DerefMut};

pub struct SheepMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> SheepMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::SHEEP).then(|| Self {
            animal_meta: AnimalMeta::from_living_entity_meta(living_entity_meta),
        })
    }

    pub fn get_color(&self) -> DyeColor {
        let color_id = self
            .get_state()
            .get_metadata()
            .get_byte(&definitions::sheep::color_id());
        DyeColor::ALL
            .get(color_id as usize)
            .copied()
            .unwrap_or(DyeColor::White)
    }

    pub fn set_color(&mut self, color: DyeColor) {
        let color_id = DyeColor::ALL
            .iter()
            .position(|candidate| candidate == &color)
            .unwrap_or(0) as i8;
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_byte(&definitions::sheep::color_id(), color_id);
    }

    pub fn is_sheared(&self) -> bool {
        self.get_state()
            .get_metadata()
            .get_flag(&definitions::sheep::is_sheared())
    }

    pub fn set_sheared(&mut self, is_sheared: bool) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_flag(&definitions::sheep::is_sheared(), is_sheared);
    }
}

impl<'entity> Deref for SheepMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for SheepMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
