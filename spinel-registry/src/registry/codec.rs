use spinel_nbt::NbtCompound;

pub trait RegistryCodec {
    fn registry_nbt(&self) -> NbtCompound;
}
