use crate::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

macro_rules! protocol_enum {
    ($name:ident { $($variant:ident = $protocol_id:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant),+
        }

        impl DataType for $name {
            fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
                VarIntWrapper(match self {
                    $(Self::$variant => $protocol_id),+
                }).encode(writer)
            }

            fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
                match VarIntWrapper::decode(reader)?.0 {
                    $($protocol_id => Ok(Self::$variant)),+,
                    protocol_id => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("unknown {} protocol id {protocol_id}", stringify!($name)),
                    )),
                }
            }
        }
    };
}

protocol_enum!(StructureBlockUpdateType {
    UpdateData = 0,
    SaveArea = 1,
    LoadArea = 2,
    ScanArea = 3,
});

protocol_enum!(StructureBlockMode {
    Save = 0,
    Load = 1,
    Corner = 2,
    Data = 3,
});

protocol_enum!(StructureBlockMirror {
    None = 0,
    LeftRight = 1,
    FrontBack = 2,
});
