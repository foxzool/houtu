use bevy::prelude::*;
use bevy_http_client::{HttpClientPlugin, HttpRequest, HttpResponse};

pub use core::*;
use houtu_resource::{HoutuNetResourcePlugin, NetworkResource};

mod core;

pub struct TilesetPlugin;

impl Plugin for TilesetPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<HoutuNetResourcePlugin>() {
            app.add_plugins(HoutuNetResourcePlugin);
        }
        app.add_systems(Update, (added_tileset, handle_remote_tile_json));
    }
}

fn added_tileset(
    mut commands: Commands,
    q_added_tile_set: Query<
        (Entity, &HoutuTileset),
        (Added<HoutuTileset>, Without<NetworkResource>),
    >,
) {
    for (entity, tileset) in q_added_tile_set.iter() {
        let net_res = NetworkResource::new(tileset.url.clone());
        debug!(
            "load tileset from remote url: {:?}",
            tileset.url.to_string()
        );
        commands.entity(entity).insert(net_res);
    }
}

fn handle_remote_tile_json(
    mut commands: Commands,
    mut q_tile_json: Query<(Entity, &mut HoutuTileset, &HttpResponse), Added<HttpResponse>>,
) {
    for (entity, mut tileset, response) in q_tile_json.iter_mut() {
        if response.ok {
            let tileset_json: crate::specification::Tileset =
                serde_json::from_slice(&response.bytes).unwrap();
            debug!("{:#?}", tileset_json);
        } else {
            error!("url {} load error: {:?}", tileset.url, response.status_text);
        }
        commands.entity(entity).remove::<HttpRequest>();
    }
}
