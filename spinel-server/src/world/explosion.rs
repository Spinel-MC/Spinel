use crate::entity::EntityPosition;
use crate::world::{Block, BlockPosition, World};
use spinel_core::network::clientbound::play::explosion::{ExplosionPacket, ExplosionParticle};
use spinel_nbt::NbtCompound;
use spinel_network::types::Vector3d;
use spinel_network::types::sound::SoundEvent;
use std::fmt;
use std::sync::Arc;

type PostExplosion = Arc<dyn Fn(&mut World, &[BlockPosition], &mut ExplosionPacket) + Send + Sync>;
type PostSend = Arc<dyn Fn(&mut World, &[BlockPosition]) + Send + Sync>;
type PrepareExplosion =
    Arc<dyn Fn(&mut World, &[BlockPosition]) -> Vec<BlockPosition> + Send + Sync>;

#[derive(Clone)]
pub struct Explosion {
    center: EntityPosition,
    strength: f32,
    affected_blocks: Vec<BlockPosition>,
    prepare: Option<PrepareExplosion>,
    post_explosion: Option<PostExplosion>,
    post_send: Option<PostSend>,
}

pub trait ExplosionSupplier: Send + Sync {
    fn create_explosion(&self, center: EntityPosition, strength: f32) -> Explosion;

    fn create_explosion_with_data(
        &self,
        center: EntityPosition,
        strength: f32,
        _additional_data: Option<&NbtCompound>,
    ) -> Explosion {
        self.create_explosion(center, strength)
    }
}

impl<F> ExplosionSupplier for F
where
    F: Fn(EntityPosition, f32) -> Explosion + Send + Sync,
{
    fn create_explosion(&self, center: EntityPosition, strength: f32) -> Explosion {
        self(center, strength)
    }
}

impl Explosion {
    pub fn new(center: EntityPosition, strength: f32, affected_blocks: Vec<BlockPosition>) -> Self {
        Self {
            center,
            strength,
            affected_blocks,
            prepare: None,
            post_explosion: None,
            post_send: None,
        }
    }

    pub fn with_prepare(
        mut self,
        prepare: impl Fn(&mut World, &[BlockPosition]) -> Vec<BlockPosition> + Send + Sync + 'static,
    ) -> Self {
        self.prepare = Some(Arc::new(prepare));
        self
    }

    pub fn with_post_explosion(
        mut self,
        post_explosion: impl Fn(&mut World, &[BlockPosition], &mut ExplosionPacket)
        + Send
        + Sync
        + 'static,
    ) -> Self {
        self.post_explosion = Some(Arc::new(post_explosion));
        self
    }

    pub fn with_post_send(
        mut self,
        post_send: impl Fn(&mut World, &[BlockPosition]) + Send + Sync + 'static,
    ) -> Self {
        self.post_send = Some(Arc::new(post_send));
        self
    }

    pub const fn center(&self) -> EntityPosition {
        self.center
    }

    pub const fn strength(&self) -> f32 {
        self.strength
    }

    pub fn affected_blocks(&self) -> &[BlockPosition] {
        &self.affected_blocks
    }

    pub fn prepare(&self, world: &mut World) -> Vec<BlockPosition> {
        if let Some(prepare) = &self.prepare {
            return prepare(world, &self.affected_blocks);
        }
        self.affected_blocks.clone()
    }

    pub fn apply(&self, world: &mut World) -> std::io::Result<Vec<BlockPosition>> {
        let affected_blocks = self.prepare(world);
        affected_blocks
            .iter()
            .try_for_each(|position| world.set_block(*position, Block::AIR).map(|_| ()))?;
        let mut packet = self.packet();
        if let Some(post_explosion) = &self.post_explosion {
            post_explosion(world, &affected_blocks, &mut packet);
        }
        world.dispatch_packet_to_players(packet)?;
        if let Some(post_send) = &self.post_send {
            post_send(world, &affected_blocks);
        }
        Ok(affected_blocks)
    }

    pub fn packet(&self) -> ExplosionPacket {
        ExplosionPacket {
            center: Vector3d {
                x: self.center.get_x(),
                y: self.center.get_y(),
                z: self.center.get_z(),
            },
            radius: 0.0,
            block_count: 0,
            player_knockback: Some(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            particle: ExplosionParticle::Explosion,
            sound: SoundEvent::Id(668),
            block_particles: Vec::new(),
        }
    }
}

impl fmt::Debug for Explosion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Explosion")
            .field("center", &self.center)
            .field("strength", &self.strength)
            .field("affected_blocks", &self.affected_blocks)
            .finish()
    }
}

impl PartialEq for Explosion {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
            && self.strength == other.strength
            && self.affected_blocks == other.affected_blocks
    }
}
