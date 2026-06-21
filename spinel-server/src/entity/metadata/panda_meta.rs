use crate::entity::metadata::{AnimalMeta, EntityMeta, PandaGene, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct PandaMeta<'entity> {
    animal_meta: AnimalMeta<'entity>,
}

impl<'entity> PandaMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PANDA).then(|| Self {
            animal_meta: AnimalMeta::from_entity_meta(entity_meta),
        })
    }

    pub fn get_breed_timer(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::panda::get_breed_timer())
        {
            MetadataValue::VarInt(breed_timer) => breed_timer,
            _ => 0,
        }
    }

    pub fn set_breed_timer(&mut self, breed_timer: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::panda::get_breed_timer(),
            MetadataValue::VarInt(breed_timer),
        );
    }

    pub fn get_sneeze_timer(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::panda::get_sneeze_timer())
        {
            MetadataValue::VarInt(sneeze_timer) => sneeze_timer,
            _ => 0,
        }
    }

    pub fn set_sneeze_timer(&mut self, sneeze_timer: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::panda::get_sneeze_timer(),
            MetadataValue::VarInt(sneeze_timer),
        );
    }

    pub fn get_eat_timer(&self) -> i32 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::panda::get_eat_timer())
        {
            MetadataValue::VarInt(eat_timer) => eat_timer,
            _ => 0,
        }
    }

    pub fn set_eat_timer(&mut self, eat_timer: i32) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::panda::get_eat_timer(),
            MetadataValue::VarInt(eat_timer),
        );
    }

    pub fn get_main_gene(&self) -> PandaGene {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::panda::get_main_gene())
        {
            MetadataValue::Byte(gene_id) => {
                PandaGene::from_protocol_id(gene_id as i32).unwrap_or_default()
            }
            _ => PandaGene::default(),
        }
    }

    pub fn set_main_gene(&mut self, main_gene: PandaGene) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::panda::get_main_gene(),
            MetadataValue::Byte(main_gene.get_protocol_id() as i8),
        );
    }

    pub fn get_hidden_gene(&self) -> PandaGene {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::panda::get_hidden_gene())
        {
            MetadataValue::Byte(gene_id) => {
                PandaGene::from_protocol_id(gene_id as i32).unwrap_or_default()
            }
            _ => PandaGene::default(),
        }
    }

    pub fn set_hidden_gene(&mut self, hidden_gene: PandaGene) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::panda::get_hidden_gene(),
            MetadataValue::Byte(hidden_gene.get_protocol_id() as i8),
        );
    }

    pub fn is_sneezing(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::panda::is_sneezing())
    }

    pub fn set_sneezing(&mut self, is_sneezing: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::panda::is_sneezing(), is_sneezing);
    }

    pub fn is_rolling(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::panda::is_rolling())
    }

    pub fn set_rolling(&mut self, is_rolling: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::panda::is_rolling(), is_rolling);
    }

    pub fn is_sitting(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::panda::is_sitting())
    }

    pub fn set_sitting(&mut self, is_sitting: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::panda::is_sitting(), is_sitting);
    }

    pub fn is_on_back(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .flag(&definitions::panda::is_on_back())
    }

    pub fn set_on_back(&mut self, is_on_back: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::panda::is_on_back(), is_on_back);
    }
}

impl<'entity> Deref for PandaMeta<'entity> {
    type Target = AnimalMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.animal_meta
    }
}

impl<'entity> DerefMut for PandaMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.animal_meta
    }
}
