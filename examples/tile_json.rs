use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use houtu_3d_tiles_serde::Tileset;
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, load_json)
        .run();
}

fn load_json() {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("samples/3d-tiles-samples/1.0/TilesetWithRequestVolume/tileset.json");
    let file = std::fs::File::open(file_path).unwrap();
    let tileset_json: Tileset = serde_json::from_reader(&file).unwrap();

    println!("{:#?}", tileset_json);
}
