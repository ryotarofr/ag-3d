use crate::game::components::Size;
use crate::rendering::shapes::SphereSprite;
use bevy::prelude::*;

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

fn spawn_initial_food(mut commands: Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        // Generate random 3D positions in a cube around the origin
        let x = rng.gen_range(-50.0..50.0);
        let y = rng.gen_range(-50.0..50.0);
        let z = rng.gen_range(-25.0..25.0);

        // Random color variations for food
        let green_variation = rng.gen_range(0.5..1.0);
        let red_tint = rng.gen_range(0.0..0.5);
        let color = Color::srgb(red_tint, green_variation, 0.0);

        commands.spawn((
            SphereSprite::new(5.0, color),
            Transform::from_xyz(x, y, z),
            Food::default(),
            Size::new(0.2, 1.0),
        ));
    }
}
