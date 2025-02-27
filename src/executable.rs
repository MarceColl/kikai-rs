use bevy::prelude::*;

use crate::tools::assembler::Program;
use crate::components::Executable;

#[derive(Event, Debug)]
pub struct CodeReloadEvent {
    pub program: Program,
    pub unit_id: u64,
}

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

fn code_reload_event_handler(
    mut reader: EventReader<CodeReloadEvent>,
    mut query: Query<&mut Executable>
) {
    for ev in reader.read() {
        println!("EVENT: {:?}", ev);
        for mut executable in &mut query {
            println!("{} == {}?", executable.unit_id, ev.unit_id);
            if executable.unit_id == ev.unit_id {
                println!("YES!");
                executable.load_program(&ev.program);
            }
        }
    }
}

pub struct ExecutablePlugin;

impl Plugin for ExecutablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CodeReloadEvent>()
            .add_systems(
                Update,
                (update_executables, code_reload_event_handler).chain()
            );
    }
}
