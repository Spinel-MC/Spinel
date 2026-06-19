use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use crate::wrappers::JsonTextComponent;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum ScoreNumberFormat {
    Blank,
    Styled {
        encoded_style_payload: Vec<u8>,
    },
    Fixed {
        value: TextComponent,
    },
    Unknown {
        format_type_id: i32,
        encoded_payload: Vec<u8>,
    },
}

impl ScoreNumberFormat {
    const BLANK_FORMAT_TYPE_ID: i32 = 0;
    const STYLED_FORMAT_TYPE_ID: i32 = 1;
    const FIXED_FORMAT_TYPE_ID: i32 = 2;
}

impl DataType for ScoreNumberFormat {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Blank => VarIntWrapper(Self::BLANK_FORMAT_TYPE_ID).encode(writer),
            Self::Styled {
                encoded_style_payload,
            } => {
                VarIntWrapper(Self::STYLED_FORMAT_TYPE_ID).encode(writer)?;
                writer.write_all(encoded_style_payload)
            }
            Self::Fixed { value } => {
                VarIntWrapper(Self::FIXED_FORMAT_TYPE_ID).encode(writer)?;
                JsonTextComponent(value.clone()).encode(writer)
            }
            Self::Unknown {
                format_type_id,
                encoded_payload,
            } => {
                VarIntWrapper(*format_type_id).encode(writer)?;
                writer.write_all(encoded_payload)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let format_type_id = VarIntWrapper::decode(reader)?.0;
        match format_type_id {
            Self::BLANK_FORMAT_TYPE_ID => Ok(Self::Blank),
            Self::FIXED_FORMAT_TYPE_ID => Ok(Self::Fixed {
                value: JsonTextComponent::decode(reader)?.0,
            }),
            Self::STYLED_FORMAT_TYPE_ID => {
                let mut encoded_style_payload = Vec::new();
                reader.read_to_end(&mut encoded_style_payload)?;
                Ok(Self::Styled {
                    encoded_style_payload,
                })
            }
            _ => {
                let mut encoded_payload = Vec::new();
                reader.read_to_end(&mut encoded_payload)?;
                Ok(Self::Unknown {
                    format_type_id,
                    encoded_payload,
                })
            }
        }
    }
}
