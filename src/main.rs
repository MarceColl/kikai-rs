use bevy::{
    prelude::*,
    input::{
        ButtonState,
        mouse::MouseButtonInput
    },
    math::bounding::{BoundingVolume, Aabb2d},
    color::palettes::css::*,
};
use egui::{
    style::{WidgetVisuals, Widgets},
    Color32, Label, Sense, RichText, vec2, WidgetText,
    FontDefinitions, FontData, FontFamily, Pos2, Stroke,
};
use anyhow::Result;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rusqlite::{params, Connection, OpenFlags};
use rusqlite_migration::{Migrations, M};

mod devices;
mod components;
mod bundles;
mod tools;
mod sandbox;
mod unit_spawn;
mod unit_repo;

use crate::bundles::UnitBundle;
use crate::components::{Selectable, Selected, Executable};
use crate::devices::{CommandPorts, MovementPorts};
use crate::tools::assembler::{disassm, DisassmAtom, Disassm};
use crate::sandbox::SandboxPlugin;
use crate::unit_spawn::UnitSpawnPlugin;
use crate::unit_repo::UnitRepoPlugin;

const BACKGROUND_COLOR: Color = Color::srgb(0., 0., 0.);

fn update_executables(mut query: Query<(&mut Executable, &mut Transform)>) {
    for (mut executable, mut transform) in &mut query {
        executable.cycles_left = 1000;
        if let None = executable.pc {
            let loop_vec = executable.loop_vector();
            executable.pc = Some(loop_vec);
            executable.cont(&mut transform);
        }
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_initial_unit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.spawn(UnitBundle::new(Vec2::new(100., 100.), &mut meshes, &mut materials));
    commands.spawn(UnitBundle::new(Vec2::new(300., 100.), &mut meshes, &mut materials));
}

fn selection_system(
    mut context: EguiContexts,
    query: Query<(Entity, &Selectable, &Transform)>,
    q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    let window = q_window.single();
    let (camera, global_transform) = q_camera.single();

    if context.ctx_mut().wants_pointer_input() {
        return;
    }

    if let Some(Ok(world_position)) = window.cursor_position().map(|cursor| camera.viewport_to_world_2d(global_transform, cursor)) {
        let wp_aabb = Aabb2d::new(world_position, Vec2::new(0., 0.));
        for event in mouse_events.read() {
            query.iter().for_each(|(eid, selectable, transform)| {
                let aabb = Aabb2d::new(transform.translation.xy(), transform.scale.xy() / 2.);

                if aabb.contains(&wp_aabb) {
                    commands.entity(eid).insert(Selected);
                }
            })
        }
    };
}

fn gizmos(
    mut gizmos: Gizmos,
    query: Query<(Entity, &Transform), With<Selected>>,
    q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = q_window.single();
    let (camera, global_transform) = q_camera.single();

    if let Some(Ok(world_position)) = window.cursor_position().map(|cursor| camera.viewport_to_world_2d(global_transform, cursor)) {
        let wp_aabb = Aabb2d::new(world_position, Vec2::new(0., 0.));
        query.iter().for_each(|(eid, transform)| {
            let aabb = Aabb2d::new(transform.translation.xy(), transform.scale.xy() / 2.);

            let color = if aabb.contains(&wp_aabb) { RED } else { YELLOW };
            gizmos.rect_2d(aabb.center().xy(), aabb.half_size().xy() * 2., color);
        })
    };
}

fn command_system(
    mut context: EguiContexts,
    mut query: Query<(Entity, &mut Executable, &mut Transform), With<Selected>>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    if context.ctx_mut().wants_pointer_input() {
        return;
    }

    for event in mouse_events.read() {
        query.iter_mut().for_each(|(eid, mut executable, mut transform)| {
            match (event.button, event.state) {
                (MouseButton::Right, ButtonState::Released) => {
                    let move_vec = executable.move_vector();
                    executable.set_move_command_coords(5, 5);
                    executable.pc = Some(move_vec);
                    executable.cont(&mut transform);
                },
                (MouseButton::Left, ButtonState::Released) => {
                    commands.entity(eid).remove::<Selected>();
                },
                _ => {}
            }
        })
    }
}

fn format_disassm_atom(atom: &DisassmAtom, is_current_instr: bool) -> WidgetText {
    let mut out = match atom {
        DisassmAtom::Instr(instr) => {
            RichText::new(format!("    {:?}", instr)).color(Color32::WHITE)
        },
        DisassmAtom::Lit(val) => {
            RichText::new(format!("    #{}", val)).color(Color32::CYAN)
        },
        DisassmAtom::Lit2(val) => {
            RichText::new(format!("    #{}", val)).color(Color32::CYAN)
        },
        DisassmAtom::Jsi(val) => {
            RichText::new(format!("    JSI {}", val)).color(Color32::ORANGE)
        },
        DisassmAtom::Jci(val) => {
            RichText::new(format!("    JCI {}", val)).color(Color32::ORANGE)
        },
        DisassmAtom::Jmi(val) => {
            RichText::new(format!("    JSI {}", val)).color(Color32::ORANGE)
        },
        DisassmAtom::AbsoluteLabel(label) => {
            RichText::new(format!("@{}", label)).color(Color32::GREEN)
        },
        _ => {
            RichText::new(format!("???"))
        }
    };

    if is_current_instr {
        out = out.color(Color32::RED);
    }

    out.monospace().into()
}

fn executable_debugging(
    mut context: EguiContexts,
    mut executables: Query<(Entity, &mut Executable, &mut Transform), With<Selected>>,
) {
    executables.iter_mut().for_each(|(eid, mut executable, mut transform)| {
        let device = executable.device.arm(&mut transform);

        egui::Window::new("Unit Inspector".to_string()).scroll(true).show(context.ctx_mut(), |ui| {
            egui::CollapsingHeader::new("Devices").show(ui, |ui| {
                egui::CollapsingHeader::new("Command").show(ui, |ui| {
                    let cmd = executable.cpu.dev::<CommandPorts>();
                    ui.label(format!("Move Vector: {}", cmd.move_vector.get()));
                    ui.label(format!("Attack Vector: {}", cmd.attack_vector.get()));
                   ui.label(format!("Create Vector: {}", cmd.create_vector.get()));
                   ui.label(format!("x: {}", cmd.x.get()));
                   ui.label(format!("y: {}", cmd.y.get()));
                   ui.label(format!("Loop Vector: {}", cmd.loop_vector.get()));
               });

               egui::CollapsingHeader::new("Movement").show(ui, |ui| {
                   let cmd = executable.cpu.dev::<MovementPorts>();
                   ui.label(format!("Vector: {}", cmd.vector.get()));
                   ui.label(format!("x: {}", cmd.x.get()));
                   ui.label(format!("y: {}", cmd.y.get()));
                   ui.label(format!("Dir: {}", cmd.dir));
               });
           });

           egui::CollapsingHeader::new("Disassembly").show(ui, |ui| {
               let disassm = disassm(&executable.program);
               let mut instr_docs: Option<String> = None;

               ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                   egui::ScrollArea::vertical().show(ui, |ui| {
                       ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                           let instr_height = Label::new("Your text").layout_in_ui(ui).1.size().y;

                           for span in disassm.spans {
                               ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                                   if executable.has_breakpoint_at(&span.addr) {
                                       let text = RichText::new(format!("  {:04X}   ", span.addr))
                                           .color(Color32::BLACK)
                                           .background_color(Color32::RED);
                                       if ui.add(Label::new(text).sense(Sense::click())).clicked() {
                                           executable.remove_breakpoint(&span.addr);
                                       }
                                   } else {
                                       if ui.add(Label::new(format!("  {:04X}   ", span.addr)).sense(Sense::click())).clicked() {
                                           executable.add_breakpoint(&span.addr);
                                       }
                                   }

                                   let mut is_current_instr = matches!(executable.pc, Some(pc) if pc == span.addr);

                                   if ui.add(Label::new(format_disassm_atom(&span.atom, is_current_instr)).sense(Sense::hover())).hovered() {
                                       instr_docs = Some("This is an explanation of things".to_string());
                                   }

                                   let pos = ui.next_widget_position();

                                   if let DisassmAtom::Jci(offset) = span.atom {
                                       ui.painter().line(
                                           vec![
                                               pos + vec2(0., instr_height / 2.),
                                               pos + vec2(80.0, instr_height / 2.),
                                               pos + vec2(80.0, (offset as f32) * instr_height + instr_height / 2.),
                                               pos + vec2(0., (offset as f32) * instr_height + instr_height / 2.),
                                           ],
                                           Stroke::new(3.0, Color32::from_rgb(255, 0, 0))
                                       );
                                   }
                               });
                           }
                       });
                   });

                   ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                       ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                           if ui.button("Start").clicked() {
                               executable.start();
                           }

                           if ui.add_enabled(executable.can_step(), egui::Button::new("Step")).clicked() {
                               executable.step(&mut transform);
                           }

                           if ui.button("Continue").clicked() {
                               executable.cont(&mut transform);
                           }
                       });

                       egui::ScrollArea::vertical().show(ui, |ui| {
                           for i in 0..executable.cpu.stack.len() {
                               let mut text = RichText::new(format!("{:02X}", executable.cpu.stack.peek_byte_at(i)))
                                   .size(20.)
                                   .monospace();

                               if i % 2 == 0 {
                                   text = text.background_color(Color32::GRAY).color(Color32::BLACK);
                               }

                               ui.label(text);
                           }
                       });

                       if let Some(docs) = instr_docs {
                           ui.label(docs);
                       }
                   });
               });
           });
       });
   })
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_initial_unit);
        app.add_systems(Startup, configure_visuals_system);
        app.add_systems(
            Update,
            (
                executable_debugging,
                update_executables,
                command_system,
                gizmos,
                selection_system,
            ).chain()
        );
    }
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_corner_radius: 0.0.into(),
        window_fill: Color32::BLACK,
        widgets: Widgets {
            ..Default::default()
        },
        ..Default::default()
    });

    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "IBM BIOS".to_owned(),
        std::sync::Arc::new(
            // .ttf and .otf supported
            FontData::from_static(include_bytes!("../assets/AcPlus_IBM_BIOS.ttf"))
        )
    );

    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
                                                     .insert(0, "IBM BIOS".to_owned());

    // Put my font as last fallback for monospace:
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
                                                  .insert(0, "IBM BIOS".to_owned());
    contexts.ctx_mut().set_fonts(fonts);
}

#[derive(Component)]
struct Unit {}

fn main() -> Result<()> {
    let mut conn = Connection::open_with_flags("./units.db", OpenFlags::default()).unwrap();

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(())).unwrap();

    App::new()
        .add_plugins(UnitRepoPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(UnitSpawnPlugin)
        .add_plugins(SandboxPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();

    Ok(())
}
