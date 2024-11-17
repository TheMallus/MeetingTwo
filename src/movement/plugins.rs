pub struct MovementPlugin;

use bevy_rapier3d::prelude::Collider;

use crate::{
    combat::{CollisionDamage, Health},
    schedule::InGameSet,
};

pub use super::*;

// Components are great for organizing various items for an entity(like the block)
// but we can organize them further by grouping them into bundles
// bundles are great for categorizing your variables into different parts
// a robot for example will have a wheels bundle, movement bundle, weapons bundle
// and a physics bundle (like weight, mass, etc.) which have their own variables

// Let's make our first bundle for our block, this will add our 2 components we made
// as well as the cube model for our block

#[derive(Bundle)]
pub struct MovingObjBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration, // this is a built in bevy bundle
    pub health: Health,
    pub collider: Collider,
    pub collision: CollisionDamage,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_position, update_velocity)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_systems(Update, block_movement_controls.in_set(InGameSet::UserInput));
    }
}
