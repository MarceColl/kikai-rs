use crate::components::*;
use crate::tools::assembler::Program;
use bevy::prelude::*;

#[derive(Component)]
pub struct Unit {}

const UNIT_SIZE: Vec3 = Vec3::new(5., 5., 1.);

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    sprite: Sprite,
    executable: Executable,
    transform: Transform,
    selectable: Selectable,
    collider: Collider,
}

impl UnitBundle {
    pub fn new(
        unit_type_id: u64,
        pos: Vec2,
        sprite: Sprite,
        program: &Program,
    ) -> Self {
        UnitBundle {
            unit: Unit {},
            sprite,
            executable: Executable::from_program(unit_type_id, program),
            transform: Transform {
                translation: pos.extend(0.),
                scale: UNIT_SIZE,
                ..default()
            },
            selectable: Selectable::new(),
            collider: Collider::new(pos, 50.),
        }
    }
}
