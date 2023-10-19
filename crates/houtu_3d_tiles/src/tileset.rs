use bevy::prelude::*;
use bevy_http_client::{ehttp, HttpClientPlugin, HttpRequest, HttpResponse};

mod core;

pub use core::*;

pub struct TilesetPlugin;

impl Plugin for TilesetPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<HttpClientPlugin>() {
            app.add_plugins(HttpClientPlugin);
        }
        app.add_systems(Update, (added_tileset, handle_remote_tile_json));
    }
}

fn added_tileset(
    mut commands: Commands,
    q_added_url: Query<(Entity, &HoutuTileset), Added<HoutuTileset>>,
) {
    for (entity, tileset) in q_added_url.iter() {
        // match tileset.base_path.scheme() {
        //     "http" | "https" => {
        //         debug!("load tileset from remote url: {:?}", tileset.base_path);
        //         commands
        //             .entity(entity)
        //             .insert(HttpRequest(ehttp::Request::get(tileset.base_path.as_str())));
        //     }
        //     _ => {
        //         warn!("url {} not reader to handle", tileset.base_path.as_str());
        //         continue;
        //     }
        // }
    }
}

fn handle_remote_tile_json(
    mut commands: Commands,
    mut q_tile_json: Query<(Entity, &mut HoutuTileset, &HttpResponse), Added<HttpResponse>>,
) {
    for (entity, mut tileset, response) in q_tile_json.iter_mut() {
        // if response.ok {
        //     let tileset_json: crate::specification::Tileset =
        //         serde_json::from_slice(&response.bytes).unwrap();
        //     debug!("{:#?}", tileset_json);
        // } else {
        //     error!(
        //         "url {} load error: {:?}",
        //         tileset.base_path, response.status_text
        //     );
        // }
        // commands.entity(entity).remove::<HttpRequest>();
    }
}
