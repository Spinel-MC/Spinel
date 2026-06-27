use crate::entity::metadata::{AbstractMinecartMeta, EntityMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use spinel_utils::component::text::TextComponent;
use std::ops::{Deref, DerefMut};

pub struct CommandBlockMinecartMeta<'entity> {
    abstract_minecart_meta: AbstractMinecartMeta<'entity>,
}

impl<'entity> CommandBlockMinecartMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::COMMAND_BLOCK_MINECART).then(
            || Self {
                abstract_minecart_meta: AbstractMinecartMeta::from_entity_meta(entity_meta),
            },
        )
    }

    pub fn get_command(&self) -> String {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::command_block_minecart::get_command())
        {
            MetadataValue::String(command) => command,
            _ => String::new(),
        }
    }

    pub fn set_command(&mut self, command: String) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::command_block_minecart::get_command(),
            MetadataValue::String(command),
        );
    }

    pub fn get_last_output(&self) -> TextComponent {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::command_block_minecart::get_last_output())
        {
            MetadataValue::Text(last_output) => last_output,
            _ => TextComponent::empty(),
        }
    }

    pub fn set_last_output(&mut self, last_output: TextComponent) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::command_block_minecart::get_last_output(),
            MetadataValue::Text(last_output),
        );
    }
}

impl<'entity> Deref for CommandBlockMinecartMeta<'entity> {
    type Target = AbstractMinecartMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_minecart_meta
    }
}

impl<'entity> DerefMut for CommandBlockMinecartMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_minecart_meta
    }
}
