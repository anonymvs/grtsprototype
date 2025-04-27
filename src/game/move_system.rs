// use bevy::math::bounding::BoundingCircle;
use bevy::{color, prelude::*};

use crate::game::types::{CursorPosition, Playable, UnitState};
use crate::ui::settings::GameSettings;

// fn vec2_equal_eps(a: Vec2, b: Vec2, epsilon: f32) -> bool {
//     (a - b).length_squared() < epsilon * epsilon
// }

fn move_toward_direction(position: Vec2, direction: Vec2, speed: f32) -> Vec2 {
    return position + (direction.normalize() * speed);
}

// fn move_toward_target(position: Vec2, target: Vec2, speed: f32, epsilon: f32) -> Vec2 {
//     if vec2_equal_eps(position, target, epsilon) {
//         return position;
//     }

//     let dir = position - target;
//     return move_toward_direction(position, dir, speed);
// }

pub fn move_rect_system(
    mut query: Query<(&UnitState, &mut Transform), With<Playable>>,
    settings: Res<GameSettings>,
    mut gizmos: Gizmos,
) {
    for (unit_state, mut transform) in &mut query {
        transform.translation = move_toward_direction(
            transform.translation.truncate(),
            unit_state.direction,
            unit_state.velocity,
        )
        .extend(0.0);

        if settings.is_gizmo_enabled {
            gizmos.line_2d(
                transform.translation.truncate(),
                transform.translation.truncate()
                    + (unit_state.direction * unit_state.velocity * 100.0),
                color::palettes::basic::BLUE,
            );
        }

        // Rotate the shape over time
        //let rotation_speed = 1.0; // radians per second
        //transform.rotate(Quat::from_rotation_z(rotation_speed * timer.delta_secs()));

        //println!("{} ", transform.translation.x);
    }
}

pub fn update_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    cursor_position.set(point);
}

pub fn rect_selector_pressed(keys: Res<ButtonInput<KeyCode>>, settings: Res<GameSettings>) -> bool {
    return keys.pressed(settings.rect_selector);
}

pub fn is_in_range(pos: Vec2, target: Vec2, range: f32) -> bool {
    return (pos - target).length_squared() < range * range;
}

pub fn unit_state_update_system(
    mut query: Query<(&mut UnitState, &Transform), With<Playable>>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    cursor_position: Res<CursorPosition>,
) {
    for (mut state, transform) in query.iter_mut() {
        let position = transform.translation.truncate();
        state.velocity = settings.speed * time.delta().as_secs_f32();
        state.direction = (cursor_position.0 - position).normalize_or_zero();
    }
}

fn is_intersecting(a: Vec2, radius_a: f32, b: Vec2, radius_b: f32) -> bool {
    let distance_squared = (a - b).length_squared();
    let radius_sum = radius_a + radius_b;
    return distance_squared < radius_sum * radius_sum;
}

pub fn intersect_system(
    //mut gizmos: Gizmos,
    //time: Res<Time>,
    mut query: Query<(Entity, &mut UnitState, &Transform), With<Playable>>,
    //ettings: Res<GameSettings>,
) {
    //const MAX_DIAMETER_SQUARED: f32 = 200.0;

    let mut unit_states_to_change: Vec<(Entity, Vec2)> = Vec::new();

    for (entity_curr, state_curr, transform_curr) in &query {
        for (entity_target, state_target, transform_target) in &query {
            if entity_curr == entity_target {
                continue;
            }

            if is_intersecting(
                transform_curr.translation.truncate(),
                state_curr.bounding_circle_radius,
                transform_target.translation.truncate(),
                state_target.bounding_circle_radius,
            ) {
                let dir_to_target = (transform_target.translation - transform_curr.translation)
                    .truncate()
                    .normalize_or_zero();
                unit_states_to_change.push((entity_curr, dir_to_target));
            }
        }
    }

    for (entity, dir_to_target) in unit_states_to_change {
        if let Ok((_, mut state, _)) = query.get_mut(entity) {
            state.direction = (state.direction + ((-dir_to_target) * 1.5)).normalize();
            state.velocity = state.velocity;
        }
    }

    // for (e_outer, state_outer, t_outer) in query.iter() {
    //     let radius = 50.0;
    //     let center = t_outer.translation.truncate();
    //     //let range = BoundingCircle::new(center, radius);

    //     for (e_inner, state_inner, t_inner) in query.iter() {
    //         if e_inner == e_outer {
    //             continue;
    //         }

    //         let is_in_targeting_range = is_in_range(center, t_inner.translation.truncate(), radius);
    //         let is_intersecting =
    //             is_in_range(center, t_inner.translation.truncate(), MAX_DIAMETER_SQUARED);

    //         if settings.is_gizmo_enabled {
    //             if is_in_targeting_range {
    //                 gizmos.circle_2d(center, radius, color::palettes::basic::FUCHSIA);
    //             } else {
    //                 gizmos.circle_2d(center, radius, color::palettes::basic::BLUE);
    //             }

    //             if is_intersecting {
    //                 gizmos.line_2d(
    //                     t_outer.translation.truncate(),
    //                     t_inner.translation.truncate(),
    //                     color::palettes::basic::RED,
    //                 );
    //             } else {
    //                 gizmos.line_2d(
    //                     t_outer.translation.truncate(),
    //                     t_inner.translation.truncate(),
    //                     color::palettes::basic::GREEN,
    //                 );
    //             }
    //         }
    //     }
}

// let aabb = match u_type {
//     UnitType::Rect => Aabb2d::new(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0)),
//     UnitType::Triangle => {}
//     UnitType::Circle => {}
// }

// let shape = match u_type {
//     UnitType::Rect => GizmoShape::Rect(Rectangle::new(10.0, 10.0)),
//     UnitType::Triangle => GizmoShape::Triangle(Triangle2d::new(
//         Vec2::new(0.0, 5.0),
//         Vec2::new(5.0, -5.0),
//         Vec2::new(-5.0, -5.0),
//     )),
//     UnitType::Circle => GizmoShape::Circle(Circle::new(5.0)),
// };

// gizmos.add_gizmo(shape, transform.translation.truncate());
