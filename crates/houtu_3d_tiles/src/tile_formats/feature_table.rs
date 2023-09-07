use serde::{Deserialize, Serialize};

/// An object defining the offset into a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryBodyOffset {
    pub byte_offset: u64,
}

/// An object defining the reference to a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BinaryBodyReference {
    /// The datatype of components in the property. This is defined only if the semantic allows for overriding the implicit component type. These cases are specified in each tile format.
    pub component_type: ComponentType,
}

/// The datatype of components in the property. This is defined only if the semantic allows for overriding the implicit component type. These cases are specified in each tile format.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ComponentType {
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    INT,
    UNSIGNED_INT,
    FLOAT,
    DOUBLE,
    Other(String),
}

impl<'de> serde::Deserialize<'de> for ComponentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "BYTE" => Ok(ComponentType::BYTE),
            "UNSIGNED_BYTE" => Ok(ComponentType::UNSIGNED_BYTE),
            "SHORT" => Ok(ComponentType::SHORT),
            "UNSIGNED_SHORT" => Ok(ComponentType::UNSIGNED_SHORT),
            "INT" => Ok(ComponentType::INT),
            "UNSIGNED_INT" => Ok(ComponentType::UNSIGNED_INT),
            "FLOAT" => Ok(ComponentType::FLOAT),
            "DOUBLE" => Ok(ComponentType::DOUBLE),
            _ => Ok(ComponentType::Other(value)),
        }
    }
}

impl serde::Serialize for ComponentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ComponentType::BYTE => serializer.serialize_str("BYTE"),
            ComponentType::UNSIGNED_BYTE => serializer.serialize_str("UNSIGNED_BYTE"),
            ComponentType::SHORT => serializer.serialize_str("SHORT"),
            ComponentType::UNSIGNED_SHORT => serializer.serialize_str("UNSIGNED_SHORT"),
            ComponentType::INT => serializer.serialize_str("INT"),
            ComponentType::UNSIGNED_INT => serializer.serialize_str("UNSIGNED_INT"),
            ComponentType::FLOAT => serializer.serialize_str("FLOAT"),
            ComponentType::DOUBLE => serializer.serialize_str("DOUBLE"),
            ComponentType::Other(value) => serializer.serialize_str(value),
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub enum GlobalPropertyInteger {
    BinaryBodyOffset(BinaryBodyOffset),
    Integer(i64),
}

impl<'de> Deserialize<'de> for GlobalPropertyInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => {
                        Ok(GlobalPropertyInteger::BinaryBodyOffset(binary_body_offset))
                    }
                    Err(_) => Err(serde::de::Error::custom("byteOffset must be defined")),
                }
            }
            serde_json::Value::Number(number) => match number.as_i64() {
                None => {
                    let number = number.as_f64().unwrap();
                    Ok(GlobalPropertyInteger::Integer(number as i64))
                }
                Some(integer) => Ok(GlobalPropertyInteger::Integer(integer)),
            },
            _ => Err(serde::de::Error::custom(
                "byteOffset or integer must be defined",
            )),
        }
    }
}

/// A `GlobalPropertyCartesian3` object defining a 3-component numeric property for all features. Details about this property are described in the 3D Tiles specification.
#[derive(Debug, Serialize, PartialEq)]
pub enum GlobalPropertyCartesian3 {
    BinaryBodyOffset(BinaryBodyOffset),
    Cartesian3([f64; 3]),
}

impl<'de> Deserialize<'de> for GlobalPropertyCartesian3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Object(object) => {
                match serde_json::from_value(serde_json::to_value(object).unwrap()) {
                    Ok(binary_body_offset) => Ok(GlobalPropertyCartesian3::BinaryBodyOffset(
                        binary_body_offset,
                    )),
                    Err(_) => Err(serde::de::Error::custom("byteOffset must be defined")),
                }
            }
            serde_json::Value::Array(value) => {
                if value.len() == 3 {
                    let mut array = [0.0; 3];
                    for (i, v) in value.iter().enumerate() {
                        if let Some(v) = v.as_f64() {
                            array[i] = v;
                        } else {
                            return Err(serde::de::Error::custom("Invalid array"));
                        }
                    }
                    Ok(GlobalPropertyCartesian3::Cartesian3(array))
                } else {
                    Err(serde::de::Error::custom("Invalid array"))
                }
            }
            _ => Err(serde::de::Error::custom(
                "byteOffset, cartesian3 must be defined",
            )),
        }
    }
}

/// A user-defined property which specifies application-specific metadata in a tile. Values can refer to sections in the binary body with a `BinaryBodyReference` object. Global values can be also be defined directly in the JSON.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Property {
    /// An object defining the offset into a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.    
    Offset(BinaryBodyOffset),
    /// An object defining the reference to a section of the binary body of the features table where the property values are stored if not defined directly in the JSON.
    Reference(BinaryBodyReference),
    /// An object defining a global boolean property value for all features.
    Boolean(bool),
    /// An object defining a global integer property value for all features.
    Integer(i64),
    /// An object defining a global numeric property value for all features.
    GlobalPropertyNumber(f64),
    /// An object defining a global 3-component numeric property values for all features.
    GlobalPropertyCartesian3([f64; 3]),
    /// An object defining a global 4-component numeric property values for all features.
    GlobalPropertyCartesian4([f64; 4]),
}

impl<'de> serde::Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        match value {
            serde_json::Value::Bool(value) => Ok(Property::Boolean(value)),
            serde_json::Value::Number(value) => {
                if let Some(value) = value.as_i64() {
                    Ok(Property::Integer(value))
                } else if let Some(value) = value.as_f64() {
                    Ok(Property::GlobalPropertyNumber(value))
                } else {
                    Err(serde::de::Error::custom("Invalid number"))
                }
            }
            serde_json::Value::Array(value) => {
                if value.len() == 3 {
                    let mut array = [0.0; 3];
                    for (i, v) in value.iter().enumerate() {
                        if let Some(v) = v.as_f64() {
                            array[i] = v;
                        } else {
                            return Err(serde::de::Error::custom("Invalid array"));
                        }
                    }
                    Ok(Property::GlobalPropertyCartesian3(array))
                } else if value.len() == 4 {
                    let mut array = [0.0; 4];
                    for (i, v) in value.iter().enumerate() {
                        if let Some(v) = v.as_f64() {
                            array[i] = v;
                        } else {
                            return Err(serde::de::Error::custom("Invalid array"));
                        }
                    }
                    Ok(Property::GlobalPropertyCartesian4(array))
                } else {
                    Err(serde::de::Error::custom("Invalid array"))
                }
            }
            serde_json::Value::Object(value) => {
                if let Some(value) = value.get("byteOffset") {
                    let byte_offset = value
                        .as_u64()
                        .ok_or_else(|| serde::de::Error::custom("Invalid byteOffset"))?;
                    Ok(Property::Offset(BinaryBodyOffset { byte_offset }))
                } else if let Some(value) = value.get("componentType") {
                    let component_type =
                        serde_json::from_value(serde_json::to_value(value).unwrap())
                            .map_err(|_| serde::de::Error::custom("Invalid componentType"))?;
                    Ok(Property::Reference(BinaryBodyReference { component_type }))
                } else {
                    Err(serde::de::Error::custom("Invalid object"))
                }
            }

            _ => Err(serde::de::Error::custom("Invalid json body")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_property() {
        let json = r#"
        {
            "byteOffset": 10
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::Offset(BinaryBodyOffset { byte_offset: 10 })
        );

        let json = r#"
        {
            "componentType": "INT"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::Reference(BinaryBodyReference {
                component_type: ComponentType::INT
            })
        );

        let json = r#"
        {
            "componentType": "OTHER"
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::Reference(BinaryBodyReference {
                component_type: ComponentType::Other("OTHER".to_string())
            })
        );

        let json = r#"true"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(property, Property::Boolean(true));

        let json = r#"1"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(property, Property::Integer(1));

        let json = r#"1.0"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(property, Property::GlobalPropertyNumber(1.0));

        let json = r#"[1.0, 2.0, 3.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyCartesian3([1.0, 2.0, 3.0])
        );

        let json = r#"[1.0, 2.0, 3.0, 4.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        let property: Property = serde_json::from_value(json_value).unwrap();
        assert_eq!(
            property,
            Property::GlobalPropertyCartesian4([1.0, 2.0, 3.0, 4.0])
        );

        let json = r#"[1.0, 2.0]"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(serde_json::from_value::<Property>(json_value).is_err());

        let json = r#"
        {
            "byteOffset": 1.0
        }"#;
        let json_value: serde_json::Value = serde_json::from_str(json).unwrap();

        assert!(serde_json::from_value::<Property>(json_value).is_err());
    }
}
