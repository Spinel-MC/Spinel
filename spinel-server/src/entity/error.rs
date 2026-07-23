#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown legacy equipment slot protocol id {0}")]
    UnknownLegacyEquipmentSlotProtocolId(i32),

    #[error("entity equipment mutation was rejected")]
    EquipmentMutationRejected,

    #[error("vehicle is not assigned to a world")]
    VehicleHasNoWorld,

    #[error("passenger is not assigned to a world")]
    PassengerHasNoWorld,

    #[error("an entity cannot be its own passenger")]
    PassengerIsSelf,

    #[error("the vehicle cannot be added as its own passenger")]
    PassengerIsVehicle,

    #[error("passenger is attached to a different vehicle")]
    PassengerHasDifferentVehicle,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
