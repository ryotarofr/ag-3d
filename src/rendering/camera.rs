use bevy::prelude::*;
use crate::game::entities::player::Player;
use crate::game::components::Size;

#[derive(Component)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
           .add_systems(Update, follow_player);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        (
            Camera3d::default(),
            Transform::from_xyz(0.0, 15.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ),
        GameCamera,
    ));
}

fn follow_player(
    player_query: Query<(&Transform, &Size), (With<Player>, Without<GameCamera>)>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    if let Ok((player_transform, player_size)) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let target_distance = 15.0 + player_size.radius * 2.0;
            let target_position = player_transform.translation + Vec3::new(0.0, target_distance, target_distance * 0.7);
            
            camera_transform.translation = camera_transform.translation.lerp(target_position, 0.02);
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}