use spinel::{
    core::network::serverbound::login::login_acknowledge::LoginAcknowledgedPacket,
    macros::packet_listener, network::Client, server::MinecraftServer,
};

#[packet_listener()]
fn on_login_success(
    client: &mut Client,
    _packet: LoginAcknowledgedPacket,
    _server: &mut MinecraftServer,
) -> bool {
    println!("Client {} logged in successfully.", client.addr);
    true
}
