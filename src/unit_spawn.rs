use bevy::prelude::*;
use crate::bundles::UnitBundle;

#[derive(Event)]
pub struct SpawnUnitRequest {
    pub unit_id: u64,
    pub position: Vec2,
}

pub struct UnitSpawnPlugin;

fn unit_spawner(
    mut spawn_events: EventReader<SpawnUnitRequest>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for request in spawn_events.read() {
        commands.spawn(UnitBundle::new(request.unit_id, request.position, &mut meshes, &mut materials));
    }
}

impl Plugin for UnitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnUnitRequest>()
            .add_systems(PostUpdate, unit_spawner);
    }
}
