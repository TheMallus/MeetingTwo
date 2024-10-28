use bevy::prelude::*;
use crate::health::Health;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position)
        .add_systems(Update, print_health);
    }
}

//                                               Debug
// ====================================================================================================

// Our block moves now! that's the end of the demo extension, last step we want to add for debugging cases
// (or for future purpose if you want to figure out why something doesn't work), we'll quickly make a function
// that will output to the terminal the blocks position at every frame (this will look like spam when
// when running the program but it'll tell you when the block changes position)

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, transform.translation);
    } // function basically prints location of ALL existing entities per frame
}

fn print_health(query: Query<(Entity, &Health)>) {
    for  (entity, health) in query.iter() {
        info!("Entity {:?} has {:?} health.", entity, health.value);
    }
}