use bevy::prelude::*;
use bevy_http_client::{HttpRequest, HttpResponse};
use serde_json::Value;

pub use core::*;
use houtu_resource::{HoutuNetResourcePlugin, HoutuNetworkResource};

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
        (Added<HoutuTileset>, Without<HoutuNetworkResource>),
    >,
) {
    for (entity, tileset) in q_added_tile_set.iter() {
        let net_res = HoutuNetworkResource::new(tileset.url.clone());
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

fn process_metadata_extension(resource: &HoutuNetworkResource, tileset_json: Value) {
    let metadata_json = if has_extension(&tileset_json, "3DTILES_metadata") {
        tileset_json["extensions"]["3DTILES_metadata"].clone()
    } else {
        tileset_json.clone()
    };

    if let Some(schema_uri) = metadata_json.get("schemaUri") {
        debug!("schemaUri: {}", schema_uri);
        let resource = resource.clone().set_url(schema_uri.to_string().as_str());
    }
}

fn has_extension(json: &Value, extension_name: &str) -> bool {
    match json.get("extensions") {
        None => false,
        Some(ext) => ext.get(extension_name).is_some(),
    }
}
