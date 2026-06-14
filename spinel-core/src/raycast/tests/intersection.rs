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

#[test]
fn raycast_corner_ties_use_minestom_x_z_y_axis_priority() {
    let bounding_box = RaycastBoundingBox::new(
        Vector3d {
            x: 0.0,
            y: 2.0,
            z: 2.0,
        },
        Vector3d {
            x: 1.0,
            y: 3.0,
            z: 3.0,
        },
    );

    let intersection = bounding_box
        .ray_intersection(
            Vector3d {
                x: 0.5,
                y: 0.0,
                z: 0.0,
            },
            Vector3d {
                x: 0.0,
                y: 4.0,
                z: 4.0,
            },
        )
        .unwrap();

    assert_eq!(intersection.ratio, 0.5);
    assert_eq!(intersection.normal, RaycastNormal::NegativeZ);
}
