use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "custom_report_details", state: ConnectionState::Configuration)]
pub struct CustomReportDetailsPacket {
    pub description: String,
}
