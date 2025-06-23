use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::game::entities::player::Player;
use crate::game::components::Size;

#[derive(Component)]
pub struct GameCamera {
    pub sensitivity: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.002,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
           .add_systems(Update, fps_camera_control);
    }
}

fn setup_camera(mut commands: Commands, mut windows: Query<&mut Window>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera::default(),
    ));
    
    // Lock cursor to center of window for FPS-style control
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    }
}

fn fps_camera_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let movement_speed = 50.0;
        let rotation_speed = 2.0;
        let dt = time.delta_secs();
        
        // W key for forward movement in the direction the camera is facing
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = camera_transform.rotation * Vec3::NEG_Z;
            camera_transform.translation += forward * movement_speed * dt;
        }
        
        // Arrow keys for camera rotation
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            // Pitch up (rotate around X-axis)
            camera_transform.rotation *= Quat::from_rotation_x(rotation_speed * dt);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            // Pitch down (rotate around X-axis)
            camera_transform.rotation *= Quat::from_rotation_x(-rotation_speed * dt);
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            // Yaw left (rotate around Y-axis)
            camera_transform.rotation *= Quat::from_rotation_y(rotation_speed * dt);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            // Yaw right (rotate around Y-axis)
            camera_transform.rotation *= Quat::from_rotation_y(-rotation_speed * dt);
        }
    }
}

