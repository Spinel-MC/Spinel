use crate::entity::metadata::{
    EntityMetaCast, EntityMetaRef, LivingEntityMeta, LivingEntityMetaRef,
};
use crate::entity::player::{PlayerMeta, PlayerMetaRef};
use crate::entity::{ItemEntityMeta, ProjectileEntityMeta};

pub enum EntityMetadata<'entity> {
    Generic(EntityMetaCast<'entity>),
    Item(ItemEntityMeta<'entity>),
    Player(PlayerMeta<'entity>),
    Projectile(ProjectileEntityMeta<'entity>),
}

pub enum LivingEntityMetadata<'entity> {
    Generic(LivingEntityMeta<'entity>),
    Player(PlayerMeta<'entity>),
}

pub enum EntityMetadataRef<'entity> {
    Generic(EntityMetaRef<'entity>),
    Player(PlayerMetaRef<'entity>),
}

pub enum LivingEntityMetadataRef<'entity> {
    Generic(LivingEntityMetaRef<'entity>),
    Player(PlayerMetaRef<'entity>),
}
