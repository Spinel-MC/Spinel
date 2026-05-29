use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::Slot;
use spinel_network::types::VarInt;
use std::io::{self, Read, Write};

#[packet(id: "set_equipment", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct SetEquipmentPacket {
    pub entity_id: VarInt,
    pub equipment: EntityEquipment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityEquipment(pub Vec<EntityEquipmentEntry>);

#[derive(Clone, Debug, PartialEq)]
pub struct EntityEquipmentEntry {
    pub slot: EntityEquipmentSlot,
    pub item: Slot,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityEquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
    Body,
    Saddle,
}

impl EntityEquipmentSlot {
    pub const fn legacy_protocol_id(self) -> i8 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
            Self::Boots => 2,
            Self::Leggings => 3,
            Self::Chestplate => 4,
            Self::Helmet => 5,
            Self::Body => 6,
            Self::Saddle => 7,
        }
    }

    pub const fn from_legacy_protocol_id(protocol_id: i8) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::MainHand),
            1 => Some(Self::OffHand),
            2 => Some(Self::Boots),
            3 => Some(Self::Leggings),
            4 => Some(Self::Chestplate),
            5 => Some(Self::Helmet),
            6 => Some(Self::Body),
            7 => Some(Self::Saddle),
            _ => None,
        }
    }
}

impl DataType for EntityEquipment {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.0.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Equipment entries cannot be empty",
            ));
        }
        for (entry_index, entry) in self.0.iter().enumerate() {
            let is_last_entry = entry_index == self.0.len() - 1;
            let mut slot = entry.slot.legacy_protocol_id();
            if !is_last_entry {
                slot |= -0x80i8;
            }
            slot.encode(writer)?;
            entry.item.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut entries = Vec::new();
        loop {
            let slot_byte = i8::decode(reader)?;
            let slot_id = slot_byte & 0x7F;
            let slot = EntityEquipmentSlot::from_legacy_protocol_id(slot_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Unknown entity equipment slot: {slot_id}"),
                )
            })?;
            entries.push(EntityEquipmentEntry {
                slot,
                item: Slot::decode(reader)?,
            });
            if slot_byte & -0x80i8 == 0 {
                return Ok(Self(entries));
            }
        }
    }
}

impl SetEquipmentPacket {
    pub fn new(entity_id: i32, equipment: Vec<EntityEquipmentEntry>) -> Self {
        Self {
            entity_id,
            equipment: EntityEquipment(equipment),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EntityEquipmentEntry, EntityEquipmentSlot, SetEquipmentPacket};
    use spinel_network::DataType;
    use spinel_network::types::Slot;

    #[test]
    fn set_equipment_packet_uses_minestom_legacy_slot_continuation_bit() {
        let packet = SetEquipmentPacket::new(
            7,
            vec![
                EntityEquipmentEntry {
                    slot: EntityEquipmentSlot::MainHand,
                    item: Slot::default(),
                },
                EntityEquipmentEntry {
                    slot: EntityEquipmentSlot::OffHand,
                    item: Slot::default(),
                },
            ],
        );
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(SetEquipmentPacket::get_id(), 0x64);
        assert_eq!(payload[1] as i8, -128);
        assert_eq!(payload[3], 1);
    }
}
