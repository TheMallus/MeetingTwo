use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

// const DAMAGE: f32 = 0.5;
// fn update_health (
//     mut query: Query<(Entity, &mut Health)>,
//     mut commands: Commands
// )
// {
//     for (entity, mut health) in query.iter_mut() {
//         health.value += -DAMAGE;
//         if health.value <= 0.0 {
//             commands.entity(entity).despawn_recursive();
//             continue
//         }
//     }
// }
// damage component is based on collision, possibly use rapier for this or else use raycast or the demo
// collision system that isn't run with a bevy engine and hard coded but actually works