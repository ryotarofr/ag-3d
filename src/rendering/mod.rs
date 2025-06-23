use bevy::prelude::*;

pub mod materials;
pub mod camera;
pub mod lighting;

use materials::MaterialsPlugin;
use camera::CameraPlugin;
use lighting::LightingPlugin;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialsPlugin,
            CameraPlugin,
            LightingPlugin,
        ));
    }
}