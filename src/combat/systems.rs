use crate::{
    entities::{Block, Bullet, Dummy},
    movement::MovingObjBundle,
};

// use bevy::{prelude::*, utils::HashMap};
// use crate::{entities::{Block, Bullet, Dummy}, health::Health, movement::Collider, schedule::InGameSet};
// pub struct CollisionDetectionPlugin;
use super::*;
use crate::movement::*;
use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{Collider, Sensor},
};

pub fn apply_collision_damage(
    collision_damage_query: Query<(Entity, &CollisionDamage)>,
    mut health_query: Query<&mut Health, Without<Bullet>>,
    dummies_query: Query<&Dummy>,
    rapier_context: Res<RapierContext>,
    // name_query: Query<&Name>,
    // mut commands: Commands,
) {
    for (e, damage) in collision_damage_query.iter() {
        for (e1, e2, _) in rapier_context.intersection_pairs_with(e)
        //.filter(|(_, _, bool)| bool == &true)
        {
            // dummies should not damage other dummies
            if dummies_query.get(e1).is_ok() && dummies_query.get(e2).is_ok() {
                continue;
            }
            let e_target = e1;
            let Ok(mut health) = health_query.get_mut(e_target) else {
                return;
            };
            // uncomment to test what was hit.
            // let name = name_query
            //     .get(e_target)
            //     .map_or(format!("{:#?}", e_target), |name| name.to_string());
            // println!("hit {:#}", name);

            health.value -= damage.0;
        }
    }
}

pub fn block_weapons(
    mut commands: Commands,
    query: Query<&mut Transform, With<Block>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::KeyF) {
        commands.spawn((
            MovingObjBundle {
                velocity: Velocity::new(-transform.forward() * 25.0),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::cuboid(0.05, 0.05, 0.05),
                health: Health::new(1.0),
                collision: CollisionDamage(25.0),
                model: PbrBundle {
                    mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                    material: materials.add(Color::BLACK),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * 1.1,
                    ),
                    ..default()
                },
            },
            Bullet,
            Sensor,
            Name::new("bullet"),
        ));
    }
}
