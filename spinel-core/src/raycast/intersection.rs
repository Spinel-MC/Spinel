use spinel_network::types::Vector3d;

use crate::raycast::bounding_box::RaycastBoundingBox;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaycastNormal {
    NegativeX,
    PositiveX,
    NegativeY,
    PositiveY,
    NegativeZ,
    PositiveZ,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaycastIntersection {
    pub ratio: f64,
    pub position: Vector3d,
    pub normal: RaycastNormal,
}

impl RaycastIntersection {
    pub fn between_ray_and_box(
        start: Vector3d,
        direction: Vector3d,
        bounding_box: RaycastBoundingBox,
    ) -> Option<Self> {
        let x_axis = RaycastAxisIntersection::new(
            start.x,
            direction.x,
            bounding_box.minimum.x,
            bounding_box.maximum.x,
            RaycastNormal::NegativeX,
            RaycastNormal::PositiveX,
        )?;
        let y_axis = RaycastAxisIntersection::new(
            start.y,
            direction.y,
            bounding_box.minimum.y,
            bounding_box.maximum.y,
            RaycastNormal::NegativeY,
            RaycastNormal::PositiveY,
        )?;
        let z_axis = RaycastAxisIntersection::new(
            start.z,
            direction.z,
            bounding_box.minimum.z,
            bounding_box.maximum.z,
            RaycastNormal::NegativeZ,
            RaycastNormal::PositiveZ,
        )?;

        let entrance = x_axis.entrance.max(y_axis.entrance).max(z_axis.entrance);
        let exit = x_axis.exit.min(y_axis.exit).min(z_axis.exit);

        if entrance > exit || !(0.0..=1.0).contains(&entrance) {
            return None;
        }

        Some(Self {
            ratio: entrance,
            position: Vector3d {
                x: start.x + direction.x * entrance,
                y: start.y + direction.y * entrance,
                z: start.z + direction.z * entrance,
            },
            normal: intersection_normal(entrance, x_axis, y_axis, z_axis),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct RaycastAxisIntersection {
    entrance: f64,
    exit: f64,
    entrance_normal: RaycastNormal,
}

impl RaycastAxisIntersection {
    fn new(
        start: f64,
        direction: f64,
        minimum: f64,
        maximum: f64,
        negative_normal: RaycastNormal,
        positive_normal: RaycastNormal,
    ) -> Option<Self> {
        if direction == 0.0 {
            return (minimum..=maximum).contains(&start).then_some(Self {
                entrance: f64::NEG_INFINITY,
                exit: f64::INFINITY,
                entrance_normal: negative_normal,
            });
        }

        let minimum_ratio = (minimum - start) / direction;
        let maximum_ratio = (maximum - start) / direction;
        let ray_moves_positive = direction > 0.0;

        Some(Self {
            entrance: minimum_ratio.min(maximum_ratio),
            exit: minimum_ratio.max(maximum_ratio),
            entrance_normal: if ray_moves_positive {
                negative_normal
            } else {
                positive_normal
            },
        })
    }
}

fn intersection_normal(
    entrance: f64,
    x_axis: RaycastAxisIntersection,
    y_axis: RaycastAxisIntersection,
    z_axis: RaycastAxisIntersection,
) -> RaycastNormal {
    if entrance == x_axis.entrance {
        return x_axis.entrance_normal;
    }
    if entrance == y_axis.entrance {
        return y_axis.entrance_normal;
    }
    z_axis.entrance_normal
}

#[cfg(test)]
mod tests {
    use spinel_network::types::Vector3d;

    use crate::raycast::{RaycastBoundingBox, RaycastNormal};

    #[test]
    fn raycast_hits_nearest_box_face() {
        let bounding_box = RaycastBoundingBox::new(
            Vector3d {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3d {
                x: 3.0,
                y: 1.0,
                z: 1.0,
            },
        );

        let intersection = bounding_box
            .ray_intersection(
                Vector3d {
                    x: 0.0,
                    y: 0.5,
                    z: 0.5,
                },
                Vector3d {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
            )
            .unwrap();

        assert_eq!(intersection.ratio, 0.5);
        assert_eq!(intersection.normal, RaycastNormal::NegativeX);
        assert_eq!(
            intersection.position,
            Vector3d {
                x: 2.0,
                y: 0.5,
                z: 0.5
            }
        );
    }

    #[test]
    fn raycast_rejects_box_beyond_ray_segment() {
        let bounding_box = RaycastBoundingBox::new(
            Vector3d {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3d {
                x: 3.0,
                y: 1.0,
                z: 1.0,
            },
        );

        assert!(
            bounding_box
                .ray_intersection(
                    Vector3d {
                        x: 0.0,
                        y: 0.5,
                        z: 0.5,
                    },
                    Vector3d {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                )
                .is_none()
        );
    }
}
