use spinel_nbt::NbtCompound;

pub trait PermissionVerifier {
    fn is_valid(&self, nbt_data: Option<&NbtCompound>) -> bool;
}

impl<F> PermissionVerifier for F
where
    F: Fn(Option<&NbtCompound>) -> bool,
{
    fn is_valid(&self, nbt_data: Option<&NbtCompound>) -> bool {
        self(nbt_data)
    }
}
