use serde_json::Value;

pub fn strip_mapset_raw(mut value: Value) -> Value {
    let Value::Object(obj) = &mut value else {
        return value;
    };

    obj.remove("id");
    value
}
