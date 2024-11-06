use bevy::prelude::*;
use crate::{health::Health, states::GameState};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn_far_away_entities, despawn_dead_entities))
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
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

fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health)>
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With <Health>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}