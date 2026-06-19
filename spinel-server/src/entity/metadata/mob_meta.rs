use crate::entity::metadata::LivingEntityMeta;
use std::ops::{Deref, DerefMut};

pub struct MobMeta<'entity> {
    living_entity_meta: LivingEntityMeta<'entity>,
}

impl<'entity> MobMeta<'entity> {
    pub(crate) fn new(living_entity_meta: LivingEntityMeta<'entity>) -> Self {
        Self { living_entity_meta }
    }

    pub fn is_no_ai(&self) -> bool {
        self.entity().is_no_ai()
    }

    pub fn set_no_ai(&mut self, is_no_ai: bool) {
        self.entity_mut().set_no_ai(is_no_ai);
    }

    pub fn is_left_handed(&self) -> bool {
        self.entity().is_left_handed()
    }

    pub fn set_left_handed(&mut self, is_left_handed: bool) {
        self.entity_mut().set_left_handed(is_left_handed);
    }

    pub fn is_aggressive(&self) -> bool {
        self.entity().is_aggressive()
    }

    pub fn set_aggressive(&mut self, is_aggressive: bool) {
        self.entity_mut().set_aggressive(is_aggressive);
    }
}

impl<'entity> Deref for MobMeta<'entity> {
    type Target = LivingEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.living_entity_meta
    }
}

impl<'entity> DerefMut for MobMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity_meta
    }
}
