use crate::components::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Unit {}

const UNIT_SIZE: Vec3 = Vec3::new(50., 50., 1.);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    executable: Executable,
    transform: Transform,
    selectable: Selectable,
    collider: Collider,
}

impl UnitBundle {
    pub fn new(
        unit_type_id: u64,
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        UnitBundle {
            unit: Unit {},
            mesh: Mesh2d(meshes.add(Circle::default())),
            material: MeshMaterial2d(materials.add(BALL_COLOR)),
            executable: Executable::from_file(unit_type_id, "unit.rom"),
            transform: Transform {
                translation: pos.extend(0.),
                scale: UNIT_SIZE,
                ..default()
            },
            selectable: Selectable::new(),
            collider: Collider::new(pos, 25.),
        }
    }
}
