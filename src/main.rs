use bevy::prelude::*;

mod game;
mod rendering;
mod network;
mod utils;

use game::GamePlugin;
use rendering::RenderingPlugin;
use utils::UtilsPlugin;

fn main() {
    println!("Starting Agario 2D...");
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(RenderingPlugin)
        .add_plugins(UtilsPlugin)
        .run();
}