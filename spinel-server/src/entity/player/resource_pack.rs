use crate::entity::Player;
use spinel_core::network::clientbound::configuration::resource_pack_pop::ResourcePackPopPacket as ConfigurationResourcePackPopPacket;
use spinel_core::network::clientbound::configuration::resource_pack_push::ResourcePackPushPacket as ConfigurationResourcePackPushPacket;
use spinel_core::network::clientbound::play::resource_pack_pop::ResourcePackPopPacket;
use spinel_core::network::clientbound::play::resource_pack_push::ResourcePackPushPacket;
use spinel_core::network::resource_pack::{ResourcePackInfo, ResourcePackStatus};
use spinel_network::ConnectionState;
use spinel_network::DataType;
use spinel_network::PacketSender;
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::text::TextComponent;
use std::collections::BTreeMap;
use std::io;
use std::sync::Arc;
use uuid::Uuid;

pub type ResourcePackCallback = Arc<dyn Fn(ResourcePackStatus) + Send + Sync>;

#[derive(Clone)]
pub struct ResourcePackRequest {
    packs: Vec<ResourcePackInfo>,
    required: bool,
    replace: bool,
    prompt: Option<TextComponent>,
    callback: Option<ResourcePackCallback>,
}

#[derive(Clone)]
pub(crate) struct PendingResourcePack {
    required: bool,
    callback: Option<ResourcePackCallback>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourcePackFuture {
    pending_count: usize,
}

impl ResourcePackRequest {
    pub fn new(packs: Vec<ResourcePackInfo>) -> Self {
        Self {
            packs,
            required: false,
            replace: false,
            prompt: None,
            callback: None,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn replace(mut self, replace: bool) -> Self {
        self.replace = replace;
        self
    }

    pub fn prompt(mut self, prompt: Option<TextComponent>) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn callback(mut self, callback: ResourcePackCallback) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn get_packs(&self) -> &[ResourcePackInfo] {
        &self.packs
    }

    pub const fn is_required(&self) -> bool {
        self.required
    }

    pub const fn should_replace(&self) -> bool {
        self.replace
    }

    pub fn get_prompt_text(&self) -> Option<&TextComponent> {
        self.prompt.as_ref()
    }
}

impl PendingResourcePack {
    pub(crate) fn new(required: bool, callback: Option<ResourcePackCallback>) -> Self {
        Self { required, callback }
    }

    pub(crate) const fn is_required(&self) -> bool {
        self.required
    }

    pub(crate) fn notify(&self, status: ResourcePackStatus) {
        if let Some(callback) = &self.callback {
            callback(status);
        }
    }
}

impl ResourcePackFuture {
    pub const fn new(pending_count: usize) -> Self {
        Self { pending_count }
    }

    pub const fn get_pending_count(&self) -> usize {
        self.pending_count
    }
}

impl Player {
    pub fn send_resource_packs(&mut self, request: ResourcePackRequest) -> io::Result<()> {
        if request.should_replace() {
            self.clear_resource_packs()?;
        }
        for resource_pack in request.get_packs() {
            self.send_resource_pack_push_packet(ResourcePackPushPacket::new(
                resource_pack,
                request.is_required(),
                request.get_prompt_text().cloned(),
            ))?;
            self.pending_resource_packs.insert(
                resource_pack.id(),
                PendingResourcePack::new(request.is_required(), request.callback.clone()),
            );
        }
        Ok(())
    }

    pub fn remove_resource_packs(
        &mut self,
        id: Uuid,
        additional_ids: impl IntoIterator<Item = Uuid>,
    ) -> io::Result<()> {
        self.send_resource_pack_pop_packet(ResourcePackPopPacket::new(Some(id)))?;
        for additional_id in additional_ids {
            self.send_resource_pack_pop_packet(ResourcePackPopPacket::new(Some(additional_id)))?;
        }
        Ok(())
    }

    pub fn clear_resource_packs(&mut self) -> io::Result<()> {
        self.pending_resource_packs.clear();
        self.send_resource_pack_pop_packet(ResourcePackPopPacket::new(None))
    }

    pub fn get_resource_pack_future(&self) -> Option<ResourcePackFuture> {
        let pending_count = self.pending_resource_packs.len();
        if pending_count == 0 {
            return None;
        }
        Some(ResourcePackFuture::new(pending_count))
    }

    pub fn get_pending_resource_pack_count(&self) -> usize {
        self.pending_resource_packs.len()
    }

    pub fn on_resource_pack_status(
        &mut self,
        id: Uuid,
        status: ResourcePackStatus,
    ) -> io::Result<()> {
        let Some(pending_pack) = self.pending_resource_packs.remove(&id) else {
            return Ok(());
        };
        pending_pack.notify(status);
        let pack_failed_required_load =
            pending_pack.is_required() && status != ResourcePackStatus::SuccessfullyLoaded;
        if pack_failed_required_load {
            let disconnect_reason = TextComponent::literal_with_color(
                "Required resource pack was not loaded.",
                TextColor::from_named(NamedTextColor::Red),
            );
            return self.kick(disconnect_reason);
        }
        Ok(())
    }

    fn send_resource_pack_push_packet(&mut self, packet: ResourcePackPushPacket) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        match client.state {
            ConnectionState::Configuration => {
                let configuration_packet = ConfigurationResourcePackPushPacket {
                    id: packet.id,
                    url: packet.url,
                    hash: packet.hash,
                    forced: packet.forced,
                    prompt: packet.prompt,
                };
                let mut payload = Vec::new();
                configuration_packet.encode(&mut payload)?;
                client.send_packet(ConfigurationResourcePackPushPacket::get_id(), &payload)
            }
            ConnectionState::Play => {
                let mut payload = Vec::new();
                packet.encode(&mut payload)?;
                client.send_packet(ResourcePackPushPacket::get_id(), &payload)
            }
            _ => Ok(()),
        }
    }

    fn send_resource_pack_pop_packet(&mut self, packet: ResourcePackPopPacket) -> io::Result<()> {
        let Some(client) = self.get_client_mut() else {
            return Ok(());
        };
        match client.state {
            ConnectionState::Configuration => {
                let configuration_packet = ConfigurationResourcePackPopPacket { id: packet.id };
                let mut payload = Vec::new();
                configuration_packet.encode(&mut payload)?;
                client.send_packet(ConfigurationResourcePackPopPacket::get_id(), &payload)
            }
            ConnectionState::Play => {
                let mut payload = Vec::new();
                packet.encode(&mut payload)?;
                client.send_packet(ResourcePackPopPacket::get_id(), &payload)
            }
            _ => Ok(()),
        }
    }
}

impl Default for ResourcePackRequest {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

pub(crate) type PendingResourcePacks = BTreeMap<Uuid, PendingResourcePack>;
