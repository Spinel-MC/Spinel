use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use crate::wrappers::JsonTextComponent;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct ChatType {
    pub type_id: i32,
    pub name: TextComponent,
    pub target: Option<TextComponent>,
}

impl DataType for ChatType {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.type_id).encode(w)?;
        JsonTextComponent(self.name.clone()).encode(w)?;

        let target_wrapper = self.target.clone().map(JsonTextComponent);
        target_wrapper.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let type_id = VarIntWrapper::decode(r)?.0;
        let name = JsonTextComponent::decode(r)?.0;
        let target = Option::<JsonTextComponent>::decode(r)?.map(|w| w.0);

        Ok(ChatType {
            type_id,
            name,
            target,
        })
    }
}
