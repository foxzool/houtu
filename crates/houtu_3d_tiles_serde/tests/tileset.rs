#[cfg(test)]
mod test_tileset {
    use houtu_3d_tiles_serde::Tileset;
    use std::path::PathBuf;

    #[test]
    fn test_tileset_with_discrete_lod() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("../../samples/3d-tiles-samples/1.0/TilesetWithDiscreteLOD/tileset.json");
        let file = std::fs::File::open(file_path).unwrap();
        let tileset_json: Tileset = serde_json::from_reader(&file).unwrap();

        println!("{:?}", tileset_json);
    }

    #[test]
    fn test_tileset_with_request_volume() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("../../samples/3d-tiles-samples/1.0/TilesetWithRequestVolume/tileset.json");
        let file = std::fs::File::open(file_path).unwrap();
        let tileset_json: Tileset = serde_json::from_reader(&file).unwrap();

        println!("{:?}", tileset_json);
    }

    #[test]
    fn test_tileset_with_tree_billboards() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("../../samples/3d-tiles-samples/1.0/TilesetWithTreeBillboards/tileset.json");
        let file = std::fs::File::open(file_path).unwrap();
        let tileset_json: Tileset = serde_json::from_reader(&file).unwrap();

        println!("{:?}", tileset_json);
    }
}
