use bevy::prelude::*;
use crate::game::components::{Physics, Size};
use crate::rendering::shapes::SphereSprite;

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
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    for _ in 0..5 {
        // Generate random 3D positions
        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);
        let z = rng.gen_range(-50.0..50.0);
        
        // Add color variation to enemies
        let red_intensity = rng.gen_range(0.8..1.0);
        let green_tint = rng.gen_range(0.0..0.3);
        let blue_tint = rng.gen_range(0.0..0.2);
        let color = Color::srgb(red_intensity, green_tint, blue_tint);
        
        commands.spawn((
            SphereSprite::new(15.0, color),
            Transform::from_xyz(x, y, z),
            Enemy::default(),
            Physics::default(),
            Size::new(0.8, 8.0),
        ));
    }
}