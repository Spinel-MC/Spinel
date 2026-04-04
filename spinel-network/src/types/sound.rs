use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub enum SoundEvent {
    Id(i32),
    Named {
        name: String,
        fixed_range: Option<f32>,
    },
}

impl DataType for SoundEvent {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            SoundEvent::Id(id) => {
                VarIntWrapper(*id + 1).encode(w)?;
            }
            SoundEvent::Named { name, fixed_range } => {
                VarIntWrapper(0).encode(w)?;
                name.encode(w)?;
                fixed_range.encode(w)?;
            }
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let id = VarIntWrapper::decode(r)?.0;
        if id == 0 {
            let name = String::decode(r)?;
            let fixed_range = Option::<f32>::decode(r)?;
            Ok(SoundEvent::Named { name, fixed_range })
        } else {
            Ok(SoundEvent::Id(id - 1))
        }
    }
}
