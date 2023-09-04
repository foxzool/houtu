use houtu_3d_tiles::asset::Asset;

#[test]
fn test_de() {
    let json = r#"
    {
        "version": "1.0",
        "tilesetVersion": "1.0.0",
        "extras": {
            "name": "test"
        }
    }
    "#;
    let asset: Asset = serde_json::from_str(json).unwrap();
    assert_eq!(asset.version, "1.0");
    assert_eq!(asset.tileset_version, Some("1.0.0".to_string()));
    assert_eq!(
        asset.extras,
        Some(serde_json::json!({
            "name": "test"
        }))
    );
}
