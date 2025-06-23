use bevy::prelude::*;
use bevy::render::settings::{WgpuSettings, Backends};
use bevy::render::RenderPlugin;

mod game;
mod rendering;
mod network;
mod utils;

use game::GamePlugin;
use rendering::RenderingPlugin;
use utils::UtilsPlugin;

fn main() {
    println!("Starting Agario 3D...");
    
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::GL),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(RenderingPlugin)
        .add_plugins(UtilsPlugin)
        .run();
}