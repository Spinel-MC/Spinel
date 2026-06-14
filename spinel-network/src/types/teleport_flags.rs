use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeleportFlags {
    bitmask: i32,
}

impl TeleportFlags {
    pub const X: i32 = 1 << 0;
    pub const Y: i32 = 1 << 1;
    pub const Z: i32 = 1 << 2;
    pub const Y_ROTATION: i32 = 1 << 3;
    pub const X_ROTATION: i32 = 1 << 4;
    pub const DELTA_X: i32 = 1 << 5;
    pub const DELTA_Y: i32 = 1 << 6;
    pub const DELTA_Z: i32 = 1 << 7;
    pub const ROTATE_DELTA: i32 = 1 << 8;
    pub const COORD: i32 = Self::X | Self::Y | Self::Z;
    pub const VIEW: i32 = Self::Y_ROTATION | Self::X_ROTATION;
    pub const DELTA_COORD: i32 = Self::DELTA_X | Self::DELTA_Y | Self::DELTA_Z;
    pub const DELTA: i32 = Self::DELTA_COORD | Self::ROTATE_DELTA;
    pub const ALL: i32 = Self::COORD | Self::VIEW | Self::DELTA;

    pub const fn absolute() -> Self {
        Self { bitmask: 0 }
    }

    pub const fn from_bitmask(bitmask: i32) -> Self {
        Self { bitmask }
    }

    pub const fn bitmask(self) -> i32 {
        self.bitmask
    }

    pub const fn contains(self, flag: i32) -> bool {
        self.bitmask & flag != 0
    }

    pub const fn with(self, flags: i32) -> Self {
        Self::from_bitmask(self.bitmask | flags)
    }
}

impl DataType for TeleportFlags {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.bitmask.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Self::from_bitmask(i32::decode(r)?))
    }
}
