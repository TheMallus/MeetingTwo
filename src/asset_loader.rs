use bevy::prelude::*;
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub block: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    *scene_assets = SceneAssets {
        block: asset_server.load("test.glb#Scene0"),
    }
}