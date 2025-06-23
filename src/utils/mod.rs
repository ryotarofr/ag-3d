use bevy::prelude::*;

pub mod math;
pub mod config;

use math::MathPlugin;
use config::ConfigPlugin;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MathPlugin,
            ConfigPlugin,
        ));
    }
}