use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::EguiContext;
use crate::{entities::Bullet, health::Health, schedule::InGameSet};
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, health_ui.in_set(InGameSet::EntityUpdates));
    }
}

fn health_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    entity_health: Query<(Entity, &Health), Without <Bullet>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Health").show(context.get_mut(), |ui| {
            for (e, health) in entity_health.iter() {
                ui.heading(format!(
                    "{:#?}'s Health: {:#}",
                    e, health.value
                ));
            }
        });
    }
}