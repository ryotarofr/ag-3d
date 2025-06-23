use bevy::prelude::*;
use crate::game::components::Size;

#[derive(Component)]
pub struct Food {
    pub nutrition: f32,
    pub respawn_timer: f32,
}

impl Default for Food {
    fn default() -> Self {
        Self {
            nutrition: 1.0,
            respawn_timer: 0.0,
        }
    }
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_food);
    }
}

fn spawn_initial_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..50 {
        let x = (i as f32 * 3.0) % 20.0 - 10.0;
        let z = ((i / 10) as f32 * 3.0) - 7.5;
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
            Transform::from_xyz(x, 0.0, z),
            Food::default(),
            Size::new(0.2, 1.0),
        ));
    }
}