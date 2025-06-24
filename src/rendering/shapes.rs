use bevy::prelude::*;

#[derive(Component)]
pub struct SphereSprite {
    pub radius: f32,
    pub color: Color,
}

impl SphereSprite {
    pub fn new(radius: f32, color: Color) -> Self {
        Self { radius, color }
    }
}

pub struct SphereSpritePlugin;

impl Plugin for SphereSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sphere_resources)
            .add_systems(PostUpdate, update_sphere_sprites);
    }
}

fn setup_sphere_resources(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Create a sphere mesh resource
    let sphere_mesh = Sphere::new(1.0).mesh().uv(32, 18);
    let sphere_mesh_handle = meshes.add(sphere_mesh);
    commands.insert_resource(SphereMeshResource {
        mesh: sphere_mesh_handle,
    });
}

#[derive(Resource)]
pub struct SphereMeshResource {
    pub mesh: Handle<Mesh>,
}

fn update_sphere_sprites(
    mut commands: Commands,
    sphere_mesh: Res<SphereMeshResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &SphereSprite, &Transform), (Without<Mesh3d>, Added<SphereSprite>)>,
) {
    for (entity, sphere_sprite, transform) in query.iter() {
        // Create 3D material with proper lighting for sphere effect
        let material = materials.add(StandardMaterial {
            base_color: sphere_sprite.color,
            metallic: 0.1,
            perceptual_roughness: 0.8,
            ..default()
        });

        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert((
                Mesh3d(sphere_mesh.mesh.clone()),
                MeshMaterial3d(material),
                Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: Vec3::splat(sphere_sprite.radius * 0.01), // Scale the sphere properly
                },
            ));
        }
    }
}
