use crate::entity::{EquipmentSlot, Player};
use crate::inventory::ClickType;
use crate::inventory::slot_conversion::{CRAFT_RESULT, CRAFT_SLOT_1, CRAFT_SLOT_4, OFFHAND_SLOT};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_registry::data_components::vanilla_components::EQUIPPABLE;
use spinel_registry::{ITEM_REGISTRY, Identifier, ItemStack, Material, Registries};
use std::collections::HashSet;
use std::sync::LazyLock;

impl Player {
    pub(super) fn apply_shift_click(
        &mut self,
        slot: i32,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        let Some(clicked) = self.get_item_at(slot) else {
            return false;
        };
        if clicked.is_air() {
            return true;
        }
        if self.shift_click_equips_item(slot, &clicked, player, server, client) {
            self.set_item_at_with_change_event(slot, ItemStack::air(), player, server, client);
            let cursor_item = self.get_inventory_ref().cursor_item().clone();
            let click_was_dispatched = self.dispatch_inventory_click(
                slot,
                ClickType::ShiftClick,
                &ItemStack::air(),
                &cursor_item,
                player,
                server,
                client,
            );
            return click_was_dispatched && self.resync_inventory_after_bulk_click(client);
        }
        let remaining_item = self.shift_click_remaining_item(slot, clicked, player, server, client);
        self.set_item_at_with_change_event(slot, remaining_item.clone(), player, server, client);
        let cursor_item = self.get_inventory_ref().cursor_item().clone();
        let click_was_dispatched = self.dispatch_inventory_click(
            slot,
            ClickType::ShiftClick,
            &remaining_item,
            &cursor_item,
            player,
            server,
            client,
        );
        click_was_dispatched && self.resync_inventory_after_bulk_click(client)
    }

    fn shift_click_equips_item(
        &mut self,
        slot: i32,
        clicked: &ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> bool {
        if self.get_opened_inventory().is_some() || self.is_crafting_slot(slot) {
            return false;
        }
        let Some(equipment_slot) = equipment_slot_for_item(clicked) else {
            return false;
        };
        if !self
            .get_inventory_ref()
            .get_equipment(equipment_slot, self.get_held_slot())
            .is_air()
        {
            return false;
        }
        let target_slot = match equipment_slot {
            EquipmentSlot::OffHand => OFFHAND_SLOT,
            _ => equipment_slot.get_armor_slot(),
        };
        if !self.set_item_at_with_change_event(target_slot, clicked.clone(), player, server, client)
        {
            return false;
        }
        let cursor_item = self.get_inventory_ref().cursor_item().clone();
        self.dispatch_inventory_click(
            target_slot,
            ClickType::ShiftClick,
            clicked,
            &cursor_item,
            player,
            server,
            client,
        )
    }

    fn shift_click_remaining_item(
        &mut self,
        slot: i32,
        clicked: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if self.get_opened_inventory().is_some() {
            return self.shift_click_with_open_inventory(slot, clicked, player, server, client);
        }
        self.shift_click_player_inventory(slot, clicked, player, server, client)
    }

    fn shift_click_with_open_inventory(
        &mut self,
        slot: i32,
        clicked: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        let open_inventory_size = self.open_inventory_size();
        if slot < open_inventory_size {
            let hotbar = ((open_inventory_size)..=(open_inventory_size + 8)).rev();
            let inventory = ((open_inventory_size + 9)..(open_inventory_size + 36)).rev();
            return self.move_item_to_slots(
                clicked,
                hotbar.chain(inventory),
                player,
                server,
                client,
            );
        }
        self.move_item_to_slots(clicked, 0..open_inventory_size, player, server, client)
    }

    fn shift_click_player_inventory(
        &mut self,
        slot: i32,
        clicked: ItemStack,
        player: *mut Player,
        server: &mut MinecraftServer,
        client: &mut Client,
    ) -> ItemStack {
        if self.is_equipment_slot(slot) || self.is_crafting_grid_slot(slot) {
            return self.move_item_to_slots(clicked, (9..36).chain(0..9), player, server, client);
        }
        if slot == CRAFT_RESULT {
            return self.move_item_to_slots(
                clicked,
                (0..=8).rev().chain((9..36).rev()),
                player,
                server,
                client,
            );
        }
        if slot < 9 {
            return self.move_item_to_slots(clicked, 9..36, player, server, client);
        }
        self.move_item_to_slots(clicked, 0..9, player, server, client)
    }

    fn is_crafting_slot(&self, slot: i32) -> bool {
        (CRAFT_RESULT..=CRAFT_SLOT_4).contains(&slot)
    }

    fn is_crafting_grid_slot(&self, slot: i32) -> bool {
        (CRAFT_SLOT_1..=CRAFT_SLOT_4).contains(&slot)
    }

    fn is_equipment_slot(&self, slot: i32) -> bool {
        (41..=OFFHAND_SLOT).contains(&slot)
    }
}

fn equipment_slot_for_item(item_stack: &ItemStack) -> Option<EquipmentSlot> {
    if let Some(equippable) = item_stack.get(EQUIPPABLE) {
        return EquipmentSlot::from_equippable_slot(equippable.slot());
    }
    let material_id = item_stack.material().id();
    if HEAD_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Helmet);
    }
    if CHEST_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Chestplate);
    }
    if LEG_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Leggings);
    }
    if FOOT_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::Boots);
    }
    if OFFHAND_EQUIPMENT_ITEMS.contains(&material_id) {
        return Some(EquipmentSlot::OffHand);
    }
    None
}

static HEAD_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> = LazyLock::new(|| {
    item_tag_entries("head_armor")
        .into_iter()
        .chain(item_ids([
            Material::CARVED_PUMPKIN,
            Material::CREEPER_HEAD,
            Material::DRAGON_HEAD,
            Material::PIGLIN_HEAD,
            Material::PLAYER_HEAD,
            Material::SKELETON_SKULL,
            Material::WITHER_SKELETON_SKULL,
            Material::ZOMBIE_HEAD,
        ]))
        .collect()
});

static CHEST_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> = LazyLock::new(|| {
    item_tag_entries("chest_armor")
        .into_iter()
        .chain(item_ids([Material::ELYTRA]))
        .collect()
});

static LEG_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> =
    LazyLock::new(|| item_tag_entries("leg_armor").into_iter().collect());

static FOOT_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> =
    LazyLock::new(|| item_tag_entries("foot_armor").into_iter().collect());

static OFFHAND_EQUIPMENT_ITEMS: LazyLock<HashSet<i32>> =
    LazyLock::new(|| item_ids([Material::SHIELD, Material::TOTEM_OF_UNDYING]).collect());

fn item_tag_entries(tag_path: &'static str) -> Vec<i32> {
    Registries::new_vanilla()
        .dynamic_tag_entries()
        .ok()
        .into_iter()
        .flatten()
        .find(|registry_tags| registry_tags.registry_name == ITEM_REGISTRY)
        .and_then(|registry_tags| {
            registry_tags
                .tags
                .into_iter()
                .find(|registry_tag| registry_tag.tag_name == Identifier::vanilla_static(tag_path))
        })
        .map(|registry_tag| registry_tag.entries)
        .unwrap_or_default()
}

fn item_ids<const N: usize>(materials: [Material; N]) -> impl Iterator<Item = i32> {
    materials.into_iter().map(|material| material.id())
}
