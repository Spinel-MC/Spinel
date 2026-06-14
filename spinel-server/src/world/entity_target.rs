use crate::entity::{Entity, EntityId, GenericEntity};
use crate::world::World;
use spinel_registry::EntityType;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum SetEntityTargetError {
    OwnerNotFound {
        owner_id: EntityId,
    },
    WrongOwnerType {
        owner_id: EntityId,
        expected_type: EntityType,
    },
    TargetNotFound {
        target_id: EntityId,
    },
}

impl Display for SetEntityTargetError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OwnerNotFound { owner_id } => {
                write!(formatter, "target owner {owner_id:?} does not exist")
            }
            Self::WrongOwnerType {
                owner_id,
                expected_type,
            } => write!(formatter, "entity {owner_id:?} is not a {expected_type:?}"),
            Self::TargetNotFound { target_id } => {
                write!(formatter, "target entity {target_id:?} does not exist")
            }
        }
    }
}

impl std::error::Error for SetEntityTargetError {}

impl World {
    pub fn guardian_target(&self, guardian_id: EntityId) -> Option<&Entity> {
        let target_id = self
            .target_owner(guardian_id, EntityType::GUARDIAN)?
            .guardian_target_entity_id();
        self.target_by_protocol_id(target_id)
    }

    pub fn set_guardian_target(
        &mut self,
        guardian_id: EntityId,
        target_id: Option<EntityId>,
    ) -> Result<(), SetEntityTargetError> {
        self.validate_target(target_id)?;
        self.target_owner_mut(guardian_id, EntityType::GUARDIAN)?
            .set_guardian_target_entity_id(target_protocol_id(target_id));
        Ok(())
    }

    pub fn wither_center_head_target(&self, wither_id: EntityId) -> Option<&Entity> {
        let target_id = self
            .target_owner(wither_id, EntityType::WITHER)?
            .wither_center_head_entity_id();
        self.target_by_protocol_id(target_id)
    }

    pub fn set_wither_center_head_target(
        &mut self,
        wither_id: EntityId,
        target_id: Option<EntityId>,
    ) -> Result<(), SetEntityTargetError> {
        self.validate_target(target_id)?;
        self.target_owner_mut(wither_id, EntityType::WITHER)?
            .set_wither_center_head_entity_id(target_protocol_id(target_id));
        Ok(())
    }

    pub fn wither_left_head_target(&self, wither_id: EntityId) -> Option<&Entity> {
        let target_id = self
            .target_owner(wither_id, EntityType::WITHER)?
            .wither_left_head_entity_id();
        self.target_by_protocol_id(target_id)
    }

    pub fn set_wither_left_head_target(
        &mut self,
        wither_id: EntityId,
        target_id: Option<EntityId>,
    ) -> Result<(), SetEntityTargetError> {
        self.validate_target(target_id)?;
        self.target_owner_mut(wither_id, EntityType::WITHER)?
            .set_wither_left_head_entity_id(target_protocol_id(target_id));
        Ok(())
    }

    pub fn wither_right_head_target(&self, wither_id: EntityId) -> Option<&Entity> {
        let target_id = self
            .target_owner(wither_id, EntityType::WITHER)?
            .wither_right_head_entity_id();
        self.target_by_protocol_id(target_id)
    }

    pub fn set_wither_right_head_target(
        &mut self,
        wither_id: EntityId,
        target_id: Option<EntityId>,
    ) -> Result<(), SetEntityTargetError> {
        self.validate_target(target_id)?;
        self.target_owner_mut(wither_id, EntityType::WITHER)?
            .set_wither_right_head_entity_id(target_protocol_id(target_id));
        Ok(())
    }

    fn target_owner(
        &self,
        owner_id: EntityId,
        expected_type: EntityType,
    ) -> Option<&GenericEntity> {
        match self.entity_by_id(owner_id)? {
            Entity::Creature(owner) if owner.entity_type() == expected_type => Some(owner.entity()),
            Entity::Generic(owner) if owner.entity_type() == expected_type => Some(owner),
            _ => None,
        }
    }

    fn target_owner_mut(
        &mut self,
        owner_id: EntityId,
        expected_type: EntityType,
    ) -> Result<&mut GenericEntity, SetEntityTargetError> {
        let owner = self
            .entity_by_id_mut(owner_id)
            .ok_or(SetEntityTargetError::OwnerNotFound { owner_id })?;
        match owner {
            Entity::Creature(owner) if owner.entity_type() == expected_type => {
                Ok(owner.entity_mut())
            }
            Entity::Generic(owner) if owner.entity_type() == expected_type => Ok(owner),
            _ => Err(SetEntityTargetError::WrongOwnerType {
                owner_id,
                expected_type,
            }),
        }
    }

    fn target_by_protocol_id(&self, target_id: i32) -> Option<&Entity> {
        (target_id != 0)
            .then_some(EntityId::from_raw(target_id))
            .and_then(|target_id| self.entity_by_id(target_id))
    }

    fn validate_target(&self, target_id: Option<EntityId>) -> Result<(), SetEntityTargetError> {
        let Some(target_id) = target_id else {
            return Ok(());
        };
        if self.entity_by_id(target_id).is_none() {
            return Err(SetEntityTargetError::TargetNotFound { target_id });
        }
        Ok(())
    }
}

fn target_protocol_id(target_id: Option<EntityId>) -> i32 {
    target_id.map_or(0, EntityId::value)
}
