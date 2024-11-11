use bevy::prelude::*;

use crate::schedule::InGameSet;

use super::systems::*;

pub struct CombatManagementPlugin;

impl Plugin for CombatManagementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, apply_collision_damage);
        app.add_systems(Update, block_weapons.in_set(InGameSet::UserInput));
    }
}
