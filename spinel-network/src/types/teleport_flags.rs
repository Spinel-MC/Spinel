#[derive(Debug, Clone, Copy)]
pub struct TeleportFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub y_rot: bool,
    pub x_rot: bool,
}

impl TeleportFlags {
    pub fn from_byte(byte: i8) -> Self {
        Self {
            x: (byte & 0x01) != 0,
            y: (byte & 0x02) != 0,
            z: (byte & 0x04) != 0,
            y_rot: (byte & 0x08) != 0,
            x_rot: (byte & 0x10) != 0,
        }
    }

    pub fn to_byte(&self) -> i8 {
        let mut byte = 0;
        if self.x { byte |= 0x01; }
        if self.y { byte |= 0x02; }
        if self.z { byte |= 0x04; }
        if self.y_rot { byte |= 0x08; }
        if self.x_rot { byte |= 0x10; }
        byte
    }
}