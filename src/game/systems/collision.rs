use bevy::prelude::*;
use crate::game::entities::{player::Player, food::Food, enemy::Enemy};
use crate::game::components::Size;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_player_food_collision,
            check_sphere_collisions,
        ));
    }
}

fn check_player_food_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Size), (With<Player>, Without<Food>)>,
    food_query: Query<(Entity, &Transform, &Size), (With<Food>, Without<Player>)>,
) {
    for (_player_entity, player_transform, mut player_size) in player_query.iter_mut() {
        for (food_entity, food_transform, food_size) in food_query.iter() {
            let distance = player_transform.translation.distance(food_transform.translation);
            let collision_distance = player_size.radius + food_size.radius;
            
            if distance < collision_distance {
                player_size.mass += food_size.mass;
                player_size.radius = (player_size.mass / 10.0).sqrt();
                
                commands.entity(food_entity).despawn();
            }
        }
    }
}

fn check_sphere_collisions(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &mut Size), Or<(With<Player>, With<Enemy>)>>,
) {
    let mut combinations = query.iter_combinations_mut();
    
    while let Some([(entity_a, transform_a, mut size_a), (entity_b, transform_b, mut size_b)]) = combinations.fetch_next() {
        let distance = transform_a.translation.distance(transform_b.translation);
        let collision_distance = size_a.radius + size_b.radius;
        
        if distance < collision_distance {
            if size_a.mass > size_b.mass * 1.1 {
                size_a.mass += size_b.mass;
                size_a.radius = (size_a.mass / 10.0).sqrt();
                commands.entity(entity_b).despawn();
            } else if size_b.mass > size_a.mass * 1.1 {
                size_b.mass += size_a.mass;
                size_b.radius = (size_b.mass / 10.0).sqrt();
                commands.entity(entity_a).despawn();
            }
        }
    }
}