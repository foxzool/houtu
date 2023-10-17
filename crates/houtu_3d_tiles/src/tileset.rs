use bevy::prelude::*;
use bevy_http_client::{ehttp, HttpClientPlugin, HttpRequest, HttpResponse};
use houtu_3d_tiles_serde::Tileset;

pub struct TilesetPlugin;

impl Plugin for TilesetPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<HttpClientPlugin>() {
            app.add_plugins(HttpClientPlugin);
        }
        app.add_systems(Update, (added_url, handle_tile_json));
    }
}

#[derive(Debug, Component)]
pub struct TilesetUrl(pub String);

impl TilesetUrl {
    pub fn new(url: &str) -> Self {
        Self(url.to_string())
    }
}

fn added_url(mut commands: Commands, q_added_url: Query<(Entity, &TilesetUrl), Added<TilesetUrl>>) {
    for (entity, tileset_url) in q_added_url.iter() {
        debug!("load tileset url: {:?}", tileset_url);
        commands
            .entity(entity)
            .insert(HttpRequest(ehttp::Request::get(tileset_url.0.clone())));
    }
}

fn handle_tile_json(
    mut commands: Commands,
    q_tile_json: Query<(Entity, &TilesetUrl, &HttpResponse), Added<HttpResponse>>,
) {
    for (entity, tileset_url, response) in q_tile_json.iter() {
        if response.ok {
            let tileset_json: Tileset = serde_json::from_slice(&response.bytes).unwrap();
            debug!("{:#?}", tileset_json);
        } else {
            error!(
                "url {} load error: {:?}",
                tileset_url.0, response.status_text
            );
        }
        commands.entity(entity).remove::<HttpRequest>();
    }
}
