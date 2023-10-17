use bevy::prelude::*;
use houtu_3d_tiles::Houtu3DTilesPlugin;

pub struct HoutuPlugin;

impl Plugin for HoutuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Houtu3DTilesPlugin);
    }
}
