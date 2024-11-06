pub mod debug;
pub mod health;
pub mod movement;
pub mod block;
pub mod setup;
pub mod collision_detector;
pub mod weapons;
pub mod despawn;
pub mod entities;
pub mod ui;
pub mod states;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use block::BlockPlugin;
use setup::StartingWorldPlugin;
use collision_detector::CollisionDetectionPlugin;
use weapons::WeaponsPlugin;
use despawn::DespawnPlugin;
use ui::UIPlugin;
use states::StatePlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BlockPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(StartingWorldPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(WeaponsPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(StatePlugin)
        .run();
}