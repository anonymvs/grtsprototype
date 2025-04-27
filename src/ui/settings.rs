use bevy::prelude::*;

use crate::game::unit_type::UnitType;

#[derive(Resource)]
pub struct GameSettings {
    pub eps: f32,
    pub speed: f32,
    pub rect_selector: KeyCode,
    pub is_gizmo_enabled: bool,
    pub unit_type_selected: UnitType,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            eps: 0.5,
            speed: 100.0,
            rect_selector: KeyCode::KeyD,
            is_gizmo_enabled: false,
            unit_type_selected: UnitType::Rect,
        }
    }
}
