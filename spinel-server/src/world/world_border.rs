use spinel_core::network::clientbound::play::initialize_world_border::InitializeWorldBorderPacket;
use std::io::{Error, ErrorKind, Result};

pub const WORLD_BORDER_SIZE: i32 = 29_999_984;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldBorder {
    diameter: f64,
    center_x: f64,
    center_z: f64,
    warning_distance: i32,
    warning_time: i32,
    dimension_teleport_boundary: i32,
}

impl WorldBorder {
    pub const DEFAULT: Self = Self {
        diameter: (WORLD_BORDER_SIZE as f64) * 2.0,
        center_x: 0.0,
        center_z: 0.0,
        warning_distance: 5,
        warning_time: 15,
        dimension_teleport_boundary: WORLD_BORDER_SIZE,
    };

    pub fn new(
        diameter: f64,
        center_x: f64,
        center_z: f64,
        warning_distance: i32,
        warning_time: i32,
        dimension_teleport_boundary: i32,
    ) -> Result<Self> {
        if diameter < 0.0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Diameter should be >= 0",
            ));
        }
        Ok(Self {
            diameter,
            center_x,
            center_z,
            warning_distance,
            warning_time,
            dimension_teleport_boundary,
        })
    }

    pub const fn diameter(self) -> f64 {
        self.diameter
    }

    pub const fn center_x(self) -> f64 {
        self.center_x
    }

    pub const fn center_z(self) -> f64 {
        self.center_z
    }

    pub const fn warning_distance(self) -> i32 {
        self.warning_distance
    }

    pub const fn warning_time(self) -> i32 {
        self.warning_time
    }

    pub const fn dimension_teleport_boundary(self) -> i32 {
        self.dimension_teleport_boundary
    }

    pub fn with_diameter(self, diameter: f64) -> Result<Self> {
        Self::new(
            diameter,
            self.center_x,
            self.center_z,
            self.warning_distance,
            self.warning_time,
            self.dimension_teleport_boundary,
        )
    }

    pub fn with_center(self, center_x: f64, center_z: f64) -> Self {
        Self {
            center_x,
            center_z,
            ..self
        }
    }

    pub const fn with_warning_distance(self, warning_distance: i32) -> Self {
        Self {
            warning_distance,
            ..self
        }
    }

    pub const fn with_warning_time(self, warning_time: i32) -> Self {
        Self {
            warning_time,
            ..self
        }
    }

    pub fn contains(self, x: f64, z: f64) -> bool {
        let radius = self.diameter / 2.0;
        x <= self.center_x + radius
            && x >= self.center_x - radius
            && z <= self.center_z + radius
            && z >= self.center_z - radius
    }

    pub fn initialize_packet(
        self,
        target_diameter: f64,
        transition_time: i64,
    ) -> InitializeWorldBorderPacket {
        InitializeWorldBorderPacket::new(
            self.center_x,
            self.center_z,
            self.diameter,
            target_diameter,
            transition_time,
            self.dimension_teleport_boundary,
            self.warning_distance,
            self.warning_time,
        )
    }
}
