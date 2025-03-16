use crate::bundles::UnitBundle;
use crate::tools::assembler::assemble;
use crate::assets::AssetLibrary;
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
    repo: Res<UnitRepository>,
    asset_lib: Res<AssetLibrary>,
) {
    for request in spawn_events.read() {
        let program_code = repo.get_latest_code_for_unit(request.unit_id).unwrap();
        let program = assemble(program_code).unwrap();
        let asset = &asset_lib.assets["purple"];

        let sprite = Sprite::from_atlas_image(
            asset.image.clone(),
            TextureAtlas {
                layout: asset.layout.clone(),
                index: asset.mappings["fulldots"]
            }
        );

        commands.spawn(UnitBundle::new(
            request.unit_id,
            request.position,
            sprite,
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
