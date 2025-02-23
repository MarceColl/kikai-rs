use bevy::{
    prelude::*,
    input::{
        ButtonState,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel}
    },
    math::bounding::{BoundingVolume, Aabb2d},
    color::palettes::css::*,
};
use anyhow::{Context, Result};
use std::path::Path;
use zerocopy::{BigEndian, U16};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::BTreeMap;

mod devices;
mod components;
mod bundles;
mod tools;

use crate::bundles::UnitBundle;
use crate::components::{Selectable, Selected, Executable, Collider};
use crate::devices::{UnitIO, CommandPorts, MovementPorts};
use crate::tools::assembler::{Program, disassm};

const BACKGROUND_COLOR: Color = Color::srgb(0., 0., 0.);

fn update_executables(mut query: Query<(&mut Executable, &mut Transform)>) {
    for (mut executable, mut transform) in &mut query {
        let loop_vec = executable.loop_vector();
        executable.execute(&mut transform, loop_vec);
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

    if (context.ctx_mut().wants_pointer_input()) {
        return;
    }

    match window.cursor_position().and_then(|cursor| Some(camera.viewport_to_world_2d(global_transform, cursor)))
    {
        Some(Ok(world_position)) => {
            let wp_aabb = Aabb2d::new(world_position, Vec2::new(0., 0.));
            for event in mouse_events.read() {
                query.iter().for_each(|(eid, selectable, transform)| {
                    let aabb = Aabb2d::new(transform.translation.xy(), transform.scale.xy() / 2.);

                    if aabb.contains(&wp_aabb) {
                        commands.entity(eid).insert(Selected);
                    }
                })
            }
        },
        _ => {},
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

    match window.cursor_position().and_then(|cursor| Some(camera.viewport_to_world_2d(global_transform, cursor)))
    {
        Some(Ok(world_position)) => {
            let wp_aabb = Aabb2d::new(world_position, Vec2::new(0., 0.));
            query.iter().for_each(|(eid, transform)| {
                let aabb = Aabb2d::new(transform.translation.xy(), transform.scale.xy() / 2.);

                let color = if aabb.contains(&wp_aabb) { RED } else { YELLOW };
                gizmos.rect_2d(aabb.center().xy(), aabb.half_size().xy() * 2., color);
            })
        },
        _ => {},
    };
}

fn command_system(
    mut context: EguiContexts,
    mut query: Query<(Entity, &mut Executable, &mut Transform), With<Selected>>,
    mut mouse_events: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    if (context.ctx_mut().wants_pointer_input()) {
        return;
    }

    for event in mouse_events.read() {
        query.iter_mut().for_each(|(eid, mut executable, mut transform)| {
            match (event.button, event.state) {
                (MouseButton::Right, ButtonState::Released) => {
                    let move_vec = executable.move_vector();
                    println!("MOVE COMMAND! {:?} ({})", eid, move_vec);
                    executable.set_move_command_coords(5, 5);
                    executable.execute(&mut transform, move_vec);
                },
                (MouseButton::Left, ButtonState::Released) => {
                    commands.entity(eid).remove::<Selected>();
                },
                _ => {}
            }
        })
    }
}

fn executable_debugging(
    mut context: EguiContexts,
    mut executables: Query<(Entity, &mut Executable, &mut Transform), With<Selected>>,
) {
   executables.iter_mut().for_each(|(eid, mut executable, mut transform)| {
       let mut device = executable.device.arm(&mut transform);

       egui::Window::new(format!("Unit Inspector")).show(context.ctx_mut(), |ui| {
           egui::CollapsingHeader::new("Devices").show(ui, |ui| {
               egui::CollapsingHeader::new("Command").show(ui, |ui| {
                   let mut cmd = executable.cpu.dev::<CommandPorts>();
                   ui.label(format!("Move Vector: {}", cmd.move_vector.get()));
                   ui.label(format!("Attack Vector: {}", cmd.attack_vector.get()));
                   ui.label(format!("Create Vector: {}", cmd.create_vector.get()));
                   ui.label(format!("x: {}", cmd.x.get()));
                   ui.label(format!("y: {}", cmd.y.get()));
                   ui.label(format!("Loop Vector: {}", cmd.loop_vector.get()));
               });

               egui::CollapsingHeader::new("Movement").show(ui, |ui| {
                   let mut cmd = executable.cpu.dev::<MovementPorts>();
                   ui.label(format!("Vector: {}", cmd.vector.get()));
                   ui.label(format!("x: {}", cmd.x.get()));
                   ui.label(format!("y: {}", cmd.y.get()));
                   ui.label(format!("Dir: {}", cmd.dir));
               });
           });

           egui::CollapsingHeader::new("Disassembly").show(ui, |ui| {
               ui.label(egui::RichText::new(disassm(&executable.program)));
           })
       });
   })
}


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_initial_unit);
        app.add_systems(
            Update,
            (executable_debugging, update_executables, command_system, gizmos, selection_system).chain()
        );
    }
}

#[derive(Component)]
struct Unit {}

fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();

    Ok(())
}
