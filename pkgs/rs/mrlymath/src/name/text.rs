use mrlycore::errors::{value_error, Result};

pub(super) fn compose(kind: &str, fields: &[String]) -> String {
    let mut out = format!("mrly_{kind}");
    for field in fields {
        out.push('_');
        out.push_str(field);
    }
    out
}

pub(super) fn split<'a>(text: &'a str, kind: &str) -> Result<Vec<&'a str>> {
    if !text
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return value_error(format!(
            "name {text:?} strays outside a-z, 0-9 and underscore."
        ));
    }
    let head = format!("mrly_{kind}_");
    let Some(body) = text.strip_prefix(&head) else {
        return value_error(format!("name {text:?} does not start with {head:?}."));
    };
    let fields: Vec<&str> = body.split('_').collect();
    if fields.iter().any(|f| f.is_empty()) {
        return value_error(format!("name {text:?} holds an empty field."));
    }
    Ok(fields)
}

pub(super) fn run(tags: &[(char, Option<u128>)]) -> String {
    let mut out = String::new();
    for (tag, value) in tags {
        out.push(*tag);
        if let Some(value) = value {
            out.push_str(&value.to_string());
        }
    }
    out
}

pub(super) fn tags(field: &str) -> Result<Vec<(char, Option<u128>)>> {
    let mut out = Vec::new();
    let mut rest = field.chars().peekable();
    while let Some(tag) = rest.next() {
        if !tag.is_ascii_lowercase() {
            return value_error(format!("field {field:?} opens a tag with {tag:?}."));
        }
        let mut digits = String::new();
        while let Some(&c) = rest.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            digits.push(c);
            rest.next();
        }
        if digits.is_empty() {
            out.push((tag, None));
        } else {
            out.push((tag, Some(number(&digits)?)));
        }
    }
    if out.is_empty() {
        return value_error("field is empty.");
    }
    Ok(out)
}

pub(super) fn number(digits: &str) -> Result<u128> {
    if digits.len() > 1 && digits.starts_with('0') {
        return value_error(format!("number {digits:?} wears a leading zero."));
    }
    match digits.parse::<u128>() {
        Ok(n) => Ok(n),
        Err(_) => value_error(format!("number {digits:?} does not read.")),
    }
}

pub(super) fn small(value: u128) -> Result<usize> {
    usize::try_from(value)
        .map_err(|_| mrlycore::errors::MrlyError::Value(format!("value {value} is too large.")))
}
