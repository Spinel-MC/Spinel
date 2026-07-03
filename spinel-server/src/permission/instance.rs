use spinel_nbt::NbtCompound;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Permission {
    permission_name: String,
    nbt_data: Option<NbtCompound>,
}

impl Permission {
    pub fn new(permission_name: impl Into<String>) -> Self {
        Self::from_nbt_data(permission_name, None)
    }

    pub fn from_nbt_data(
        permission_name: impl Into<String>,
        nbt_data: impl Into<Option<NbtCompound>>,
    ) -> Self {
        Self {
            permission_name: permission_name.into(),
            nbt_data: nbt_data.into(),
        }
    }

    pub fn get_permission_name(&self) -> &str {
        &self.permission_name
    }

    pub const fn get_nbt_data(&self) -> Option<&NbtCompound> {
        self.nbt_data.as_ref()
    }
}
