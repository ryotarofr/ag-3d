use bevy::prelude::*;
use crate::game::components::{Physics, Size};

#[derive(Component)]
pub struct Player {
    pub mass: f32,
    pub speed: f32,
    pub max_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            mass: 10.0,
            speed: 5.0,
            max_speed: 10.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player::default(),
        Physics::default(),
        Size::new(1.0, 10.0),
    ));
}