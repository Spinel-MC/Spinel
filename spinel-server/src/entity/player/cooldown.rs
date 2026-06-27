use crate::entity::{Player, PlayerHand};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::cooldown::CooldownPacket;
use spinel_registry::data_components::vanilla_components::USE_COOLDOWN;
use spinel_registry::{ItemStack, UseCooldown};
use std::io;

impl Player {
    pub(crate) fn use_item_with_cooldown(
        &mut self,
        hand: PlayerHand,
        current_tick: u64,
        client: &mut Client,
    ) -> io::Result<bool> {
        let item_stack = self.get_item_in_hand(hand);
        let Some(use_cooldown) = item_stack.get(USE_COOLDOWN) else {
            return Ok(true);
        };
        let cooldown_group = item_cooldown_group(&item_stack, &use_cooldown);
        if self.item_cooldown_is_active(&cooldown_group, current_tick) {
            return Ok(false);
        }
        self.start_item_cooldown(cooldown_group, use_cooldown.ticks(), current_tick, client)?;
        Ok(true)
    }

    pub(crate) fn item_cooldown_is_active(&self, cooldown_group: &str, current_tick: u64) -> bool {
        self.item_cooldowns
            .get(cooldown_group)
            .is_some_and(|cooldown_expires_at| *cooldown_expires_at > current_tick)
    }

    fn start_item_cooldown(
        &mut self,
        cooldown_group: String,
        cooldown_ticks: i32,
        current_tick: u64,
        client: &mut Client,
    ) -> io::Result<()> {
        if cooldown_ticks <= 0 {
            return Ok(());
        }
        self.item_cooldowns.insert(
            cooldown_group.clone(),
            current_tick + u64::try_from(cooldown_ticks).unwrap_or(0),
        );
        CooldownPacket::new(cooldown_group, cooldown_ticks).dispatch(client)
    }
}

fn item_cooldown_group(item_stack: &ItemStack, use_cooldown: &UseCooldown) -> String {
    use_cooldown
        .cooldown_group()
        .map(str::to_string)
        .unwrap_or_else(|| item_stack.material().key().to_string())
}
