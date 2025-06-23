use bevy::prelude::*;

#[derive(Component)]
pub struct Size {
    pub radius: f32,
    pub mass: f32,
}

impl Size {
    pub fn new(radius: f32, mass: f32) -> Self {
        Self { radius, mass }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            radius: 1.0,
            mass: 10.0,
        }
    }
}

pub struct SizePlugin;

impl Plugin for SizePlugin {
    fn build(&self, _app: &mut App) {
    }
}