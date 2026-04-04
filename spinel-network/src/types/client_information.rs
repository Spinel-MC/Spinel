use crate::data_type::DataType;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_status: i32,
}

impl Default for ClientInformation {
    fn default() -> Self {
        Self {
            locale: "en_US".to_string(),
            view_distance: 12,
            chat_mode: 0,
            chat_colors: true,
            displayed_skin_parts: 127,
            main_hand: 1,
            enable_text_filtering: false,
            allow_server_listings: true,
            particle_status: 0,
        }
    }
}

impl DataType for ClientInformation {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.locale.encode(w)?;
        self.view_distance.encode(w)?;
        crate::types::var_int::VarIntWrapper(self.chat_mode).encode(w)?;
        self.chat_colors.encode(w)?;
        self.displayed_skin_parts.encode(w)?;
        crate::types::var_int::VarIntWrapper(self.main_hand).encode(w)?;
        self.enable_text_filtering.encode(w)?;
        self.allow_server_listings.encode(w)?;
        crate::types::var_int::VarIntWrapper(self.particle_status).encode(w)?;
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            locale: String::decode(r)?,
            view_distance: i8::decode(r)?,
            chat_mode: crate::types::var_int::VarIntWrapper::decode(r)?.0,
            chat_colors: bool::decode(r)?,
            displayed_skin_parts: u8::decode(r)?,
            main_hand: crate::types::var_int::VarIntWrapper::decode(r)?.0,
            enable_text_filtering: bool::decode(r)?,
            allow_server_listings: bool::decode(r)?,
            particle_status: crate::types::var_int::VarIntWrapper::decode(r)?.0,
        })
    }
}
