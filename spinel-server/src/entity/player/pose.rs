use spinel_registry::EntityBoundingBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerPose {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    Dying,
}

impl PlayerPose {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Standing => 0,
            Self::FallFlying => 1,
            Self::Sleeping => 2,
            Self::Swimming => 3,
            Self::SpinAttack => 4,
            Self::Sneaking => 5,
            Self::Dying => 6,
        }
    }

    pub const fn bounding_box(
        self,
        standing_bounding_box: EntityBoundingBox,
    ) -> Option<EntityBoundingBox> {
        match self {
            Self::Standing => Some(standing_bounding_box),
            Self::FallFlying | Self::Swimming | Self::SpinAttack => {
                Some(EntityBoundingBox::new(0.6, 0.6, 0.6))
            }
            Self::Sleeping => Some(EntityBoundingBox::new(0.2, 0.2, 0.2)),
            Self::Sneaking => Some(EntityBoundingBox::new(0.6, 1.5, 0.6)),
            Self::Dying => None,
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
            6 => Some(Self::Dying),
            _ => None,
        }
    }
}
