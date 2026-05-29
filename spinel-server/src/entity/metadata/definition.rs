use spinel_network::types::entity_metadata::MetadataValue;

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataDefinition {
    index: u8,
    default_value: MetadataValue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataBitMaskDefinition {
    index: u8,
    bit_mask: i8,
    default_value: bool,
}

impl MetadataDefinition {
    pub const fn new(index: u8, default_value: MetadataValue) -> Self {
        Self {
            index,
            default_value,
        }
    }

    pub const fn index(&self) -> u8 {
        self.index
    }

    pub const fn default_value(&self) -> &MetadataValue {
        &self.default_value
    }
}

impl MetadataBitMaskDefinition {
    pub const fn new(index: u8, bit_mask: i8, default_value: bool) -> Self {
        Self {
            index,
            bit_mask,
            default_value,
        }
    }

    pub const fn index(&self) -> u8 {
        self.index
    }

    pub const fn bit_mask(&self) -> i8 {
        self.bit_mask
    }

    pub const fn default_value(&self) -> bool {
        self.default_value
    }
}
