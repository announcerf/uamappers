use serde_json::Value;

/// Avoid duplication, strip only the top-level `"id"` field from cached payloads
pub fn strip_top_level_id(mut value: Value) -> Value {
    let Value::Object(obj) = &mut value else {
        return value;
    };

    obj.remove("id");
    value
}
