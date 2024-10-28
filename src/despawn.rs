use bevy::prelude::*;
use crate::{movement::Collider, entities::Dummy};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn_far_away_entities, handle_collisions));
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > 25.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_collisions(mut commands: Commands, query: Query<(Entity, &Collider), With<Dummy>>,) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}