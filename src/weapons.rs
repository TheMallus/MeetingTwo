use bevy::prelude::*;
use crate::{collision_detector::CollisionDamage, entities::{Block, Bullet}, health::Health, movement::{Acceleration, Collider, MovingObjBundle, Velocity}, schedule::InGameSet};
pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, block_weapons.in_set(InGameSet::UserInput));
    }
}


fn block_weapons(
    mut commands: Commands,
    query: Query<&mut Transform, With<Block>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) 
{
    let Ok(transform) = query.get_single() else {
        return;
    };
    if keyboard_input.pressed(KeyCode::KeyF) {
        commands.spawn((MovingObjBundle {
            velocity: Velocity::new(-transform.forward() * 25.0),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(0.1),
            health: Health::new(1.0),
            collision: CollisionDamage::new(5.0),
            model: PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1,0.1,0.1)),
                material: materials.add(Color::BLACK),
                transform: Transform::from_translation(
                    transform.translation + -transform.forward() * 1.1,
                ),
                ..default()
            },
        },
        Bullet,
        ));
    }

}
