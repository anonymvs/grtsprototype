use bevy::prelude::*;
use std::fmt::Debug;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub enum UnitType {
    Rect,
    Triangle,
    Circle,
}

#[derive(Component)]
pub struct UnitConstants {
    pub unit_type: UnitType,
    pub bounding_circle_radius: f32,
    pub easing_distance: f32,
}

// impl Debug for UnitType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             UnitType::Rect => write!(f, "Rect"),
//             UnitType::Triangle => write!(f, "Triangle"),
//             UnitType::Circle => write!(f, "Circle"),
//         }
//     }
// }

// impl PartialEq for UnitType {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (UnitType::Rect, UnitType::Rect) => true,
//             (UnitType::Triangle, UnitType::Triangle) => true,
//             (UnitType::Circle, UnitType::Circle) => true,
//             _ => false,
//         }
//     }
// }
