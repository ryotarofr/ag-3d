use bevy::prelude::*;

#[derive(Component)]
pub struct Physics {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, _app: &mut App) {
    }
}