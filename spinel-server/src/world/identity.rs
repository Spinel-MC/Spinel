use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldIdentity {
    uuid: Uuid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldPointers {
    uuid: Uuid,
}

impl WorldIdentity {
    pub const fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }

    pub const fn uuid(self) -> Uuid {
        self.uuid
    }
}

impl WorldPointers {
    pub const fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }

    pub const fn uuid(self) -> Uuid {
        self.uuid
    }

    pub const fn identity(self) -> WorldIdentity {
        WorldIdentity::new(self.uuid)
    }
}
