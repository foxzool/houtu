use houtu_3d_tiles::schema::Schema;

#[test]
fn test_schema() {
    let json = r#"
        {
        "id": "SimplePropertyTextureSchema",
        "classes" : {
          "buildingComponents" : {
            "name" : "Building properties",
            "properties" : {
              "insideTemperature" : {
                "name" : "Inside Temperature",
                "type" : "SCALAR",
                "componentType" : "UINT8"
              },
              "outsideTemperature" : {
                "name" : "Outside Temperature",
                "type" : "SCALAR",
                "componentType" : "UINT8"
              },
              "insulation" : {
                "name" : "Insulation Thickness",
                "type" : "SCALAR",
                "componentType" : "UINT8",
                "normalized" : true
              }
            }
          }
        }
      }
    
    "#;
    let schema: Schema = serde_json::from_str(json).unwrap();

    assert_eq!(schema.id, "SimplePropertyTextureSchema");
    let classes = schema.classes.unwrap();
    assert_eq!(classes.len(), 1);
    let building_components = classes.get("buildingComponents").unwrap();
    assert_eq!(
        building_components.name.as_deref(),
        Some("Building properties")
    );
    let properties = building_components.properties.as_ref().unwrap();
    assert_eq!(properties.len(), 3);
    assert!(properties.get("insideTemperature").is_some());
    assert!(properties.get("outsideTemperature").is_some());
    assert!(properties.get("insulation").is_some());
}
