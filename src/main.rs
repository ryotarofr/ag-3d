use bevy::prelude::*;

mod game;
mod network;
mod rendering;
mod utils;

use game::GamePlugin;
use rendering::RenderingPlugin;
use utils::UtilsPlugin;

fn main() {
    println!("Starting Agario 2D...");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(GamePlugin)
        .add_plugins(RenderingPlugin)
        .add_plugins(UtilsPlugin)
        .run();
}
