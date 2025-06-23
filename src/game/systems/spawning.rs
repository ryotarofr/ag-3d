use bevy::prelude::*;
use rand::Rng;
use crate::game::entities::food::Food;
use crate::game::components::Size;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    food_query: Query<&Food>,
) {
    spawn_timer.timer.tick(time.delta());
    
    if spawn_timer.timer.just_finished() && food_query.iter().count() < 100 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-15.0..15.0);
        let z = rng.gen_range(-15.0..15.0);
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
            Transform::from_xyz(x, 0.0, z),
            Food::default(),
            Size::new(0.2, 1.0),
        ));
    }
}