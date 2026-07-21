use crate::server::MinecraftServer;
use spinel_events::{Event, EventContext};
use spinel_utils::Priority;
use std::any::TypeId;
use std::cmp::Reverse;
use std::collections::HashMap;

pub type ServerEventHandler<E> = fn(&mut E, &mut MinecraftServer);
pub type ServerEventHandlerMethod<T, E> = fn(&T, &mut E, &mut MinecraftServer);

type ErasedServerEventHandler = unsafe fn(usize, usize, *mut (), *mut MinecraftServer);

#[derive(Clone, Copy)]
pub struct EventHandlerEntry {
    priority: Priority,
    receiver: usize,
    handler: usize,
    dispatch: ErasedServerEventHandler,
}

pub struct GlobalEventHandler {
    handlers: HashMap<TypeId, Vec<EventHandlerEntry>>,
}

pub trait EventHandler {
    fn register_event_handlers(&self, global_event_handler: &mut GlobalEventHandler);
}

impl GlobalEventHandler {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_listener<E: Event + 'static>(&mut self, handler: ServerEventHandler<E>) {
        self.add_listener_with_priority(Priority::Medium, handler);
    }

    pub fn add_listener_with_priority<E: Event + 'static>(
        &mut self,
        priority: Priority,
        handler: ServerEventHandler<E>,
    ) {
        let event_handlers = self.handlers.entry(TypeId::of::<E>()).or_default();
        event_handlers.push(EventHandlerEntry {
            priority,
            receiver: 0,
            handler: handler as usize,
            dispatch: dispatch_server_event::<E>,
        });
        event_handlers.sort_by_key(|event_handler| Reverse(event_handler.priority.to_order()));
    }

    pub fn add_method_listener_with_priority<T, E: Event + 'static>(
        &mut self,
        priority: Priority,
        receiver: &T,
        handler: ServerEventHandlerMethod<T, E>,
    ) {
        let event_handlers = self.handlers.entry(TypeId::of::<E>()).or_default();
        event_handlers.push(EventHandlerEntry {
            priority,
            receiver: receiver as *const T as usize,
            handler: handler as usize,
            dispatch: dispatch_server_event_method::<T, E>,
        });
        event_handlers.sort_by_key(|event_handler| Reverse(event_handler.priority.to_order()));
    }

    pub fn dispatch<E: Event + 'static>(&mut self, event: &mut E, server: *mut MinecraftServer) {
        let event_type = TypeId::of::<E>();
        let Some(event_handlers) = self.handlers.remove(&event_type) else {
            return;
        };
        let event_ptr = event as *mut E as *mut ();
        event_handlers.iter().for_each(|event_handler| unsafe {
            (event_handler.dispatch)(
                event_handler.receiver,
                event_handler.handler,
                event_ptr,
                server,
            );
        });
        self.handlers.insert(event_type, event_handlers);
    }
}

impl Default for GlobalEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Event + 'static> EventContext<E> for MinecraftServer {
    fn dispatch_registered_event(&mut self, event: &mut E) {
        let server = self as *mut Self;
        self.global_event_handler.dispatch(event, server);
    }
}

unsafe fn dispatch_server_event<E: Event + 'static>(
    _receiver: usize,
    handler: usize,
    event: *mut (),
    server: *mut MinecraftServer,
) {
    let handler: ServerEventHandler<E> = unsafe { std::mem::transmute(handler) };
    handler(unsafe { &mut *(event as *mut E) }, unsafe { &mut *server });
}

unsafe fn dispatch_server_event_method<T, E: Event + 'static>(
    receiver: usize,
    handler: usize,
    event: *mut (),
    server: *mut MinecraftServer,
) {
    let receiver = unsafe { &*(receiver as *const T) };
    let handler: ServerEventHandlerMethod<T, E> = unsafe { std::mem::transmute(handler) };
    handler(receiver, unsafe { &mut *(event as *mut E) }, unsafe {
        &mut *server
    });
}
