use serde::{Deserialize, Serialize};

use crate::common::RootProperty;

/// A bounding volume that encloses a tile or its content.
/// At least one bounding volume property is required.
/// Bounding volumes include `box`, `region`, or `sphere`.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct BoundingVolume {
    /// A basis for storing extensions and extras.
    #[serde(flatten)]
    pub root: RootProperty,
    /// An array of 12 numbers that define an oriented bounding box.
    /// The first three elements define the x, y, and z values for the center of the box.
    /// The next three elements (with indices 3, 4, and 5) define the x axis direction and half-length.
    /// The next three elements (indices 6, 7, and 8) define the y axis direction and half-length.
    /// The last three elements (indices 9, 10, and 11) define the z axis direction and half-length.
    #[serde(rename = "box")]
    pub r#box: Option<[f64; 12]>,
    /// An array of six numbers that define a bounding geographic region
    /// in EPSG:4979 coordinates with the order [west, south, east, north, minimum height, maximum height].
    /// Longitudes and latitudes are in radians.
    /// The range for latitudes is [-PI/2,PI/2].
    /// The range for longitudes is [-PI,PI].
    /// The value that is given as the 'south' of the region shall not be larger than the value for the 'north' of the region.
    /// The heights are in meters above (or below) the WGS84 ellipsoid.
    /// The 'minimum height' shall not be larger than the 'maximum height'.
    pub region: Option<[f64; 6]>,
    /// An array of four numbers that define a bounding sphere.
    /// The first three elements define the x, y, and z values for the center of the sphere.
    /// The last element (with index 3) defines the radius in meters.
    /// The radius shall not be negative.
    pub sphere: Option<[f64; 4]>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_bounding_value() {
        let json = json!({
            "box": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0],
            "region": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            "sphere": [1.0, 2.0, 3.0, 4.0],
        });
        let bounding_volume: super::BoundingVolume = serde_json::from_value(json).unwrap();
        assert_eq!(
            bounding_volume.r#box,
            Some([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0])
        );
        assert_eq!(bounding_volume.region, Some([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        assert_eq!(bounding_volume.sphere, Some([1.0, 2.0, 3.0, 4.0]));
    }
}
