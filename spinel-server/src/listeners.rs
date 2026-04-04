use crate::ServerPacketListener;
use spinel_network::ConnectionState;
use std::cmp::Reverse;
use std::collections::HashMap;

use crate::module_manager::ServerModuleManager;

pub fn get_listeners() -> (
    HashMap<(ConnectionState, i32), Vec<&'static ServerPacketListener>>,
    HashMap<ConnectionState, Vec<&'static ServerPacketListener>>,
) {
    let module_manager = ServerModuleManager::new();

    let active_unqualified_modules = &module_manager.active_unqualified_modules;
    let independent_events = &module_manager.independent_events;

    let mut assigned_packet_listener: HashMap<
        (ConnectionState, i32),
        Vec<&'static ServerPacketListener>,
    > = HashMap::new();
    let mut generic_packet_listener: HashMap<ConnectionState, Vec<&'static ServerPacketListener>> =
        HashMap::new();

    for listener in inventory::iter::<&'static ServerPacketListener>() {
        let module_is_active = listener.modules.is_empty()
            || listener
                .modules
                .iter()
                .any(|unqualified_name| active_unqualified_modules.contains(unqualified_name));

        let event_is_independent = listener.events.is_empty()
            || listener
                .events
                .iter()
                .any(|event_name| independent_events.contains(event_name));

        if module_is_active && event_is_independent {
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
    }

    for listener_vec in assigned_packet_listener.values_mut() {
        listener_vec.sort_by_key(|listener| Reverse(listener.priority.to_order()));
    }
    for listener_vec in generic_packet_listener.values_mut() {
        listener_vec.sort_by_key(|listener| Reverse(listener.priority.to_order()));
    }

    (assigned_packet_listener, generic_packet_listener)
}
