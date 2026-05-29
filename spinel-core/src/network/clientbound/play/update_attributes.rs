use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::types::{Identifier, VarInt};
use std::io::{self, Read, Write};

#[packet(id: "update_attributes", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct UpdateAttributesPacket {
    pub entity_id: VarInt,
    pub attributes: Vec<EntityAttribute>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityAttribute {
    pub attribute_id: VarInt,
    pub base_value: f64,
    pub modifiers: Vec<EntityAttributeModifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityAttributeModifier {
    pub modifier_id: Identifier,
    pub amount: f64,
    pub operation: VarInt,
}

impl EntityAttribute {
    pub fn attack_speed(base_value: f64, modifiers: Vec<EntityAttributeModifier>) -> Self {
        Self {
            attribute_id: VarInt::from(4),
            base_value,
            modifiers,
        }
    }
}

impl EntityAttributeModifier {
    pub fn attack_speed(modifier_id: Identifier, amount: f64) -> Self {
        Self {
            modifier_id,
            amount,
            operation: VarInt::from(0),
        }
    }

    pub fn base_attack_speed(amount: f64) -> Self {
        Self::attack_speed(Identifier::minecraft("base_attack_speed"), amount)
    }
}

impl DataType for EntityAttribute {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.attribute_id).encode(writer)?;
        self.base_value.encode(writer)?;
        self.modifiers.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            attribute_id: VarIntWrapper::decode(reader)?.0,
            base_value: f64::decode(reader)?,
            modifiers: Vec::<EntityAttributeModifier>::decode(reader)?,
        })
    }
}

impl DataType for EntityAttributeModifier {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.modifier_id.encode(writer)?;
        self.amount.encode(writer)?;
        VarIntWrapper(self.operation).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            modifier_id: Identifier::decode(reader)?,
            amount: f64::decode(reader)?,
            operation: VarIntWrapper::decode(reader)?.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{EntityAttribute, EntityAttributeModifier, UpdateAttributesPacket};
    use spinel_network::DataType;

    #[test]
    fn update_attributes_uses_play_packet_id_and_round_trips_attack_speed() {
        let packet = UpdateAttributesPacket {
            entity_id: 41,
            attributes: vec![EntityAttribute::attack_speed(
                4.0,
                vec![EntityAttributeModifier::base_attack_speed(-2.8)],
            )],
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = UpdateAttributesPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(UpdateAttributesPacket::get_id(), 0x81);
        assert_eq!(decoded_packet.entity_id, packet.entity_id);
        assert_eq!(decoded_packet.attributes, packet.attributes);
    }

    #[test]
    fn update_attributes_nested_var_int_fields_are_not_four_byte_integers() {
        let attribute = EntityAttribute::attack_speed(
            4.0,
            vec![EntityAttributeModifier::base_attack_speed(-2.8)],
        );
        let mut payload = Vec::new();

        attribute.encode(&mut payload).unwrap();

        assert_eq!(payload[0], 0x04);
        assert_eq!(payload[payload.len() - 1], 0x00);
    }
}
