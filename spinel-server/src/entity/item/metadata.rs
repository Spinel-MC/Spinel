use crate::entity::ItemEntity;
use spinel_registry::ItemStack;

pub struct ItemEntityMeta<'entity> {
    item_entity: &'entity mut ItemEntity,
}

impl<'entity> ItemEntityMeta<'entity> {
    pub(crate) fn new(item_entity: &'entity mut ItemEntity) -> Self {
        Self { item_entity }
    }

    pub fn get_item(&self) -> &ItemStack {
        self.item_entity.get_item_stack()
    }

    pub fn set_item(&mut self, item: ItemStack) {
        self.item_entity.set_item_metadata(item);
    }
}
