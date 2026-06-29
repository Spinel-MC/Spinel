use crate::entity::Entity;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PassengerError {
    #[error("vehicle is not assigned to a world")]
    VehicleHasNoWorld,

    #[error("passenger is not assigned to a world")]
    PassengerHasNoWorld,

    #[error("vehicle and passenger belong to different worlds")]
    DifferentWorlds {
        vehicle_world: Uuid,
        passenger_world: Uuid,
    },

    #[error("an entity cannot be its own passenger")]
    SameEntity,

    #[error("the vehicle cannot be added as its own passenger")]
    PassengerIsVehicle,

    #[error("passenger is attached to a different vehicle")]
    PassengerHasDifferentVehicle,
}

impl Entity {
    pub fn add_passenger(&mut self, passenger: &mut Entity) -> Result<bool, PassengerError> {
        if self.get_entity_id() == passenger.get_entity_id() {
            return Err(PassengerError::SameEntity);
        }
        if self.get_vehicle() == Some(passenger.get_entity_id()) {
            return Err(PassengerError::PassengerIsVehicle);
        }
        let vehicle_world = self.get_world().ok_or(PassengerError::VehicleHasNoWorld)?;
        passenger
            .get_world()
            .ok_or(PassengerError::PassengerHasNoWorld)?;
        if passenger.get_world() != Some(vehicle_world) {
            passenger.assign_world(vehicle_world);
        }
        if passenger
            .get_vehicle()
            .is_some_and(|vehicle_id| vehicle_id != self.get_entity_id())
        {
            return Err(PassengerError::PassengerHasDifferentVehicle);
        }
        if !self.attach_passenger(passenger.get_entity_id()) {
            return Ok(false);
        }
        passenger.set_vehicle(self.get_entity_id());
        passenger.set_position(self.get_passenger_position(passenger));
        Ok(true)
    }

    pub fn remove_passenger(&mut self, passenger: &mut Entity) -> Result<bool, PassengerError> {
        let vehicle_world = self.get_world().ok_or(PassengerError::VehicleHasNoWorld)?;
        passenger
            .get_world()
            .ok_or(PassengerError::PassengerHasNoWorld)?;
        if passenger.get_world() != Some(vehicle_world) {
            passenger.assign_world(vehicle_world);
        }
        if !self.detach_passenger(passenger.get_entity_id()) {
            return Ok(false);
        }
        passenger.clear_vehicle();
        Ok(true)
    }
}
