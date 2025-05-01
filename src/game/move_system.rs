// use bevy::math::bounding::BoundingCircle;
use bevy::{color, prelude::*};

use crate::game::types::{CursorPosition, Playable, UnitState};
use crate::game::unit_type::UnitType;
use crate::ui::settings::GameSettings;

use super::unit_type::UnitConstants;

// fn vec2_equal_eps(a: Vec2, b: Vec2, epsilon: f32) -> bool {
//     (a - b).length_squared() < epsilon * epsilon
// }

fn is_equel_eps(a: f32, b: f32, epsilon: f32) -> bool {
    return (a - b).abs() < epsilon;
}

fn is_zero_eps(a: f32, epsilon: f32) -> bool {
    return a.abs() < epsilon;
}

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

pub fn move_unit_system(
    mut query: Query<(&UnitState, &UnitConstants, &mut Transform), With<Playable>>,
    mut gizmos: Gizmos,
    settings: Res<GameSettings>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (unit_state, constants, mut transform) in &mut query {
        let mut should_move = false;

        if keys.pressed(settings.rect_selector) && constants.unit_type == UnitType::Rect {
            should_move = true;
        } else if keys.pressed(settings.circ_selector) && constants.unit_type == UnitType::Circle {
            should_move = true;
        } else if keys.pressed(settings.tria_selector) && constants.unit_type == UnitType::Triangle
        {
            should_move = true;
        }

        if should_move {
            transform.translation = move_toward_direction(
                transform.translation.truncate(),
                unit_state.direction,
                unit_state.velocity,
            )
            .extend(0.0);
        }

        if settings.is_gizmo_enabled {
            gizmos.line_2d(
                transform.translation.truncate(),
                transform.translation.truncate()
                    + (unit_state.direction * unit_state.velocity * 10.0),
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

pub fn unit_selector_pressed(keys: Res<ButtonInput<KeyCode>>, settings: Res<GameSettings>) -> bool {
    return keys.pressed(settings.rect_selector)
        || keys.pressed(settings.circ_selector)
        || keys.pressed(settings.tria_selector);
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

pub fn calc_easing_vecotr_system(
    mut query: Query<(Entity, &mut UnitState, &UnitType, &Transform), With<Playable>>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    cursor_position: Res<CursorPosition>,
) {
    // todo use: https://docs.rs/force_graph/latest/force_graph/

    for (entity, mut state, unit_type, transform) in query.iter_mut() {
        // for current unit

        let mut easing_components: Vec<Vec2> = Vec::new();
        for (entity_target, state_target, unit_type_target, transform_target) in &query {
            if entity == entity_target {
                continue;
            }

            if unit_type == unit_type_target {
                let mut dir_target =
                    transform_target.translation.truncate() - transform.translation.truncate();
                let dist = dir_target.length() - settings.easing_distance;
                if dist < 0.0 {
                    dir_target = -dir_target;
                }

                if is_zero_eps(dist, settings.eps) {
                    continue;
                } else {
                    // 0.5 because the target moves the same amount to the other direction
                    easing_components.push(dir_target.normalize_or_zero() * dist.abs() * 0.5);
                }
            }
        }
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
    mut query: Query<(Entity, &mut UnitState, &UnitConstants, &Transform), With<Playable>>,
    //ettings: Res<GameSettings>,
) {
    //const MAX_DIAMETER_SQUARED: f32 = 200.0;

    let mut unit_states_to_change: Vec<(Entity, Vec2)> = Vec::new();

    for (entity_curr, state_curr, constants_curr, transform_curr) in &query {
        for (entity_target, state_target, constants_target, transform_target) in &query {
            if entity_curr == entity_target {
                continue;
            }

            if is_intersecting(
                transform_curr.translation.truncate(),
                constants_curr.bounding_circle_radius,
                transform_target.translation.truncate(),
                constants_target.bounding_circle_radius,
            ) {
                let dir_to_target = (transform_target.translation - transform_curr.translation)
                    .truncate()
                    .normalize_or_zero();
                unit_states_to_change.push((entity_curr, dir_to_target));
            }
        }
    }

    for (entity, dir_to_target) in unit_states_to_change {
        if let Ok((_, mut state, _, _)) = query.get_mut(entity) {
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
