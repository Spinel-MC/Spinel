use crate::entity::EquipmentSlot;
use crate::inventory::slot_conversion::{
    BOOTS_SLOT, CHESTPLATE_SLOT, HELMET_SLOT, OFFHAND_SLOT, convert_minestom_slot_to_window_slot,
    convert_player_inventory_slot_to_minestom_slot, convert_window_0_slot_to_minestom_slot,
};
use crate::inventory::{
    Click, ClickPreprocessor, Inventory, InventoryClickProcessor, InventoryType, PlayerInventory,
    TransactionOption, TransactionResult,
};
use spinel_core::network::serverbound::play::container_click::ContainerClickPacket;
use spinel_nbt::{Tag, TagReadable, TagWritable};
use spinel_network::types::{Array, ItemStackHash};
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;

#[test]
fn inventory_types_match_minestom_sizes_and_ordinals() {
    assert_eq!(InventoryType::Chest(3).size(), 27);
    assert_eq!(InventoryType::Chest(6).size(), 54);
    assert_eq!(InventoryType::Anvil.size(), 3);
    assert_eq!(InventoryType::Chest(1).window_type(), 0);
    assert_eq!(InventoryType::StoneCutter.window_type(), 24);
}

#[test]
fn player_inventory_slot_conversion_matches_minestom() {
    assert_eq!(convert_window_0_slot_to_minestom_slot(5), HELMET_SLOT);
    assert_eq!(convert_window_0_slot_to_minestom_slot(8), BOOTS_SLOT);
    assert_eq!(convert_minestom_slot_to_window_slot(0), 36);
    assert_eq!(convert_minestom_slot_to_window_slot(OFFHAND_SLOT), 45);
    assert_eq!(
        convert_player_inventory_slot_to_minestom_slot(38),
        CHESTPLATE_SLOT
    );
}

#[test]
fn inventory_stores_items_and_tags() {
    let tag = Tag::<i32>::integer("test");
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    inventory.set_tag(&tag, Some(7));
    inventory.set_item_stack(0, ItemStack::of(Material::DIAMOND));

    assert_eq!(inventory.get_tag(&tag), Some(7));
    assert_eq!(
        inventory.get_item_stack(0).unwrap().material(),
        &Material::DIAMOND
    );
}

#[test]
fn click_preprocessor_matches_minestom_pickup_and_swap_clicks() {
    let mut preprocessor = ClickPreprocessor::default();
    let left_click = click_packet(0, 0, 0, 0);
    let hotbar_swap = click_packet(10, 2, 3, 0);

    assert_eq!(
        preprocessor.process_click(&left_click, Some(9)),
        Some(Click::Left(0))
    );
    assert_eq!(
        preprocessor.process_click(&hotbar_swap, Some(9)),
        Some(Click::HotbarSwap {
            hotbar_slot: 3,
            slot: 19,
        })
    );
}

#[test]
fn click_preprocessor_keeps_drag_slots_unique_in_packet_order() {
    let mut preprocessor = ClickPreprocessor::default();
    let start_left_drag = click_packet(-999, 5, 0, 0);
    let first_slot = click_packet(10, 5, 1, 0);
    let duplicate_first_slot = click_packet(10, 5, 1, 0);
    let second_slot = click_packet(11, 5, 1, 0);
    let end_left_drag = click_packet(-999, 5, 2, 0);

    assert_eq!(preprocessor.process_click(&start_left_drag, None), None);
    assert_eq!(preprocessor.process_click(&first_slot, None), None);
    assert_eq!(
        preprocessor.process_click(&duplicate_first_slot, None),
        None
    );
    assert_eq!(preprocessor.process_click(&second_slot, None), None);
    assert_eq!(
        preprocessor.process_click(&end_left_drag, None),
        Some(Click::LeftDrag(vec![10, 11]))
    );
}

#[test]
fn click_preprocessor_finishes_drag_with_external_inventory_open() {
    let mut preprocessor = ClickPreprocessor::default();

    assert_eq!(
        preprocessor.process_click(&click_packet(-999, 5, 0, 0), Some(9)),
        None
    );
    assert_eq!(
        preprocessor.process_click(&click_packet(0, 5, 1, 0), Some(9)),
        None
    );
    assert_eq!(
        preprocessor.process_click(&click_packet(9, 5, 1, 0), Some(9)),
        None
    );
    assert_eq!(
        preprocessor.process_click(&click_packet(-999, 5, 2, 0), Some(9)),
        Some(Click::LeftDrag(vec![0, 18]))
    );
}

#[test]
fn click_window_conversion_matches_minestom_for_player_inventory_slots() {
    let click = Click::HotbarSwap {
        hotbar_slot: 2,
        slot: 14,
    };
    let window_click = click.clone().to_window(Some(9));

    assert!(!window_click.in_open_inventory());
    assert_eq!(
        window_click.click(),
        &Click::HotbarSwap {
            hotbar_slot: 2,
            slot: 5,
        }
    );
    assert_eq!(window_click.from_window(Some(9)), click);
}

#[test]
fn click_processor_matches_minestom_left_and_right_click_rules() {
    let diamond = ItemStack::of(Material::DIAMOND).with_amount(10);
    let cursor = ItemStack::of(Material::DIAMOND).with_amount(5);
    let left_result = InventoryClickProcessor::left_click(diamond.clone(), cursor.clone());
    let right_result = InventoryClickProcessor::right_click(diamond, cursor);

    assert_eq!(left_result.clicked().amount(), 15);
    assert!(left_result.cursor().is_air());
    assert_eq!(right_result.clicked().amount(), 11);
    assert_eq!(right_result.cursor().amount(), 4);
}

#[test]
fn inventory_transactions_match_minestom_add_take_options() {
    let mut inventory = Inventory::new(InventoryType::Chest(1), Component::text("Test").build());
    assert!(inventory.add_item_stack(ItemStack::of(Material::DIAMOND).with_amount(32)));
    assert_eq!(inventory.get_item_stack(0).unwrap().amount(), 32);

    let remaining = inventory.add_item_stack_with_option(
        ItemStack::of(Material::DIAMOND).with_amount(40),
        TransactionOption::All,
    );

    assert_eq!(inventory.get_item_stack(0).unwrap().amount(), 64);
    assert_eq!(inventory.get_item_stack(1).unwrap().amount(), 8);
    assert!(matches!(remaining, TransactionResult::Remaining(item) if item.is_air()));

    let dry_run = inventory.take_item_stack(
        ItemStack::of(Material::DIAMOND).with_amount(64),
        TransactionOption::DryRun,
    );

    assert_eq!(inventory.get_item_stack(0).unwrap().amount(), 64);
    assert_eq!(dry_run, TransactionResult::Complete(true));
}

#[test]
fn player_inventory_equipment_and_packet_slots_match_minestom() {
    let mut inventory = PlayerInventory::new();
    inventory.set_item_stack(
        HELMET_SLOT as usize,
        ItemStack::of(Material::DIAMOND_HELMET),
    );
    inventory.set_equipment(EquipmentSlot::OffHand, 0, ItemStack::of(Material::DIAMOND));

    assert_eq!(
        inventory.get_equipment(EquipmentSlot::Helmet, 0).material(),
        &Material::DIAMOND_HELMET
    );
    assert_eq!(
        inventory.get_equipment(EquipmentSlot::OffHand, 0).material(),
        &Material::DIAMOND
    );
    assert_eq!(
        inventory.send_slot_refresh(HELMET_SLOT, &ItemStack::air()),
        Some(crate::inventory::PlayerInventoryPacketSlot::PlayerInventory(39))
    );
}

fn click_packet(slot: i16, click_type: i32, button: i8, container_id: i32) -> ContainerClickPacket {
    ContainerClickPacket {
        container_id,
        state_id: 0,
        slot,
        button,
        click_type,
        changed_slots: Array(Vec::new()),
        carried_item: ItemStackHash::Air,
    }
}
