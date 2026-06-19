use spinel_network::data_type::DataType;
use spinel_network::types::{Statistic, var_int::VarIntWrapper};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwardStatValue {
    pub statistic: Statistic,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwardStatsPacket {
    pub stats: Vec<AwardStatValue>,
}

impl DataType for AwardStatsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.stats.len() as i32).encode(writer)?;
        self.stats.iter().try_for_each(|statistic_value| {
            statistic_value.statistic.encode(writer)?;
            VarIntWrapper(statistic_value.value).encode(writer)
        })
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let statistic_count = VarIntWrapper::decode(reader)?.0;
        if statistic_count < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "award stat count cannot be negative",
            ));
        }

        let stats = (0..statistic_count)
            .map(|_| {
                Ok(AwardStatValue {
                    statistic: Statistic::decode(reader)?,
                    value: VarIntWrapper::decode(reader)?.0,
                })
            })
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Self { stats })
    }
}

impl PacketStruct for AwardStatsPacket {
    fn get_id() -> i32 {
        0x03
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(AwardStatsPacket, spinel_network::Recipient::Client);
