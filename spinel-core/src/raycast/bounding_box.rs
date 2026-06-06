use spinel_network::types::Vector3d;

use crate::raycast::intersection::RaycastIntersection;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastBoundingBox {
    pub minimum: Vector3d,
    pub maximum: Vector3d,
}

impl RaycastBoundingBox {
    pub const fn new(minimum: Vector3d, maximum: Vector3d) -> Self {
        Self { minimum, maximum }
    }

    pub fn from_center_dimensions(center: Vector3d, width: f64, height: f64, depth: f64) -> Self {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;
        Self {
            minimum: Vector3d {
                x: center.x - half_width,
                y: center.y,
                z: center.z - half_depth,
            },
            maximum: Vector3d {
                x: center.x + half_width,
                y: center.y + height,
                z: center.z + half_depth,
            },
        }
    }

    pub fn ray_intersection(
        self,
        start: Vector3d,
        direction: Vector3d,
    ) -> Option<RaycastIntersection> {
        RaycastIntersection::between_ray_and_box(start, direction, self)
    }
}
