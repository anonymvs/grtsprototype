use bevy::prelude::*;

pub struct UnitPlugin;

use crate::game::move_system::{
    intersect_system, move_unit_system, unit_selector_pressed, unit_state_update_system,
    update_cursor,
};
use crate::game::types::{CursorPosition, UnitFlags};

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(Vec2::new(0.0, 0.0)));
        app.insert_resource(UnitFlags::NONE);
        app.add_systems(
            Update,
            (
                update_cursor,
                unit_state_update_system,
                intersect_system,
                move_unit_system.run_if(unit_selector_pressed),
            )
                .chain(),
        );
    }
}
