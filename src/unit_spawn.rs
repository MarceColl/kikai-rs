use crate::bundles::UnitBundle;
use crate::tools::assembler::assemble;
use crate::unit_repo::UnitRepository;
use bevy::prelude::*;

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
    repo: Res<UnitRepository>,
) {
    for request in spawn_events.read() {
        let program_code = repo.get_latest_code_for_unit(request.unit_id).unwrap();
        let program = assemble(program_code).unwrap();
        commands.spawn(UnitBundle::new(
            request.unit_id,
            request.position,
            &mut meshes,
            &mut materials,
            &program,
        ));
    }
}

impl Plugin for UnitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnUnitRequest>()
            .add_systems(PostUpdate, unit_spawner);
    }
}
