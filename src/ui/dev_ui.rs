use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};

use crate::game::unit_type::UnitType;
use crate::ui::settings::GameSettings;

#[derive(Resource)]
pub struct UiState {
    pub eps_text: String,
    pub easing_distance_str: String,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            eps_text: GameSettings::default().eps.to_string(),
            easing_distance_str: GameSettings::default().easing_distance.to_string(),
        }
    }
}

fn dev_ui_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<GameSettings>,
    mut ui_state: ResMut<UiState>,
) {
    if let Some(ctx) = contexts.try_ctx_mut() {
        egui::Window::new("Dev ui").show(ctx, |ui| {
            ui.label("world");

            // speed widget
            ui.add(egui::Slider::new(&mut settings.speed, 0.0..=1000.0).text("value"));
            ui.horizontal(|ui| {
                ui.label("eps: ");
                let response =
                    ui.add(egui::TextEdit::singleline(&mut ui_state.eps_text).hint_text("0.001"));
                if response.lost_focus() {
                    if let Ok(eps) = ui_state.eps_text.parse::<f32>() {
                        settings.eps = eps;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("easing distance: ");
                let response = ui.add(
                    egui::TextEdit::singleline(&mut ui_state.easing_distance_str).hint_text("20.0"),
                );
                if response.lost_focus() {
                    if let Ok(easing_distance) = ui_state.easing_distance_str.parse::<f32>() {
                        settings.easing_distance = easing_distance;
                    }
                }
            });

            // gizmo enabled checkbox
            ui.checkbox(&mut settings.is_gizmo_enabled, "enable gizmos");

            // spawn selector
            egui::ComboBox::from_label("Unit type")
                .selected_text(format!("{:?}", settings.unit_type_selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.unit_type_selected, UnitType::Rect, "Rect");
                    ui.selectable_value(
                        &mut settings.unit_type_selected,
                        UnitType::Triangle,
                        "Tria",
                    );
                    ui.selectable_value(&mut settings.unit_type_selected, UnitType::Circle, "Circ");
                });
        });
    }

    // egui::SidePanel::left("side_panel")
    //     .default_width(200.0)
    //     .show(contexts.ctx_mut(), |ui| {
    //         ui.label("Hello, side panel!");
    //     });
}

pub struct DevUIPlugin;

impl Plugin for DevUIPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup, setup);
        app.add_plugins(EguiPlugin);
        app.insert_resource(GameSettings::default());
        app.insert_resource(UiState::default());
        app.add_systems(Update, dev_ui_system);
    }
}
