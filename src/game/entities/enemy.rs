use bevy::prelude::*;
use crate::game::components::{Physics, Size};

#[derive(Component)]
pub struct Enemy {
    pub target: Option<Entity>,
    pub aggression: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            target: None,
            aggression: 0.5,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..5 {
        let angle = i as f32 * std::f32::consts::TAU / 5.0;
        let x = angle.cos() * 8.0;
        let z = angle.sin() * 8.0;
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.8))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
            Transform::from_xyz(x, 0.0, z),
            Enemy::default(),
            Physics::default(),
            Size::new(0.8, 8.0),
        ));
    }
}