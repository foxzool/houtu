use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use houtu::HoutuPlugin;
use houtu_3d_tiles::HoutuTileset;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,naga=warn,houtu=debug".to_string(),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(HoutuPlugin)
        .add_systems(Startup, load_tileset)
        .run();
}

fn load_tileset(mut commands: Commands) {
    commands.spawn(HoutuTileset::from_url(
        "https://sandcastle.cesium.com/SampleData/Cesium3DTiles/Tilesets/Tileset/tileset.json",
    ));
}
