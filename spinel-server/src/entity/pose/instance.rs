use spinel_registry::EntityBoundingBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityPose {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    LongJumping,
    Dying,
    Croaking,
    UsingTongue,
    Sitting,
    Roaring,
    Sniffing,
    Emerging,
    Digging,
    Sliding,
    Shooting,
    Inhaling,
}

impl EntityPose {
    pub const fn get_protocol_id(self) -> i32 {
        match self {
            Self::Standing => 0,
            Self::FallFlying => 1,
            Self::Sleeping => 2,
            Self::Swimming => 3,
            Self::SpinAttack => 4,
            Self::Sneaking => 5,
            Self::LongJumping => 6,
            Self::Dying => 7,
            Self::Croaking => 8,
            Self::UsingTongue => 9,
            Self::Sitting => 10,
            Self::Roaring => 11,
            Self::Sniffing => 12,
            Self::Emerging => 13,
            Self::Digging => 14,
            Self::Sliding => 15,
            Self::Shooting => 16,
            Self::Inhaling => 17,
        }
    }

    pub const fn get_bounding_box(
        self,
        standing_bounding_box: EntityBoundingBox,
    ) -> Option<EntityBoundingBox> {
        match self {
            Self::Standing => Some(standing_bounding_box),
            Self::FallFlying | Self::Swimming | Self::SpinAttack => {
                Some(EntityBoundingBox::new(0.6, 0.6, 0.6))
            }
            Self::Sleeping | Self::Dying => Some(EntityBoundingBox::new(0.2, 0.2, 0.2)),
            Self::Sneaking => Some(EntityBoundingBox::new(0.6, 1.5, 0.6)),
            _ => None,
        }
    }

    pub const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Standing),
            1 => Some(Self::FallFlying),
            2 => Some(Self::Sleeping),
            3 => Some(Self::Swimming),
            4 => Some(Self::SpinAttack),
            5 => Some(Self::Sneaking),
            6 => Some(Self::LongJumping),
            7 => Some(Self::Dying),
            8 => Some(Self::Croaking),
            9 => Some(Self::UsingTongue),
            10 => Some(Self::Sitting),
            11 => Some(Self::Roaring),
            12 => Some(Self::Sniffing),
            13 => Some(Self::Emerging),
            14 => Some(Self::Digging),
            15 => Some(Self::Sliding),
            16 => Some(Self::Shooting),
            17 => Some(Self::Inhaling),
            _ => None,
        }
    }
}
