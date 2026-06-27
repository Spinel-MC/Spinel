use crate::entity::metadata::{AbstractGolemMeta, EntityMeta, definitions};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct IronGolemMeta<'entity> {
    abstract_golem_meta: AbstractGolemMeta<'entity>,
}

impl<'entity> IronGolemMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::IRON_GOLEM).then(|| Self {
            abstract_golem_meta: AbstractGolemMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn is_player_created(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::iron_golem::is_player_created())
    }

    pub fn set_player_created(&mut self, is_player_created: bool) {
        self.get_entity_mut().get_metadata_mut().set_flag(
            &definitions::iron_golem::is_player_created(),
            is_player_created,
        );
    }
}

impl<'entity> Deref for IronGolemMeta<'entity> {
    type Target = AbstractGolemMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_golem_meta
    }
}

impl<'entity> DerefMut for IronGolemMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_golem_meta
    }
}
