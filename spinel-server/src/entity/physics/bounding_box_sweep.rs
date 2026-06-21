use crate::entity::physics::EntityBoundingBoxSweepResult;
use spinel_network::types::Vector3d;
use spinel_registry::EntityBoundingBox;

enum SweepAxis {
    X,
    Y,
    Z,
}

pub(crate) fn intersect_bounding_boxes_swept(
    static_box: EntityBoundingBox,
    ray_start: Vector3d,
    ray_direction: Vector3d,
    shape_position: Vector3d,
    moving_box: EntityBoundingBox,
    final_result: &mut EntityBoundingBoxSweepResult,
) -> bool {
    let moving_center = Vector3d {
        x: moving_box.minimum_x() + moving_box.get_width() / 2.0,
        y: moving_box.minimum_y() + moving_box.get_height() / 2.0,
        z: moving_box.minimum_z() + moving_box.depth() / 2.0,
    };
    let ray_center = add(ray_start, moving_center);
    let expanded_minimum = Vector3d {
        x: static_box.minimum_x() + shape_position.x - moving_box.get_width() / 2.0,
        y: static_box.minimum_y() + shape_position.y - moving_box.get_height() / 2.0,
        z: static_box.minimum_z() + shape_position.z - moving_box.depth() / 2.0,
    };
    let expanded_maximum = Vector3d {
        x: static_box.maximum_x() + shape_position.x + moving_box.get_width() / 2.0,
        y: static_box.maximum_y() + shape_position.y + moving_box.get_height() / 2.0,
        z: static_box.maximum_z() + shape_position.z + moving_box.depth() / 2.0,
    };

    let mut earliest_ratio = f64::MAX;
    let mut collision_axis = None;
    update_earliest_collision(
        SweepAxis::X,
        ray_center,
        ray_direction,
        expanded_minimum,
        expanded_maximum,
        &mut earliest_ratio,
        &mut collision_axis,
    );
    update_earliest_collision(
        SweepAxis::Z,
        ray_center,
        ray_direction,
        expanded_minimum,
        expanded_maximum,
        &mut earliest_ratio,
        &mut collision_axis,
    );
    update_earliest_collision(
        SweepAxis::Y,
        ray_center,
        ray_direction,
        expanded_minimum,
        expanded_maximum,
        &mut earliest_ratio,
        &mut collision_axis,
    );

    let Some(collision_axis) = collision_axis else {
        return false;
    };
    let collision_ratio = earliest_ratio * 0.99999;
    if collision_ratio < 0.0 || collision_ratio > final_result.get_ratio() {
        return false;
    }

    let normal = match collision_axis {
        SweepAxis::X => Vector3d {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        SweepAxis::Y => Vector3d {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        SweepAxis::Z => Vector3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let collision_point = Vector3d {
        x: ray_start.x + ray_direction.x * collision_ratio,
        y: ray_start.y + ray_direction.y * collision_ratio,
        z: ray_start.z + ray_direction.z * collision_ratio,
    };
    final_result.set_collision(
        collision_ratio,
        normal,
        collision_point,
        static_box,
        shape_position,
    );
    true
}

fn update_earliest_collision(
    axis: SweepAxis,
    ray_center: Vector3d,
    ray_direction: Vector3d,
    expanded_minimum: Vector3d,
    expanded_maximum: Vector3d,
    earliest_ratio: &mut f64,
    collision_axis: &mut Option<SweepAxis>,
) {
    let axis_direction = coordinate(ray_direction, &axis);
    if axis_direction == 0.0 {
        return;
    }

    let collision_plane = if axis_direction > 0.0 {
        coordinate(expanded_minimum, &axis)
    } else {
        coordinate(expanded_maximum, &axis)
    };
    let ratio = epsilon((collision_plane - coordinate(ray_center, &axis)) / axis_direction);
    if ratio >= *earliest_ratio {
        return;
    }

    let intersection = Vector3d {
        x: ray_center.x + ray_direction.x * ratio,
        y: ray_center.y + ray_direction.y * ratio,
        z: ray_center.z + ray_direction.z * ratio,
    };
    if !intersection_is_within_expanded_face(
        intersection,
        expanded_minimum,
        expanded_maximum,
        &axis,
    ) {
        return;
    }

    *earliest_ratio = ratio;
    *collision_axis = Some(axis);
}

fn intersection_is_within_expanded_face(
    intersection: Vector3d,
    expanded_minimum: Vector3d,
    expanded_maximum: Vector3d,
    excluded_axis: &SweepAxis,
) -> bool {
    match excluded_axis {
        SweepAxis::X => {
            intersection.y >= expanded_minimum.y
                && intersection.y <= expanded_maximum.y
                && intersection.z >= expanded_minimum.z
                && intersection.z <= expanded_maximum.z
        }
        SweepAxis::Y => {
            intersection.x >= expanded_minimum.x
                && intersection.x <= expanded_maximum.x
                && intersection.z >= expanded_minimum.z
                && intersection.z <= expanded_maximum.z
        }
        SweepAxis::Z => {
            intersection.x >= expanded_minimum.x
                && intersection.x <= expanded_maximum.x
                && intersection.y >= expanded_minimum.y
                && intersection.y <= expanded_maximum.y
        }
    }
}

fn coordinate(vector: Vector3d, axis: &SweepAxis) -> f64 {
    match axis {
        SweepAxis::X => vector.x,
        SweepAxis::Y => vector.y,
        SweepAxis::Z => vector.z,
    }
}

fn epsilon(value: f64) -> f64 {
    if value.abs() < 0.000001 {
        return 0.0;
    }
    value
}

fn add(first: Vector3d, second: Vector3d) -> Vector3d {
    Vector3d {
        x: first.x + second.x,
        y: first.y + second.y,
        z: first.z + second.z,
    }
}
