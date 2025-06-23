use bevy::prelude::*;

pub fn distance_squared(pos1: Vec3, pos2: Vec3) -> f32 {
    (pos1 - pos2).length_squared()
}

pub fn distance(pos1: Vec3, pos2: Vec3) -> f32 {
    (pos1 - pos2).length()
}

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn clamp_f32(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

pub fn mass_to_radius(mass: f32, base_mass: f32) -> f32 {
    (mass / base_mass).sqrt()
}

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&self, _app: &mut App) {
    }
}