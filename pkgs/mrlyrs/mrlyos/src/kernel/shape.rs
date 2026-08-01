use mrlycore::json::Map;
use mrlycore::Json;

pub fn prune(state: &Json, shape: &Json) -> Json {
    match (state.as_object(), shape.as_object()) {
        (Some(fields), Some(keys)) => {
            let mut out = Map::new();
            for (key, sub) in keys {
                if let Some(value) = fields.get(key) {
                    out.insert(key.clone(), prune(value, sub));
                }
            }
            Json::Obj(out)
        }
        _ => state.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::json;

    #[test]
    fn nested_objects_prune() {
        let state = json!({ "a": { "x": 1, "y": 2 }, "b": 3 });
        let shape = json!({ "a": { "x": 1 } });
        assert_eq!(prune(&state, &shape), json!({ "a": { "x": 1 } }));
    }

    #[test]
    fn arrays_taken_whole() {
        let state = json!({ "items": [{ "id": 1 }, { "id": 2 }], "n": 2 });
        let shape = json!({ "items": 1 });
        assert_eq!(
            prune(&state, &shape),
            json!({ "items": [{ "id": 1 }, { "id": 2 }] })
        );
    }

    #[test]
    fn missing_keys_skipped() {
        let state = json!({ "a": 1 });
        let shape = json!({ "a": 1, "ghost": 1 });
        assert_eq!(prune(&state, &shape), json!({ "a": 1 }));
    }

    #[test]
    fn non_object_shape_takes_all() {
        let state = json!({ "a": 1, "b": [2, 3] });
        assert_eq!(prune(&state, &json!(1)), state);
        assert_eq!(prune(&state, &json!(true)), state);
    }

    #[test]
    fn object_shape_on_scalar_takes_value() {
        let state = json!({ "count": 5 });
        let shape = json!({ "count": { "x": 1 } });
        assert_eq!(prune(&state, &shape), json!({ "count": 5 }));
    }
}
