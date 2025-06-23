use bevy::prelude::*;
use rand::Rng;
use crate::game::entities::food::Food;
use crate::game::components::Size;
use crate::rendering::shapes::SphereSprite;

#[derive(Resource)]
pub struct SpawnTimer {
    timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>()
           .add_systems(Update, spawn_food);
    }
}

fn spawn_food(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    food_query: Query<&Food>,
) {
    spawn_timer.timer.tick(time.delta());
    
    if spawn_timer.timer.just_finished() && food_query.iter().count() < 100 {
        let mut rng = rand::thread_rng();
        
        // Generate random 3D positions similar to food.rs ranges
        let x = rng.gen_range(-50.0..50.0);
        let y = rng.gen_range(-50.0..50.0);
        let z = rng.gen_range(-25.0..25.0);
        
        // Add color variation for visual interest
        let green_variation = rng.gen_range(0.7..1.0);
        let red_tint = rng.gen_range(0.0..0.3);
        let blue_tint = rng.gen_range(0.0..0.2);
        let color = Color::srgb(red_tint, green_variation, blue_tint);
        
        commands.spawn((
            SphereSprite::new(10.0, color),
            Transform::from_xyz(x, y, z),
            Food::default(),
            Size::new(0.2, 1.0),
        ));
    }
}