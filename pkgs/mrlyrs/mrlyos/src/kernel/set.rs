use serde_json::{json, Value as Json};

pub fn int(slot: &mut i64, value: &Json, range: (i64, i64)) -> Result<Json, &'static str> {
    let n = value.as_i64().ok_or("value must be an integer")?;
    if !(range.0..=range.1).contains(&n) {
        return Err("out of range");
    }
    *slot = n;
    Ok(json!(n))
}

pub fn real(slot: &mut f64, value: &Json, range: (f64, f64)) -> Result<Json, &'static str> {
    let n = value.as_f64().ok_or("value must be a number")?;
    if n < range.0 || n > range.1 {
        return Err("out of range");
    }
    *slot = n;
    Ok(json!(n))
}

pub fn flag(slot: &mut bool, value: &Json) -> Result<Json, &'static str> {
    let on = value.as_bool().ok_or("value must be a bool")?;
    *slot = on;
    Ok(json!(on))
}

pub fn pick(slot: &mut String, value: &Json, options: &[&str]) -> Result<Json, &'static str> {
    let s = value.as_str().ok_or("value must be a string")?;
    if !options.contains(&s) {
        return Err("no such option");
    }
    *slot = s.to_string();
    Ok(json!(s))
}

pub fn drive<F: FnMut(&str, &Json)>(value: &Json, mut apply: F) {
    if let Some(obj) = value.as_object() {
        for (key, val) in obj {
            apply(key, val);
        }
    }
}
