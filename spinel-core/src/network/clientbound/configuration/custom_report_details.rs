use spinel_macros::packet;

#[packet(id: "custom_report_details", generate_fields: true, state: ConnectionState::Configuration, recipient: Recipient::Client)]
pub struct CustomReportDetailsPacket;
