use spinel_network::{ConnectionState, PacketListener};
use std::cmp::Reverse;
use std::collections::HashMap;

use crate::core::server::modules::ModuleManager;

pub fn get_listeners() -> (
    HashMap<(ConnectionState, i32), Vec<&'static PacketListener>>,
    HashMap<ConnectionState, Vec<&'static PacketListener>>,
) {
    let module_manager = ModuleManager::new();

    let active_unqualified_modules = &module_manager.active_unqualified_modules;
    let independent_events = &module_manager.independent_events;

    // Initialize listener maps.
    let mut assigned_packet_listener: HashMap<
        (ConnectionState, i32),
        Vec<&'static PacketListener>,
    > = HashMap::new();
    let mut generic_packet_listener: HashMap<ConnectionState, Vec<&'static PacketListener>> =
        HashMap::new();

    // Filter listeners based on active modules and events.
    for listener in inventory::iter::<&'static PacketListener>() {
        // Check if the listener's required modules are active.
        let module_is_active = listener.modules.is_empty()
            || listener
                .modules
                .iter()
                .any(|unqualified_name| active_unqualified_modules.contains(unqualified_name));

        // Check if the listener's required events are independent.
        let event_is_independent = listener.events.is_empty()
            || listener
                .events
                .iter()
                .any(|event_name| independent_events.contains(event_name));

        if module_is_active && event_is_independent {
            // Place listeners with ID -1 in the generic map.
            if listener.id == -1 {
                generic_packet_listener
                    .entry(listener.state)
                    .or_default()
                    .push(listener);
            } else {
                // Place all other listeners in the assigned map.
                let key = (listener.state, listener.id);
                assigned_packet_listener
                    .entry(key)
                    .or_default()
                    .push(listener);
            }
        }
    }

    // Sort listeners by priority.
    for listener_vec in assigned_packet_listener.values_mut() {
        listener_vec.sort_by_key(|listener| Reverse(listener.priority.to_order()));
    }
    for listener_vec in generic_packet_listener.values_mut() {
        listener_vec.sort_by_key(|listener| Reverse(listener.priority.to_order()));
    }

    (assigned_packet_listener, generic_packet_listener)
}
