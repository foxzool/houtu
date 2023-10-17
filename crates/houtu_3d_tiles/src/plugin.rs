use bevy::prelude::*;

pub struct Houtu3DTilesPlugin;

impl Plugin for Houtu3DTilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::TilesetPlugin);
    }
}
