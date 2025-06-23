use bevy::prelude::*;

pub mod entities;
pub mod systems;
pub mod components;

use entities::*;
use systems::*;
use components::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            FoodPlugin,
            EnemyPlugin,
            MovementPlugin,
            CollisionPlugin,
            GrowthPlugin,
            SpawningPlugin,
            PhysicsPlugin,
            SizePlugin,
            HealthPlugin,
        ));
    }
}