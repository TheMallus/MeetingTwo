pub mod block;
pub mod combat;
pub mod debug;
pub mod despawn;
pub mod entities;
pub mod movement;
pub mod schedule;
pub mod setup;
pub mod states;
pub mod ui;
pub mod asset_loader;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use block::BlockPlugin;
use combat::CombatManagementPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use setup::StartingWorldPlugin;
use states::StatePlugin;
use ui::UIPlugin;
use asset_loader::AssetLoaderPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetLoaderPlugin)
        // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // combat
        .add_plugins(CombatManagementPlugin)
        //.add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BlockPlugin)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(StartingWorldPlugin)
        // .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(StatePlugin)
        .run();
}
