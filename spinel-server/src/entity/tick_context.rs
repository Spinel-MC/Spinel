use crate::network::connection_manager::ConnectionManager;

pub(crate) struct EntityTickContext<'a> {
    pub connections: &'a ConnectionManager,
}
