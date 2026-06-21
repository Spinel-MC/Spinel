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

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataByteMaskDefinition {
    index: u8,
    byte_mask: i8,
    offset: u8,
    default_value: i8,
}

impl MetadataDefinition {
    pub const fn new(index: u8, default_value: MetadataValue) -> Self {
        Self {
            index,
            default_value,
        }
    }

    pub const fn get_index(&self) -> u8 {
        self.index
    }

    pub const fn get_default_value(&self) -> &MetadataValue {
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

    pub const fn get_index(&self) -> u8 {
        self.index
    }

    pub const fn get_bit_mask(&self) -> i8 {
        self.bit_mask
    }

    pub const fn get_default_value(&self) -> bool {
        self.default_value
    }
}

impl MetadataByteMaskDefinition {
    pub const fn new(index: u8, byte_mask: i8, offset: u8, default_value: i8) -> Self {
        Self {
            index,
            byte_mask,
            offset,
            default_value,
        }
    }

    pub const fn get_index(&self) -> u8 {
        self.index
    }

    pub const fn get_byte_mask(&self) -> i8 {
        self.byte_mask
    }

    pub const fn get_offset(&self) -> u8 {
        self.offset
    }

    pub const fn get_default_value(&self) -> i8 {
        self.default_value
    }
}
