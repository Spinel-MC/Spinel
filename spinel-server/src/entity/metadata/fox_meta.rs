use crate::entity::metadata::{AnimalMeta, EntityMeta, FoxVariant, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct FoxMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> FoxMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::FOX).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_variant(&self) -> FoxVariant {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::fox::get_variant())
        {
            MetadataValue::VarInt(variant_id) => {
                FoxVariant::from_protocol_id(variant_id).unwrap_or_default()
            }
            _ => FoxVariant::default(),
        }
    }

    pub fn set_variant(&mut self, variant: FoxVariant) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::fox::get_variant(),
            MetadataValue::VarInt(variant.protocol_id()),
        );
    }

    pub fn is_sitting(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_sitting())
    }

    pub fn set_sitting(&mut self, is_sitting: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_sitting(), is_sitting);
    }

    pub fn is_fox_sneaking(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_crouching())
    }

    pub fn set_fox_sneaking(&mut self, is_fox_sneaking: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_crouching(), is_fox_sneaking);
    }

    pub fn is_interested(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_interested())
    }

    pub fn set_interested(&mut self, is_interested: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_interested(), is_interested);
    }

    pub fn is_pouncing(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_pouncing())
    }

    pub fn set_pouncing(&mut self, is_pouncing: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_pouncing(), is_pouncing);
    }

    pub fn is_sleeping(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_sleeping())
    }

    pub fn set_sleeping(&mut self, is_sleeping: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_sleeping(), is_sleeping);
    }

    pub fn is_faceplanted(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_faceplanted())
    }

    pub fn set_faceplanted(&mut self, is_faceplanted: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_faceplanted(), is_faceplanted);
    }

    pub fn is_defending(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::fox::is_defending())
    }

    pub fn set_defending(&mut self, is_defending: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::fox::is_defending(), is_defending);
    }

    pub fn get_first_uuid(&self) -> Option<Uuid> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::fox::get_first_uuid())
        {
            MetadataValue::OptionalLivingEntityReference(first_uuid) => first_uuid,
            _ => None,
        }
    }

    pub fn set_first_uuid(&mut self, first_uuid: Option<Uuid>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::fox::get_first_uuid(),
            MetadataValue::OptionalLivingEntityReference(first_uuid),
        );
    }

    pub fn get_second_uuid(&self) -> Option<Uuid> {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::fox::get_second_uuid())
        {
            MetadataValue::OptionalLivingEntityReference(second_uuid) => second_uuid,
            _ => None,
        }
    }

    pub fn set_second_uuid(&mut self, second_uuid: Option<Uuid>) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::fox::get_second_uuid(),
            MetadataValue::OptionalLivingEntityReference(second_uuid),
        );
    }
}

impl<'entity> Deref for FoxMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for FoxMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
