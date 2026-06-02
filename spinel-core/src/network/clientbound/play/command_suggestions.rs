use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, DataType, PacketStruct, VarIntWrapper};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSuggestionsPacket {
    pub transaction_id: i32,
    pub start: i32,
    pub length: i32,
    pub matches: Vec<CommandSuggestionMatch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandSuggestionMatch {
    pub text: String,
    pub tooltip: Option<TextComponent>,
}

impl CommandSuggestionsPacket {
    pub fn empty(transaction_id: i32) -> Self {
        Self {
            transaction_id,
            start: 0,
            length: 0,
            matches: Vec::new(),
        }
    }
}

impl CommandSuggestionMatch {
    pub fn new(text: impl Into<String>, tooltip: Option<TextComponent>) -> Self {
        Self {
            text: text.into(),
            tooltip,
        }
    }
}

impl DataType for CommandSuggestionsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.transaction_id).encode(writer)?;
        VarIntWrapper(self.start).encode(writer)?;
        VarIntWrapper(self.length).encode(writer)?;
        VarIntWrapper(self.matches.len() as i32).encode(writer)?;
        self.matches
            .iter()
            .try_for_each(|suggestion_match| suggestion_match.encode(writer))
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let transaction_id = VarIntWrapper::decode(reader)?.0;
        let start = VarIntWrapper::decode(reader)?.0;
        let length = VarIntWrapper::decode(reader)?.0;
        let match_count = VarIntWrapper::decode(reader)?.0 as usize;
        let matches = (0..match_count)
            .map(|_| CommandSuggestionMatch::decode(reader))
            .collect::<io::Result<Vec<_>>>()?;
        Ok(Self {
            transaction_id,
            start,
            length,
            matches,
        })
    }
}

impl PacketStruct for CommandSuggestionsPacket {
    fn get_id() -> i32 {
        0x0f
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for CommandSuggestionMatch {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.text.encode(writer)?;
        self.tooltip.clone().map(JsonTextComponent).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            text: String::decode(reader)?,
            tooltip: Option::<JsonTextComponent>::decode(reader)?.map(|tooltip| tooltip.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandSuggestionMatch, CommandSuggestionsPacket};
    use spinel_network::{DataType, PacketStruct};
    use spinel_utils::component::text::TextComponent;

    #[test]
    fn command_suggestions_round_trips_minestom_shape() {
        let packet = CommandSuggestionsPacket {
            transaction_id: 4,
            start: 1,
            length: 5,
            matches: vec![CommandSuggestionMatch::new(
                "spawn",
                Some(TextComponent::literal("Spawn command")),
            )],
        };

        let mut payload = Vec::new();
        packet.encode(&mut payload).unwrap();
        let decoded_packet = CommandSuggestionsPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(CommandSuggestionsPacket::get_id(), 0x0f);
        assert_eq!(decoded_packet, packet);
    }
}
