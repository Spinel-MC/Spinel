use spinel_utils::Priority;

pub struct ListenerFn {
    pub call: fn(event: *mut (), server: *mut ()),
}

pub struct RegisteredEvent {
    pub name: &'static str,
    pub is_independent: bool,
}

pub struct RegisteredModule {
    pub name: &'static str,
}

pub struct RegisteredModuleDependency {
    pub subject_module: &'static str,
    pub dependent_on: &'static str,
}

pub struct RegisteredListener {
    pub event_name: &'static str,
    pub listener: ListenerFn,
    pub priority: Priority,
    pub dependent: bool,
    pub modules: &'static [&'static str],
}

inventory::collect!(&'static RegisteredEvent);
inventory::collect!(&'static RegisteredModule);
inventory::collect!(&'static RegisteredModuleDependency);
inventory::collect!(&'static RegisteredListener);