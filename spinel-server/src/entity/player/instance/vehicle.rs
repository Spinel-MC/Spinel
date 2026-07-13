use crate::entity::EntityId;
use spinel_core::network::clientbound::play::set_passengers::SetPassengersPacket;
use spinel_network::types::IntList;
use std::collections::BTreeSet;

use super::state::Player;

impl Player {
    pub const fn get_vehicle(&self) -> Option<EntityId> {
        self.vehicle
    }

    pub(crate) fn set_vehicle(&mut self, vehicle: EntityId) {
        self.vehicle = Some(vehicle);
    }

    pub(crate) fn clear_vehicle(&mut self) {
        self.vehicle = None;
    }

    pub(crate) fn add_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.insert(passenger_id)
    }

    pub(crate) fn remove_passenger(&mut self, passenger_id: EntityId) -> bool {
        self.passengers.remove(&passenger_id)
    }

    pub fn has_passenger(&self) -> bool {
        !self.passengers.is_empty()
    }

    pub fn get_passengers(&self) -> &BTreeSet<EntityId> {
        &self.passengers
    }

    pub(crate) fn get_passenger_packet(&self) -> SetPassengersPacket {
        SetPassengersPacket {
            vehicle_entity_id: self.get_entity_id().get_value(),
            passenger_entity_ids: IntList(
                self.passengers
                    .iter()
                    .map(|passenger_id| passenger_id.get_value())
                    .collect(),
            ),
        }
    }

    pub fn get_leashed_entities(&self) -> &BTreeSet<EntityId> {
        self.leash.get_leashed_entities()
    }

    pub const fn get_leash_holder(&self) -> Option<EntityId> {
        self.leash.get_holder()
    }

    pub(crate) fn set_leash_holder(&mut self, leash_holder: Option<EntityId>) {
        self.leash.set_holder(leash_holder);
    }

    pub(crate) fn add_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leash.add_leashed_entity(entity_id)
    }

    pub(crate) fn remove_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leash.remove_leashed_entity(entity_id)
    }

    pub(crate) fn get_attach_entity_packet(
        &self,
    ) -> spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket {
        self.leash.get_packet(self.entity_id)
    }

    pub const fn is_vanished(&self) -> bool {
        self.vanished
    }

    pub fn set_vanished(&mut self, vanished: bool) {
        self.vanished = vanished;
    }
}
