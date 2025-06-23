use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lighting);
    }
}

fn setup_lighting(mut commands: Commands) {
    // Ambient light for 3D sphere rendering
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        affects_lightmapped_meshes: false,
    });
    
    // Directional light for nice sphere shading
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: false,
            affects_lightmapped_mesh_diffuse: false,
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 0.6,
        },
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // Point light for additional illumination
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 2000.0,
            range: 100.0,
            radius: 0.0,
            shadows_enabled: false,
            affects_lightmapped_mesh_diffuse: false,
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 0.6,
            shadow_map_near_z: 0.1,
        },
        Transform::from_xyz(5.0, 5.0, 5.0),
    ));
}