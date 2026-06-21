use spinel_registry::EntityType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct EntityCollisionRules {
    has_entity_collision: bool,
    prevents_block_placement: bool,
}

impl EntityCollisionRules {
    pub(crate) fn from_entity_type(entity_type: EntityType) -> Self {
        Self {
            has_entity_collision: entity_type_has_entity_collision(entity_type),
            prevents_block_placement: entity_type_prevents_block_placement(entity_type),
        }
    }

    pub(crate) const fn has_entity_collision(self) -> bool {
        self.has_entity_collision
    }

    pub(crate) const fn can_prevent_block_placement(self) -> bool {
        self.prevents_block_placement
    }
}

fn entity_type_has_entity_collision(entity_type: EntityType) -> bool {
    !matches!(
        entity_type,
        EntityType::TEXT_DISPLAY | EntityType::ITEM_DISPLAY | EntityType::BLOCK_DISPLAY
    )
}

fn entity_type_prevents_block_placement(entity_type: EntityType) -> bool {
    !matches!(
        entity_type,
        EntityType::ARROW
            | EntityType::ITEM
            | EntityType::SNOWBALL
            | EntityType::EXPERIENCE_BOTTLE
            | EntityType::EXPERIENCE_ORB
            | EntityType::SPLASH_POTION
            | EntityType::LINGERING_POTION
            | EntityType::AREA_EFFECT_CLOUD
    )
}
