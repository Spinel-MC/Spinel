use crate::ClientPacketListener;
use spinel_network::ConnectionState;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn get_client_listeners() -> (
    HashMap<(ConnectionState, i32), Vec<&'static ClientPacketListener>>,
    HashMap<ConnectionState, Vec<&'static ClientPacketListener>>,
) {
    let mut assigned_packet_listener: HashMap<
        (ConnectionState, i32),
        Vec<&'static ClientPacketListener>,
    > = HashMap::new();
    let mut generic_packet_listener: HashMap<ConnectionState, Vec<&'static ClientPacketListener>> =
        HashMap::new();

    for listener in inventory::iter::<&'static ClientPacketListener>() {
        if listener.id == -1 {
            generic_packet_listener
                .entry(listener.state)
                .or_default()
                .push(listener);
        } else {
            let key = (listener.state, listener.id);
            assigned_packet_listener
                .entry(key)
                .or_default()
                .push(listener);
        }
    }

    for listener_vec in assigned_packet_listener.values_mut() {
        listener_vec.sort_by_key(|listener| Reverse(listener.priority.to_order()));
    }
    for listener_vec in generic_packet_listener.values_mut() {
        listener_vec.sort_by_key(|listener| Reverse(listener.priority.to_order()));
    }

    (assigned_packet_listener, generic_packet_listener)
}
