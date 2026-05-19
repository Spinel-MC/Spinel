use spinel_nbt::{Nbt, NbtCompound};

pub(crate) fn put_optional(compound: &mut NbtCompound, name: &'static str, value: Option<Nbt>) {
    if let Some(value) = value {
        compound.insert(name.to_string(), value);
    }
}

pub(crate) fn bool_tag(value: bool) -> Nbt {
    Nbt::Byte(i8::from(value))
}

pub(crate) fn color_tag(color: u32) -> Nbt {
    Nbt::String(format!("#{color:06x}"))
}
