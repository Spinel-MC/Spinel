#[macro_export]
macro_rules! simple_registry_value {
    ($type_name:ident) => {
        #[derive(Clone, Debug, Default, PartialEq)]
        pub struct $type_name {
            data: spinel_nbt::NbtCompound,
        }

        impl $type_name {
            #[must_use]
            pub fn raw(data: spinel_nbt::NbtCompound) -> Self {
                Self { data }
            }
        }

        impl $crate::RegistryCodec for $type_name {
            fn registry_nbt(&self) -> spinel_nbt::NbtCompound {
                self.data.clone()
            }
        }
    };
}
