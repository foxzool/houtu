use serde::ser::SerializeSeq;

/// An array of numeric values
pub type NumericArray1D = Vec<f64>;

/// An array of arrays of numeric values"
pub type NumericArray2D = Vec<Vec<f64>>;

/// An array of boolean values
pub type BooleanArray1D = Vec<bool>;

/// An array of string values
pub type StringArray1D = Vec<String>;

/// For `SCALAR` this is a number. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
#[derive(Debug, PartialEq)]
pub enum NumericValue {
    Numeric(f64),
    Array1D(NumericArray1D),
    Array2D(NumericArray2D),
}

impl<'de> serde::Deserialize<'de> for NumericValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(number) => {
                if let Some(number) = number.as_f64() {
                    Ok(NumericValue::Numeric(number))
                } else {
                    Err(serde::de::Error::custom("Not a valid numeric value"))
                }
            }
            serde_json::Value::Array(array) => {
                let mut numeric_array_1d = Vec::new();
                let mut numeric_array_2d = Vec::new();
                for value in array {
                    if let serde_json::Value::Number(number) = value {
                        if let Some(number) = number.as_f64() {
                            numeric_array_1d.push(number);
                        } else {
                            return Err(serde::de::Error::custom("Not a valid numeric value"));
                        }
                    } else if let serde_json::Value::Array(array) = value {
                        let numeric_array_1d = array
                            .into_iter()
                            .map(|e| e.as_f64().unwrap())
                            .collect::<Vec<f64>>();

                        numeric_array_2d.push(numeric_array_1d);
                    } else {
                        return Err(serde::de::Error::custom("Not a valid numeric value"));
                    }
                }
                if numeric_array_1d.len() > 0 {
                    Ok(NumericValue::Array1D(numeric_array_1d))
                } else if numeric_array_2d.len() > 0 {
                    Ok(NumericValue::Array2D(numeric_array_2d))
                } else {
                    Err(serde::de::Error::custom("Not a valid numeric value"))
                }
            }

            _ => Err(serde::de::Error::custom("Not a valid numeric value")),
        }
    }
}

impl serde::Serialize for NumericValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NumericValue::Numeric(value) => serializer.serialize_f64(*value),
            NumericValue::Array1D(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            NumericValue::Array2D(array) => {
                let value = serde_json::to_value(&array);

                match value {
                    Ok(value) => value.serialize(serializer),
                    Err(_) => Err(serde::ser::Error::custom("Not a valid numeric value")),
                }
            }
        }
    }
}

/// For `SCALAR` this is a number. For `STRING` this is a string. For `ENUM` this is a string that shall be a valid enum `name`, not an integer value. For `VECN` this is an array of `N` numbers. For `MATN` this is an array of `N²` numbers. For fixed-length arrays this is an array of `count` elements of the given `type`.
#[derive(Debug, PartialEq)]
pub enum NoDataValue {
    Numeric(f64),
    Array1D(NumericArray1D),
    Array2D(NumericArray2D),
    String(String),
    String1D(Vec<String>),
}

impl serde::Serialize for NoDataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NoDataValue::Numeric(value) => serializer.serialize_f64(*value),
            NoDataValue::Array1D(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            NoDataValue::Array2D(array) => {
                let value = serde_json::to_value(&array);

                match value {
                    Ok(value) => value.serialize(serializer),
                    Err(_) => Err(serde::ser::Error::custom("Not a valid numeric value")),
                }
            }
            NoDataValue::String(value) => serializer.serialize_str(value),
            NoDataValue::String1D(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for NoDataValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Number(number) => {
                if let Some(number) = number.as_f64() {
                    Ok(NoDataValue::Numeric(number))
                } else {
                    Err(serde::de::Error::custom("Not a valid numeric value"))
                }
            }
            serde_json::Value::Array(array) => {
                let mut numeric_array_1d = Vec::new();
                let mut numeric_array_2d = Vec::new();
                let mut string_array_1d = Vec::new();
                for value in array {
                    if let serde_json::Value::Number(number) = value {
                        if let Some(number) = number.as_f64() {
                            numeric_array_1d.push(number);
                        } else {
                            return Err(serde::de::Error::custom("Not a valid numeric value"));
                        }
                    } else if let serde_json::Value::Array(array) = value {
                        let numeric_array_1d = array
                            .into_iter()
                            .map(|e| e.as_f64().unwrap())
                            .collect::<Vec<f64>>();
                        numeric_array_2d.push(numeric_array_1d);
                    } else if let serde_json::Value::String(string) = value {
                        string_array_1d.push(string);
                    } else {
                        return Err(serde::de::Error::custom("Not a valid numeric value"));
                    }
                }
                if numeric_array_1d.len() > 0 {
                    Ok(NoDataValue::Array1D(numeric_array_1d))
                } else if numeric_array_2d.len() > 0 {
                    Ok(NoDataValue::Array2D(numeric_array_2d))
                } else if string_array_1d.len() > 0 {
                    Ok(NoDataValue::String1D(string_array_1d))
                } else {
                    Err(serde::de::Error::custom("Not a valid no data value"))
                }
            }
            serde_json::Value::String(string) => Ok(NoDataValue::String(string)),

            _ => Err(serde::de::Error::custom("Not a valid no data value")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AnyValue {
    Numeric(NumericValue),
    Boolean(bool),
    Boolean1D(Vec<bool>),
    String(String),
    String1D(Vec<String>),
}

impl<'de> serde::Deserialize<'de> for AnyValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::Number(number) => {
                if let Some(number) = number.as_f64() {
                    Ok(AnyValue::Numeric(NumericValue::Numeric(number)))
                } else {
                    Err(serde::de::Error::custom("Not a valid numeric value"))
                }
            }
            serde_json::Value::Array(array) => {
                let mut numeric_array_1d = Vec::new();
                let mut numeric_array_2d = Vec::new();
                let mut boolean_array_1d = Vec::new();
                let mut string_array_1d = Vec::new();
                for value in array {
                    if let serde_json::Value::Number(number) = value {
                        if let Some(number) = number.as_f64() {
                            numeric_array_1d.push(number);
                        } else {
                            return Err(serde::de::Error::custom("Not a valid numeric value"));
                        }
                    } else if let serde_json::Value::Array(array) = value {
                        let numeric_array_1d = array
                            .into_iter()
                            .map(|e| e.as_f64().unwrap())
                            .collect::<Vec<f64>>();
                        numeric_array_2d.push(numeric_array_1d);
                    } else if let serde_json::Value::Bool(boolean) = value {
                        boolean_array_1d.push(boolean);
                    } else if let serde_json::Value::String(string) = value {
                        string_array_1d.push(string);
                    } else {
                        return Err(serde::de::Error::custom("Not a valid numeric value"));
                    }
                }
                if numeric_array_1d.len() > 0 {
                    Ok(AnyValue::Numeric(NumericValue::Array1D(numeric_array_1d)))
                } else if numeric_array_2d.len() > 0 {
                    Ok(AnyValue::Numeric(NumericValue::Array2D(numeric_array_2d)))
                } else if boolean_array_1d.len() > 0 {
                    Ok(AnyValue::Boolean1D(boolean_array_1d))
                } else if string_array_1d.len() > 0 {
                    Ok(AnyValue::String1D(string_array_1d))
                } else {
                    Err(serde::de::Error::custom("Not a valid no data value"))
                }
            }
            serde_json::Value::String(string) => Ok(AnyValue::String(string)),
            serde_json::Value::Bool(boolean) => Ok(AnyValue::Boolean(boolean)),
            _ => Err(serde::de::Error::custom("Not a valid no data value")),
        }
    }
}

impl serde::Serialize for AnyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AnyValue::Numeric(value) => value.serialize(serializer),
            AnyValue::Boolean(value) => serializer.serialize_bool(*value),
            AnyValue::Boolean1D(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
            AnyValue::String(value) => serializer.serialize_str(value),
            AnyValue::String1D(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for value in array {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_numeric_value() {
        let json = r#"1.0"#;
        let numeric_value: NumericValue = serde_json::from_str(json).unwrap();
        assert_eq!(numeric_value, NumericValue::Numeric(1.0));

        let json = r#"[1.0, 2.0, 3.0]"#;
        let numeric_value: NumericValue = serde_json::from_str(json).unwrap();
        assert_eq!(numeric_value, NumericValue::Array1D(vec![1.0, 2.0, 3.0]));

        let json = r#"[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]"#;
        let numeric_value: NumericValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            numeric_value,
            NumericValue::Array2D(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]])
        );
    }

    #[test]
    fn serialize_numeric_value() {
        let numeric_value = NumericValue::Numeric(1.0);
        let json = serde_json::to_string(&numeric_value).unwrap();
        assert_eq!(json, r#"1.0"#);

        let numeric_value = NumericValue::Array1D(vec![1.0, 2.0, 3.0]);
        let json = serde_json::to_string(&numeric_value).unwrap();
        assert_eq!(json, r#"[1.0,2.0,3.0]"#);

        let numeric_value = NumericValue::Array2D(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let json = serde_json::to_string(&numeric_value).unwrap();
        assert_eq!(json, r#"[[1.0,2.0,3.0],[4.0,5.0,6.0]]"#);
    }

    #[test]
    fn deserialize_no_data_value() {
        let json = r#"1.0"#;
        let no_data_value: NoDataValue = serde_json::from_str(json).unwrap();
        assert_eq!(no_data_value, NoDataValue::Numeric(1.0));

        let json = r#"[1.0, 2.0, 3.0]"#;
        let no_data_value: NoDataValue = serde_json::from_str(json).unwrap();
        assert_eq!(no_data_value, NoDataValue::Array1D(vec![1.0, 2.0, 3.0]));

        let json = r#"[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]"#;
        let no_data_value: NoDataValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            no_data_value,
            NoDataValue::Array2D(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]])
        );

        let json = r#""test""#;
        let no_data_value: NoDataValue = serde_json::from_str(json).unwrap();
        assert_eq!(no_data_value, NoDataValue::String("test".to_string()));

        let json = r#"["test1", "test2", "test3"]"#;
        let no_data_value: NoDataValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            no_data_value,
            NoDataValue::String1D(vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string(),
            ])
        );
    }

    #[test]
    fn serialize_no_data_value() {
        let no_data_value = NoDataValue::Numeric(1.0);
        let json = serde_json::to_string(&no_data_value).unwrap();
        assert_eq!(json, r#"1.0"#);

        let no_data_value = NoDataValue::Array1D(vec![1.0, 2.0, 3.0]);
        let json = serde_json::to_string(&no_data_value).unwrap();
        assert_eq!(json, r#"[1.0,2.0,3.0]"#);

        let no_data_value = NoDataValue::Array2D(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let json = serde_json::to_string(&no_data_value).unwrap();
        assert_eq!(json, r#"[[1.0,2.0,3.0],[4.0,5.0,6.0]]"#);

        let no_data_value = NoDataValue::String("test".to_string());
        let json = serde_json::to_string(&no_data_value).unwrap();
        assert_eq!(json, r#""test""#);

        let no_data_value = NoDataValue::String1D(vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ]);
        let json = serde_json::to_string(&no_data_value).unwrap();
        assert_eq!(json, r#"["test1","test2","test3"]"#);
    }

    #[test]
    fn deserialize_any_value() {
        let json = r#"1.0"#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(any_value, AnyValue::Numeric(NumericValue::Numeric(1.0)));

        let json = r#"[1.0, 2.0, 3.0]"#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            any_value,
            AnyValue::Numeric(NumericValue::Array1D(vec![1.0, 2.0, 3.0]))
        );

        let json = r#"[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]"#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            any_value,
            AnyValue::Numeric(NumericValue::Array2D(vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
            ]))
        );

        let json = r#""test""#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(any_value, AnyValue::String("test".to_string()));

        let json = r#"["test1", "test2", "test3"]"#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(
            any_value,
            AnyValue::String1D(vec![
                "test1".to_string(),
                "test2".to_string(),
                "test3".to_string(),
            ])
        );

        let json = r#"true"#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(any_value, AnyValue::Boolean(true));

        let json = r#"[true, false, true]"#;
        let any_value: AnyValue = serde_json::from_str(json).unwrap();
        assert_eq!(any_value, AnyValue::Boolean1D(vec![true, false, true]));
    }

    #[test]
    fn serialize_any_value() {
        let any_value = AnyValue::Numeric(NumericValue::Numeric(1.0));
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#"1.0"#);

        let any_value = AnyValue::Numeric(NumericValue::Array1D(vec![1.0, 2.0, 3.0]));
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#"[1.0,2.0,3.0]"#);

        let any_value = AnyValue::Numeric(NumericValue::Array2D(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ]));
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#"[[1.0,2.0,3.0],[4.0,5.0,6.0]]"#);

        let any_value = AnyValue::String("test".to_string());
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#""test""#);

        let any_value = AnyValue::String1D(vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ]);
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#"["test1","test2","test3"]"#);

        let any_value = AnyValue::Boolean(true);
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#"true"#);

        let any_value = AnyValue::Boolean1D(vec![true, false, true]);
        let json = serde_json::to_string(&any_value).unwrap();
        assert_eq!(json, r#"[true,false,true]"#);
    }
}
