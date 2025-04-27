use bevy::{
    math::{Isometry2d, bounding::*, ops},
    prelude::*,
};

use crate::game::units;
use crate::ui::settings::GameSettings;

pub fn input_handling(
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    //return keys.pressed(settings.rect_selector);

    if keys.just_pressed(KeyCode::Space) {
        commands.spawn(units::spawn_unit(
            meshes,
            materials,
            settings.unit_type_selected,
            Vec2::new(0.0, 0.0),
        ));
    }
    // if keys.just_released(KeyCode::ControlLeft) {
    //     // Left Ctrl was released
    // }
    // if keys.pressed(KeyCode::KeyW) {
    //     // W is being held down
    // }
    // // we can check multiple at once with `.any_*`
    // if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
    //     // Either the left or right shift are being held down
    // }
    // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
    //     // Either delete or backspace was just pressed
    // }
}

pub struct InputHandlingPlugin;

impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_handling);
    }
}
