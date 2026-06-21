use crate::entity::metadata::{EntityMeta, definitions};
use spinel_network::types::Particle;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct AreaEffectCloudMeta<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> AreaEffectCloudMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::AREA_EFFECT_CLOUD)
            .then_some(Self { entity_meta })
    }

    pub fn get_radius(&self) -> f32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::area_effect_cloud::get_radius())
        {
            MetadataValue::Float(radius) => radius,
            _ => 0.5,
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::area_effect_cloud::get_radius(),
            MetadataValue::Float(radius),
        );
    }

    pub fn is_waiting(&self) -> bool {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::area_effect_cloud::waiting())
        {
            MetadataValue::Boolean(is_waiting) => is_waiting,
            _ => false,
        }
    }

    pub fn set_waiting(&mut self, is_waiting: bool) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::area_effect_cloud::waiting(),
            MetadataValue::Boolean(is_waiting),
        );
    }

    pub fn get_particle(&self) -> Particle {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::area_effect_cloud::get_particle())
        {
            MetadataValue::Particle(particle) => particle,
            _ => Particle::effect(),
        }
    }

    pub fn set_particle(&mut self, particle: Particle) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::area_effect_cloud::get_particle(),
            MetadataValue::Particle(particle),
        );
    }
}

impl<'entity> Deref for AreaEffectCloudMeta<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for AreaEffectCloudMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
