use serde_json::Value;

pub fn strip_mapset_raw(value: Value) -> Value {
    let Value::Object(obj) = value else {
        return Value::Null;
    };

    serde_json::json!({
        "ratings": obj.get("ratings").cloned(),
        "animeCover": obj.get("anime_cover").cloned()
    })
}
