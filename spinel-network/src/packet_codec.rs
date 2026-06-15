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
    pub decode_then_encode: fn(&[u8]) -> io::Result<Vec<u8>>,
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

    pub fn codecs() -> Vec<&'static PacketCodec> {
        let mut codecs = PACKET_CODECS
            .get_or_init(Self::load_codecs)
            .values()
            .copied()
            .collect::<Vec<_>>();
        codecs.sort_by_key(|codec| {
            (
                recipient_order(codec.recipient),
                state_order((codec.state)()),
                (codec.id)(),
            )
        });
        codecs
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
                decode_then_encode: |payload| {
                    let mut payload_cursor = ::std::io::Cursor::new(payload);
                    let packet = <$packet_type as $crate::DataType>::decode(&mut payload_cursor)?;
                    let mut encoded_packet = Vec::new();
                    <$packet_type as $crate::DataType>::encode(&packet, &mut encoded_packet)?;
                    Ok(encoded_packet)
                },
            }
        }
    };
}

fn recipient_order(recipient: Recipient) -> u8 {
    match recipient {
        Recipient::Server => 0,
        Recipient::Client => 1,
    }
}

fn state_order(state: ConnectionState) -> u8 {
    match state {
        ConnectionState::Handshaking => 0,
        ConnectionState::Status => 1,
        ConnectionState::Login => 2,
        ConnectionState::Configuration => 3,
        ConnectionState::Play => 4,
    }
}
