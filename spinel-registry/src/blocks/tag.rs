use crate::{Identifier, Registries, vanilla_world_blocks::Block};
use std::sync::OnceLock;

static VANILLA_REGISTRIES: OnceLock<Registries> = OnceLock::new();

impl Block {
    pub fn is_in_vanilla_tag(self, tag_name: &Identifier) -> bool {
        VANILLA_REGISTRIES
            .get_or_init(Registries::new_vanilla)
            .block_tag_contains(tag_name, &self)
    }
}
