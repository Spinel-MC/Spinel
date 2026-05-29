use crate::data_type::DataType;
use crate::types::math::Vector3d;
use std::io::{self, Read, Write};

const DATA_BITS_MASK: u64 = 0b111111111111111;
const MAX_QUANTIZED_VALUE: f64 = 32766.0;
const SCALE_BITS_MASK: u8 = 0b11;
const CONTINUATION_FLAG: u8 = 4;
const X_OFFSET: u64 = 3;
const Y_OFFSET: u64 = 18;
const Z_OFFSET: u64 = 33;
const ABS_MAX_VALUE: f64 = 1.7179869183E10;
const ABS_MIN_VALUE: f64 = 3.051944088384301E-5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vector3d);

impl DataType for Velocity {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let x = sanitize(self.0.x);
        let y = sanitize(self.0.y);
        let z = sanitize(self.0.z);
        let max = x.abs().max(y.abs()).max(z.abs());

        if max < ABS_MIN_VALUE {
            return 0u8.encode(writer);
        }

        let scale = max.ceil() as u64;
        let has_continuation = (scale & SCALE_BITS_MASK as u64) != scale;
        let flags = match has_continuation {
            true => (scale as u8 & SCALE_BITS_MASK) | CONTINUATION_FLAG,
            false => scale as u8,
        };
        let packed = flags as u64
            | pack(x / scale as f64) << X_OFFSET
            | pack(y / scale as f64) << Y_OFFSET
            | pack(z / scale as f64) << Z_OFFSET;

        (packed as u8).encode(writer)?;
        ((packed >> 8) as u8).encode(writer)?;
        ((packed >> 16) as i32).encode(writer)?;

        if has_continuation {
            crate::types::var_int::VarIntWrapper((scale >> 2) as i32).encode(writer)?;
        }

        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let flags = u8::decode(reader)?;

        if flags == 0 {
            return Ok(Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }));
        }

        let second_byte = u8::decode(reader)? as u64;
        let high_bytes = i32::decode(reader)? as u32 as u64;
        let packed = high_bytes << 16 | second_byte << 8 | flags as u64;
        let mut scale = (flags & SCALE_BITS_MASK) as u64;

        if (flags & CONTINUATION_FLAG) == CONTINUATION_FLAG {
            scale |= (crate::types::var_int::VarIntWrapper::decode(reader)?.0 as u32 as u64) << 2;
        }

        Ok(Velocity(Vector3d {
            x: unpack(packed >> X_OFFSET) * scale as f64,
            y: unpack(packed >> Y_OFFSET) * scale as f64,
            z: unpack(packed >> Z_OFFSET) * scale as f64,
        }))
    }
}

fn sanitize(value: f64) -> f64 {
    if value.is_nan() {
        return 0.0;
    }

    value.clamp(-ABS_MAX_VALUE, ABS_MAX_VALUE)
}

fn pack(value: f64) -> u64 {
    ((value * 0.5 + 0.5) * MAX_QUANTIZED_VALUE).round() as u64
}

fn unpack(value: u64) -> f64 {
    ((value & DATA_BITS_MASK) as f64).min(MAX_QUANTIZED_VALUE) * 2.0 / MAX_QUANTIZED_VALUE - 1.0
}
