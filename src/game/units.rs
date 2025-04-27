use bevy::{color, prelude::*};

use crate::game::types::{Playable, PlayerUnitBundle, UnitState};
use crate::game::unit_type::UnitType;

fn rectangle() -> Rectangle {
    return Rectangle::new(10.0, 10.0);
}

fn triangle() -> Triangle2d {
    return Triangle2d::new(
        Vec2::new(0.0, 5.0),
        Vec2::new(5.0, -5.0),
        Vec2::new(-5.0, -5.0),
    );
}

fn circle() -> Circle {
    return Circle::new(5.0);
}

pub fn spawn_unit(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    unit_type: UnitType,
    position: Vec2,
) -> PlayerUnitBundle {
    let shape = match unit_type {
        UnitType::Rect => meshes.add(rectangle()),
        UnitType::Triangle => meshes.add(triangle()),
        UnitType::Circle => meshes.add(circle()),
    };

    let color_basic = color::palettes::basic::RED;
    let color = Color::linear_rgb(color_basic.red, color_basic.green, color_basic.blue);
    let handle = materials.add(color);

    let transform = Transform::from_xyz(position.x, position.y, 0.0);

    return PlayerUnitBundle {
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(handle),
        transform,
        playable: Playable,
        state: UnitState {
            velocity: 0.0,
            direction: Vec2::new(0.0, 0.0),
            bounding_circle_radius: 5.0,
        },
        unit_type,
    };
}
