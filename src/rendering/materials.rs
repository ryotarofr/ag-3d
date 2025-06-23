use bevy::prelude::*;

#[derive(Resource)]
pub struct GameMaterials {
    pub player_material: Handle<StandardMaterial>,
    pub food_material: Handle<StandardMaterial>,
    pub enemy_material: Handle<StandardMaterial>,
}

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_materials);
    }
}

fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let game_materials = GameMaterials {
        player_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            metallic: 0.1,
            perceptual_roughness: 0.3,
            ..default()
        }),
        food_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        }),
        enemy_material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            metallic: 0.2,
            perceptual_roughness: 0.4,
            ..default()
        }),
    };
    
    commands.insert_resource(game_materials);
}