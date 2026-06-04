use spinel::client::MinecraftClient;
use spinel::client::events::disconnect::DisconnectEvent;
use spinel::macros::event_listener;

#[event_listener]
fn on_disconnect(event: &mut DisconnectEvent, _client: &mut MinecraftClient) {
    let plain_reason = event.reason.to_plain_string();
    let reason = match plain_reason.is_empty() {
        true => event.reason.to_json_string(),
        false => plain_reason,
    };

    println!("Disconnected in {:?}: {}", event.state, reason);
}
