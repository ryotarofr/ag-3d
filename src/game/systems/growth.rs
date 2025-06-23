use bevy::prelude::*;
use crate::game::components::Size;

pub struct GrowthPlugin;

impl Plugin for GrowthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_visual_size);
    }
}

fn update_visual_size(
    mut query: Query<(&mut Transform, &Size), Changed<Size>>,
) {
    for (mut transform, size) in query.iter_mut() {
        transform.scale = Vec3::splat(size.radius);
    }
}