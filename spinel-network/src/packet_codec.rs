use crate::{ConnectionState, Recipient};
use std::collections::HashMap;
use std::io;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub struct PacketCodec {
    pub state: fn() -> ConnectionState,
    pub recipient: Recipient,
    pub id: fn() -> i32,
    pub packet_type: &'static str,
    pub decode: fn(&[u8]) -> io::Result<()>,
}

inventory::collect!(PacketCodec);

static PACKET_CODECS: OnceLock<HashMap<(Recipient, ConnectionState, i32), &'static PacketCodec>> =
    OnceLock::new();

pub struct PacketCodecRegistry;

impl PacketCodecRegistry {
    pub fn codec(
        recipient: Recipient,
        state: ConnectionState,
        packet_id: i32,
    ) -> Option<&'static PacketCodec> {
        PACKET_CODECS
            .get_or_init(Self::load_codecs)
            .get(&(recipient, state, packet_id))
            .copied()
    }

    pub fn contains(recipient: Recipient, state: ConnectionState, packet_id: i32) -> bool {
        Self::codec(recipient, state, packet_id).is_some()
    }

    fn load_codecs() -> HashMap<(Recipient, ConnectionState, i32), &'static PacketCodec> {
        inventory::iter::<PacketCodec>
            .into_iter()
            .map(|codec| ((codec.recipient, (codec.state)(), (codec.id)()), codec))
            .collect()
    }
}

#[macro_export]
macro_rules! register_packet_codec {
    ($packet_type:ty, $recipient:path) => {
        $crate::inventory::submit! {
            $crate::PacketCodec {
                state: <$packet_type as $crate::PacketStruct>::get_state,
                recipient: $recipient,
                id: <$packet_type as $crate::PacketStruct>::get_id,
                packet_type: stringify!($packet_type),
                decode: |payload| {
                    let mut payload_cursor = ::std::io::Cursor::new(payload);
                    <$packet_type as $crate::DataType>::decode(&mut payload_cursor)?;
                    if payload_cursor.position() != payload.len() as u64 {
                        return Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            format!(
                                "{} left {} unread payload bytes",
                                stringify!($packet_type),
                                payload.len() as u64 - payload_cursor.position(),
                            ),
                        ));
                    }
                    Ok(())
                },
            }
        }
    };
}
