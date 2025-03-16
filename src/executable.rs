use bevy::prelude::*;

use crate::components::Executable;
use crate::radio::RadioMessage;
use crate::tools::assembler::Program;

#[derive(Event, Debug)]
pub struct CodeReloadEvent {
    pub program: Program,
    pub unit_id: u64,
}

fn update_executables(
    mut query: Query<(Entity, &mut Executable, &mut Transform)>,
    mut radio_messages: EventWriter<RadioMessage>,
) {
    for (entity, mut executable, mut transform) in &mut query {
        executable.cycles_left = 1000;

        let pos = transform.translation;
        let target_pos = executable.target_pos();
        let dir = (target_pos - pos).normalize();

        if transform.translation.distance(target_pos) < 10.0 {
            transform.translation = target_pos;
        } else {
            transform.translation += dir * 10.0;
        }

        executable.set_current_pos(transform.translation);

        if let None = executable.pc {
            let loop_vec = executable.loop_vector();
            executable.pc = Some(loop_vec);
            if let Some(mut rm) = executable.cont(&mut transform) {
                rm.origin_entity_id = Some(entity);
                radio_messages.send(rm);
            }
        }
    }
}

fn code_reload_event_handler(
    mut reader: EventReader<CodeReloadEvent>,
    mut query: Query<(&mut Executable, &mut Transform)>,
) {
    for ev in reader.read() {
        for (mut executable, mut transform) in &mut query {
            if executable.unit_id == ev.unit_id {
                executable.load_program(&ev.program, &mut transform);
            }
        }
    }
}

pub struct ExecutablePlugin;

impl Plugin for ExecutablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CodeReloadEvent>().add_systems(
            Update,
            (update_executables, code_reload_event_handler).chain(),
        );
    }
}
