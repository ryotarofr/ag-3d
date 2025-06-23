use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameConfig {
    pub world_size: f32,
    pub max_food_count: usize,
    pub food_spawn_rate: f32,
    pub player_base_speed: f32,
    pub player_base_mass: f32,
    pub enemy_count: usize,
    pub camera_follow_speed: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            world_size: 30.0,
            max_food_count: 100,
            food_spawn_rate: 2.0,
            player_base_speed: 5.0,
            player_base_mass: 10.0,
            enemy_count: 5,
            camera_follow_speed: 0.02,
        }
    }
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameConfig>();
    }
}