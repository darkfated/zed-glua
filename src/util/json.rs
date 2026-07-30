use zed::serde_json::{Map, Value};
use zed_extension_api as zed;

pub fn get_or_insert_object<'a>(
    map: &'a mut Map<String, Value>,
    key: &str,
) -> &'a mut Map<String, Value> {
    let value = map
        .entry(key.to_owned())
        .or_insert_with(|| Value::Object(Map::new()));

    if !value.is_object() {
        *value = Value::Object(Map::new());
    }

    value.as_object_mut().expect("value should be an object")
}

pub fn get_or_insert_array<'a>(map: &'a mut Map<String, Value>, key: &str) -> &'a mut Vec<Value> {
    let value = map
        .entry(key.to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));

    if !value.is_array() {
        *value = Value::Array(Vec::new());
    }

    value.as_array_mut().expect("value should be an array")
}
