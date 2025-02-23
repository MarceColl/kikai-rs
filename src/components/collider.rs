use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;

#[derive(Component)]
pub struct Collider {
    pub bounding_box: Aabb2d,
}

impl Collider {
    pub fn new(position: Vec2, width: f32) -> Self {
        Collider {
            bounding_box: Aabb2d::new(position, Vec2::new(width, width)),
        }
    }
}
