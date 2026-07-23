#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown legacy equipment slot protocol id {0}")]
    UnknownLegacyEquipmentSlotProtocolId(i32),

    #[error("entity equipment mutation was rejected")]
    EquipmentMutationRejected,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
