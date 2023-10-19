use anyhow::{anyhow, Result};
use bevy::prelude::*;
use url::Url;

#[derive(Debug, Component)]
pub struct HoutuTileset {
    base_path: Url,
}

impl HoutuTileset {
    const SUPPORTED_EXTENSIONS: [&'static str; 8] = [
        "3DTILES_metadata",
        "3DTILES_implicit_tiling",
        "3DTILES_content_gltf",
        "3DTILES_multiple_contents",
        "3DTILES_bounding_volume_S2",
        "3DTILES_batch_table_hierarchy",
        "3DTILES_draco_point_compression",
        "MAXAR_content_geojson",
    ];

    pub fn from_url(url: &str) -> Self {
        let url = Url::parse(url).expect("parse url error");
        Self { base_path: url }
    }

    fn load_tileset(&mut self, tileset_json: crate::specification::Tileset) -> Result<()> {
        Self::check_version(&tileset_json)?;
        Self::check_supported_extensions(&tileset_json)?;

        Ok(())
    }

    /// Check if the version of the tileset is supported.
    fn check_version(tileset_json: &crate::specification::Tileset) -> Result<()> {
        match tileset_json.asset.version.as_str() {
            "0.0" | "1.0" | "1.1" => {}
            _ => {
                return Err(anyhow!(
                    "tileset version not support: {}",
                    tileset_json.asset.version
                ));
            }
        }
        Ok(())
    }

    /// Check if the extensions of the tileset are supported.
    fn check_supported_extensions(tileset_json: &crate::specification::Tileset) -> Result<()> {
        if let Some(extension_required) = &tileset_json.extensions_required {
            for extension in extension_required.iter() {
                if !Self::SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
                    return Err(anyhow!("Unsupported extension: {}", extension));
                }
            }
        }

        Ok(())
    }
}
