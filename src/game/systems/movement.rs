use bevy::prelude::*;
use crate::game::entities::player::Player;
use crate::game::components::{Physics, Size};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_input,
            apply_physics,
        ));
    }
}

fn player_input(
    mut player_query: Query<(&mut Physics, &Player), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    _time: Res<Time>,
) {
    for (mut physics, player) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
            physics.acceleration = direction * player.speed;
        } else {
            physics.acceleration = Vec3::ZERO;
        }
    }
}

fn apply_physics(
    mut query: Query<(&mut Transform, &mut Physics, &Size)>,
    time: Res<Time>,
) {
    for (mut transform, mut physics, size) in query.iter_mut() {
        let acceleration = physics.acceleration;
        physics.velocity += acceleration * time.delta_secs();
        
        let drag = 0.98;
        physics.velocity *= drag;
        
        let max_speed = 10.0 / (size.mass / 10.0).sqrt();
        if physics.velocity.length() > max_speed {
            physics.velocity = physics.velocity.normalize() * max_speed;
        }
        
        transform.translation += physics.velocity * time.delta_secs();
    }
}