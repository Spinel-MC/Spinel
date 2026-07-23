use crate::entity::{Entity, Error};

impl Entity {
    pub fn add_passenger(&mut self, passenger: &mut Entity) -> Result<bool, Error> {
        if self.get_entity_id() == passenger.get_entity_id() {
            return Err(Error::PassengerIsSelf);
        }
        if self.get_vehicle() == Some(passenger.get_entity_id()) {
            return Err(Error::PassengerIsVehicle);
        }
        let vehicle_world = self.get_world().ok_or(Error::VehicleHasNoWorld)?;
        passenger.get_world().ok_or(Error::PassengerHasNoWorld)?;
        if passenger.get_world() != Some(vehicle_world) {
            passenger.assign_world(vehicle_world);
        }
        if passenger
            .get_vehicle()
            .is_some_and(|vehicle_id| vehicle_id != self.get_entity_id())
        {
            return Err(Error::PassengerHasDifferentVehicle);
        }
        if !self.attach_passenger(passenger.get_entity_id()) {
            return Ok(false);
        }
        passenger.set_vehicle(self.get_entity_id());
        passenger.set_position(self.get_passenger_position(passenger));
        Ok(true)
    }

    pub fn remove_passenger(&mut self, passenger: &mut Entity) -> Result<bool, Error> {
        let vehicle_world = self.get_world().ok_or(Error::VehicleHasNoWorld)?;
        passenger.get_world().ok_or(Error::PassengerHasNoWorld)?;
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
