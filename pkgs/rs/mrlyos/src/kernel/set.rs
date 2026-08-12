use mrlycore::{json, Json};

/// Sets the slot to the value's integer and echoes it, refusing non-integers and values outside the range.
///
/// ```
/// use mrlycore::json;
/// let mut slot = 0;
/// assert_eq!(mrlyos::kernel::int(&mut slot, &json!(7), (0, 9)), Ok(json!(7)));
/// assert_eq!(slot, 7);
/// ```
pub fn int(slot: &mut i64, value: &Json, range: (i64, i64)) -> Result<Json, &'static str> {
    let n = value.as_i64().ok_or("value must be an integer")?;
    if !(range.0..=range.1).contains(&n) {
        return Err("out of range");
    }
    *slot = n;
    Ok(json!(n))
}

/// Sets the slot to the value's bool and echoes it, refusing non-bools.
pub fn flag(slot: &mut bool, value: &Json) -> Result<Json, &'static str> {
    let on = value.as_bool().ok_or("value must be a bool")?;
    *slot = on;
    Ok(json!(on))
}

/// Sets the slot to the value's string and echoes it, refusing strings outside the options.
pub fn pick(slot: &mut String, value: &Json, options: &[&str]) -> Result<Json, &'static str> {
    let s = value.as_str().ok_or("value must be a string")?;
    if !options.contains(&s) {
        return Err("no such option");
    }
    *slot = s.to_string();
    Ok(json!(s))
}

/// Applies the callback to each key and value of an object, and to nothing else.
pub fn drive<F: FnMut(&str, &Json)>(value: &Json, mut apply: F) {
    if let Some(obj) = value.as_object() {
        for (key, val) in obj {
            apply(key, val);
        }
    }
}
