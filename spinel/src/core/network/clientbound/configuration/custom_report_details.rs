use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: 0x0F)]
pub struct CustomReportDetailsPacket {
    pub description: String,
}
