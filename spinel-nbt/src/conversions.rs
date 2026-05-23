use crate::Nbt;

impl From<i8> for Nbt {
    fn from(value: i8) -> Self {
        Self::Byte(value)
    }
}

impl From<bool> for Nbt {
    fn from(value: bool) -> Self {
        Self::Byte(i8::from(value))
    }
}

impl From<i16> for Nbt {
    fn from(value: i16) -> Self {
        Self::Short(value)
    }
}

impl From<i32> for Nbt {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<i64> for Nbt {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<f32> for Nbt {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<f64> for Nbt {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<String> for Nbt {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Nbt {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}
