use bevy::prelude::*;
use crate::egui::{
    style::{WidgetVisuals, Widgets},
    Color32, Label, Sense, RichText, vec2, WidgetText,
    FontDefinitions, FontData, FontFamily, Pos2, Stroke,
    ComboBox, Window,
};
use bevy_egui::{egui, EguiContexts};

use crate::bundles::UnitBundle;
use crate::unit_spawn::SpawnUnitRequest;
use crate::unit_repo::{UnitRepository, UnitDefinition};

enum SandboxUIMode {
    MainMenu,
    CreateUnit {
        unit_name: String,
    },
}

#[derive(Resource)]
struct SandboxState {
    mode: SandboxUIMode,
    selected_unit: Option<UnitDefinition>,
    units: Vec<UnitDefinition>,
    editor_open: bool,
    current_code: String,
    is_modified: bool,
}

impl SandboxState {
    pub fn refresh_units(&mut self, repo: &UnitRepository) {
        self.units = repo.get_units();
    }
}

impl Default for SandboxState {
    fn default() -> Self {
        SandboxState {
            mode: SandboxUIMode::MainMenu,
            units: Vec::new(),
            selected_unit: None,
            editor_open: false,
            current_code: "".to_string(),
            is_modified: false,
        }
    }
}

fn draw_sandbox_ui(
    mut context: EguiContexts,
    mut spawn_events: EventWriter<SpawnUnitRequest>,
    mut sandbox_state: ResMut<SandboxState>,
    repo: Res<UnitRepository>,
) {
    egui::Window::new("Sandbox".to_string()).show(context.ctx_mut(), |ui| {
        match &mut sandbox_state.mode {
            SandboxUIMode::MainMenu => {
                if ui.button("New Unit Type").clicked() {
                    sandbox_state.mode = SandboxUIMode::CreateUnit {
                        unit_name: String::new(),
                    };
                }

                let mut selected_unit = sandbox_state.selected_unit.clone();
                egui::ComboBox::from_label("Unit Type")
                    .selected_text(sandbox_state.selected_unit.as_ref().map_or_else(|| "None".to_string(), |s| s.name.clone()))
                    .show_ui(ui, |ui| {
                        for ud in &sandbox_state.units {
                            ui.selectable_value(&mut selected_unit, Some(ud.clone()), &ud.name);
                        }
                    }
                );

                if sandbox_state.selected_unit != selected_unit {
                    sandbox_state.current_code = selected_unit.as_ref().unwrap().code.clone().unwrap_or_else(|| String::new());
                    sandbox_state.selected_unit = selected_unit;
                }

                if ui.button("Create Unit").clicked() {
                    spawn_events.send(SpawnUnitRequest {
                        unit_id: 1,
                        position: Vec2::new(10., 10.),
                    });
                }

                if sandbox_state.editor_open {
                    if ui.button("Close Code Editor").clicked() { sandbox_state.editor_open = false; }
                } else {
                    if ui.button("Open Code Editor").clicked() { sandbox_state.editor_open = true; }
                }
            },
            SandboxUIMode::CreateUnit { unit_name: ref mut unit_name } => {
                let name_to_create = unit_name.clone();
                ui.add(egui::TextEdit::singleline(unit_name));

                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    if ui.button("Create").clicked() {
                        repo.new_unit_type(name_to_create);
                        sandbox_state.refresh_units(&repo);
                        sandbox_state.mode = SandboxUIMode::MainMenu;
                    }

                    if ui.button("Cancel").clicked() {
                        sandbox_state.mode = SandboxUIMode::MainMenu;
                    }
                });
            },
        };
    });
}

fn draw_editor_window(
    mut context: EguiContexts,
    mut spawn_events: EventWriter<SpawnUnitRequest>,
    mut sandbox_state: ResMut<SandboxState>,
    repo: Res<UnitRepository>,
) {
    if sandbox_state.editor_open {
        egui::Window::new("Editor".to_string()).show(context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Save").clicked() {
                    repo.update_code_for_unit(
                        sandbox_state.selected_unit.as_ref().unwrap().unit_id,
                        sandbox_state.current_code.clone(),
                    );
                }
                if ui.button("Compile").clicked() {}
                if ui.button("Save & Compile").clicked() {}
            });
            ui.add(
                egui::TextEdit::multiline(&mut sandbox_state.current_code)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(10)
            );
        });
    }
}

fn initialize_sandbox_state(
    mut sandbox_state: ResMut<SandboxState>,
    repo: Res<UnitRepository>,
) {
    sandbox_state.refresh_units(&repo);
}

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource::<SandboxState>(SandboxState::default())
            .add_systems(Startup, initialize_sandbox_state)
            .add_systems(Update, (
                draw_sandbox_ui,
                draw_editor_window,
            ));
    }
}
