use bevy::prelude::*;

#[derive(Resource)]
pub struct GameColors {
    pub player_color: Color,
    pub food_color: Color,
    pub enemy_color: Color,
}

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_colors);
    }
}

fn setup_colors(
    mut commands: Commands,
) {
    let game_colors = GameColors {
        player_color: Color::srgb(0.0, 0.0, 1.0),
        food_color: Color::srgb(0.0, 1.0, 0.0),
        enemy_color: Color::srgb(1.0, 0.0, 0.0),
    };
    
    commands.insert_resource(game_colors);
}