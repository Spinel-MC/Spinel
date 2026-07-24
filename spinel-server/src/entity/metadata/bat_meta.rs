use crate::entity::metadata::{AmbientCreatureMeta, LivingEntityMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct BatMeta<'entity> {
    ambient_creature_meta: AmbientCreatureMeta<'entity>,
}

impl<'entity> BatMeta<'entity> {
    pub(crate) fn from_living_entity_meta(
        living_entity_meta: LivingEntityMeta<'entity>,
    ) -> Option<Self> {
        (living_entity_meta.get_entity_type() == EntityType::BAT).then(|| Self {
            ambient_creature_meta: AmbientCreatureMeta::new(living_entity_meta),
        })
    }

    pub fn is_hanging(&self) -> bool {
        self.get_state()
            .get_metadata()
            .get_flag(&definitions::bat::is_hanging())
    }

    pub fn set_hanging(&mut self, is_hanging: bool) {
        self.get_entity_state_mut()
            .get_metadata_mut()
            .set_flag(&definitions::bat::is_hanging(), is_hanging);
    }
}

impl<'entity> Deref for BatMeta<'entity> {
    type Target = AmbientCreatureMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.ambient_creature_meta
    }
}

impl<'entity> DerefMut for BatMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ambient_creature_meta
    }
}
