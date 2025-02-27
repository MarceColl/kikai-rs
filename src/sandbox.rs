use rand::Rng;
use crate::egui::{
    style::{WidgetVisuals, Widgets},
    text::{Fonts, LayoutJob, TextFormat},
    vec2, Color32, ComboBox, FontData, FontDefinitions, FontFamily, FontId, Label, Pos2, RichText,
    Sense, Stroke, WidgetText, Window,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use regex::Regex;
use std::sync::Arc;

use crate::bundles::UnitBundle;
use crate::components::Executable;
use crate::executable::CodeReloadEvent;
use crate::tools::assembler::assemble;
use crate::unit_repo::{UnitDefinition, UnitRepository};
use crate::unit_spawn::SpawnUnitRequest;

enum SandboxUIMode {
    MainMenu,
    CreateUnit { unit_name: String },
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
                    .selected_text(
                        sandbox_state
                            .selected_unit
                            .as_ref()
                            .map_or_else(|| "None".to_string(), |s| s.name.clone()),
                    )
                    .show_ui(ui, |ui| {
                        for ud in &sandbox_state.units {
                            ui.selectable_value(&mut selected_unit, Some(ud.clone()), &ud.name);
                        }
                    });

                if sandbox_state.selected_unit != selected_unit {
                    sandbox_state.current_code = selected_unit
                        .as_ref()
                        .unwrap()
                        .code
                        .clone()
                        .unwrap_or_else(|| String::new());
                    sandbox_state.selected_unit = selected_unit;
                }

                if ui.button("Create Unit").clicked() {
                    let mut rng = rand::thread_rng();

                    let rx: i8 = rng.gen();
                    let ry: i8 = rng.gen();
                    spawn_events.send(SpawnUnitRequest {
                        unit_id: 1,
                        position: Vec2::new(rx as f32, ry as f32),
                    });
                }

                if sandbox_state.editor_open {
                    if ui.button("Close Code Editor").clicked() {
                        sandbox_state.editor_open = false;
                    }
                } else {
                    if ui.button("Open Code Editor").clicked() {
                        sandbox_state.editor_open = true;
                    }
                }
            }
            SandboxUIMode::CreateUnit {
                unit_name: ref mut unit_name,
            } => {
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
            }
        };
    });
}

// fn editor_layouter(ui: &egui::Ui, string: &str, wrap_width: u32) -> Arc<egui::Galley> {
// }

fn tokenize(input: &str) -> Vec<&str> {
    let re = Regex::new(r"(\s+|\S+)").unwrap();
    re.find_iter(input).map(|m| m.as_str()).collect()
}

fn color_for_tok(tok: &str) -> Color32 {
    if tok.starts_with('#') {
        Color32::from_rgb(0x1F, 0xC7, 0x42)
    } else if tok.starts_with('|') {
        Color32::from_rgb(0xEB, 0x9d, 0x16)
    } else if tok.starts_with('@') {
        Color32::from_rgb(0x55, 0xFF, 0xFF)
    } else if tok.starts_with('&') {
        Color32::from_rgb(0x00, 0xAA, 0xAA)
    } else if tok == "BRK" {
        Color32::from_rgb(0x99, 0x00, 0x00)
    } else {
        Color32::from_rgb(0x8E, 0xA3, 0xA6)
    }
}

fn draw_editor_window(
    mut context: EguiContexts,
    mut code_reload_events: EventWriter<CodeReloadEvent>,
    mut sandbox_state: ResMut<SandboxState>,
    repo: Res<UnitRepository>,
) {
    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
        let tokens = tokenize(string);
        let mut job = LayoutJob::default();

        for tok in tokens {
            job.append(
                tok,
                0.0,
                TextFormat {
                    color: color_for_tok(tok),
                    ..Default::default()
                },
            );
        }

        ui.fonts(|f| f.layout_job(job))
    };

    if sandbox_state.editor_open {
        egui::Window::new("Editor".to_string()).show(context.ctx_mut(), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Save").clicked() {
                    repo.update_code_for_unit(
                        sandbox_state.selected_unit.as_ref().unwrap().unit_id,
                        sandbox_state.current_code.clone(),
                    );
                }
                if ui.button("Assemble").clicked() {
                    let program = assemble(sandbox_state.current_code.clone());
                    code_reload_events.send(CodeReloadEvent {
                        program: program.clone().unwrap(),
                        unit_id: sandbox_state.selected_unit.as_ref().unwrap().unit_id,
                    });
                }
                if ui.button("Assemble & Save").clicked() {}
            });
            let te = egui::TextEdit::multiline(&mut sandbox_state.current_code)
                .code_editor()
                .layouter(&mut layouter)
                .desired_width(f32::INFINITY)
                .desired_rows(10)
                .show(ui);

            // let hover_pos = ui.input(|i| {
            //     i.pointer.hover_pos()
            // });
            // if let Some(hover_pos) = hover_pos {
            //     if te.response.rect.contains(hover_pos) {
            //         let hover_pos = hover_pos - te.response.rect.left_top();
            //         let hover_cursor = te.galley.cursor_from_pos(hover_pos).pcursor;
            //         if let Some(line) = sandbox_state.current_code.lines().nth(hover_cursor.paragraph) {
            //             egui::show_tooltip_at_pointer(ui.ctx(), ui.layer_id(), egui::Id::new("hover tooltip"), |ui| {
            //                 ui.label(line);
            //             });
            //         }
            //     }
            // }
        });
    }
}

fn initialize_sandbox_state(mut sandbox_state: ResMut<SandboxState>, repo: Res<UnitRepository>) {
    sandbox_state.refresh_units(&repo);
}

pub struct SandboxPlugin;

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SandboxState>(SandboxState::default())
            .add_systems(Startup, initialize_sandbox_state)
            .add_systems(Update, (draw_sandbox_ui, draw_editor_window));
    }
}
