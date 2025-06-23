use bevy::prelude::*;

pub mod materials;
pub mod camera;
pub mod lighting;
pub mod shapes;

use materials::MaterialsPlugin;
use camera::CameraPlugin;
use lighting::LightingPlugin;
use shapes::SphereSpritePlugin;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialsPlugin,
            CameraPlugin,
            LightingPlugin, // Empty plugin for 2D - maintains structure
            SphereSpritePlugin,
        ));
    }
}