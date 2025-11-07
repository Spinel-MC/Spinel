use spinel_nbt::NbtCompound;

#[derive(Debug, Clone)]
pub struct Slot {
    pub is_present: bool,
    pub item_id: i32,
    pub item_count: i8,
    pub nbt: Option<NbtCompound>,
}
