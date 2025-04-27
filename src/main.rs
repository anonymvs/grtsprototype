use bevy::prelude::*;

mod game;
mod ui;

use game::unit_plugin::UnitPlugin;
use ui::dev_ui::DevUIPlugin;
use ui::input_handling::InputHandlingPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // Uncomment this to override the default log settings:
            level: bevy::log::Level::WARN,
            filter: "wgpu=warn,bevy_ecs=info".to_string(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(InputHandlingPlugin)
        .add_plugins(DevUIPlugin)
        .add_plugins(UnitPlugin)
        .run();
}
