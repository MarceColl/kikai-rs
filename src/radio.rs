use bevy::prelude::*;
use crate::components::Executable;

#[derive(Event, Copy, Clone, Debug)]
pub struct RadioMessage {
    pub origin_entity_id: Option<Entity>,
    pub packets: [u16; 2],
    pub frequency: u8,
}

fn route_radio_messages(
    mut query: Query<(Entity, &mut Executable, &mut Transform)>,
    mut in_radio_messages: EventReader<RadioMessage>,
) {
    for msg in in_radio_messages.read() {
        for (entity, mut executable, mut transform) in &mut query {
            let entity_freq = executable.radio_frequency();

            if msg.origin_entity_id.unwrap() != entity && entity_freq == msg.frequency {
                let message_vec = executable.radio_message_vector();
                executable.set_radio_packets(&msg.packets);
                executable.pc = Some(message_vec);

                if let Some(mut rm) = executable.cont(&mut transform) {
                    // rm.origin_entity_id = Some(entity);
                    // out_radio_messages.send(rm);
                }
            }
        }
    }
}

pub struct RadioPlugin;

impl Plugin for RadioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RadioMessage>()
            .add_systems(Update, route_radio_messages);
    }
}
