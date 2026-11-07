use mrly_error::{Error, Result, value_error};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug)]
pub(crate) enum Value {
    Text(String),
    Number(u64),
    List(Vec<String>),
}

#[derive(Clone, Debug)]
pub(crate) struct Collection {
    pub(crate) name: String,
    fields: BTreeMap<String, Value>,
}

fn unquote(text: &str) -> Option<String> {
    let inner = text.trim().strip_prefix('"')?.strip_suffix('"')?;
    Some(inner.to_string())
}

fn value(text: &str) -> Option<Value> {
    if let Some(inner) = text.strip_prefix('[') {
        let inner = inner.strip_suffix(']')?;
        return Some(Value::List(
            inner
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(|part| unquote(part).unwrap_or_else(|| part.to_string()))
                .collect(),
        ));
    }
    if text.starts_with('"') {
        return unquote(text).map(Value::Text);
    }
    text.parse().ok().map(Value::Number)
}

pub(crate) fn names(root: &Path) -> Result<Vec<String>> {
    let dir = root.join("collections");
    let mut out = Vec::new();
    for entry in
        std::fs::read_dir(&dir).map_err(|e| Error::Value(format!("{}: {e}", dir.display())))?
    {
        let path = entry.map_err(|e| Error::Value(e.to_string()))?.path();
        if path.join("collection.toml").is_file()
            && let Some(name) = path.file_name().and_then(|n| n.to_str())
        {
            out.push(name.to_string());
        }
    }
    out.sort();
    Ok(out)
}

impl Collection {
    pub(crate) fn read(root: &Path, name: &str) -> Result<Collection> {
        let path = root.join("collections").join(name).join("collection.toml");
        let body = std::fs::read_to_string(&path)
            .map_err(|e| Error::Value(format!("{}: {e}", path.display())))?;
        let mut fields = BTreeMap::new();
        for line in body.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, raw)) = line.split_once('=') else {
                return value_error(format!("{}: {line:?} wants key = value.", path.display()));
            };
            let Some(parsed) = value(raw.trim()) else {
                return value_error(format!("{}: {raw:?} is no value.", path.display()));
            };
            fields.insert(key.trim().to_string(), parsed);
        }
        Ok(Collection {
            name: name.to_string(),
            fields,
        })
    }

    pub(crate) fn text(&self, key: &str) -> Option<&str> {
        match self.fields.get(key) {
            Some(Value::Text(text)) => Some(text),
            _ => None,
        }
    }

    pub(crate) fn number(&self, key: &str) -> Option<u64> {
        match self.fields.get(key) {
            Some(Value::Number(n)) => Some(*n),
            _ => None,
        }
    }

    pub(crate) fn numbers(&self, key: &str) -> Result<Option<Vec<u64>>> {
        let Some(Value::List(list)) = self.fields.get(key) else {
            return Ok(None);
        };
        list.iter()
            .map(|item| match item.parse() {
                Ok(n) => Ok(n),
                Err(_) => value_error(format!(
                    "{}: {key} holds {item:?}, not a number.",
                    self.name
                )),
            })
            .collect::<Result<Vec<_>>>()
            .map(Some)
    }

    pub(crate) fn list(&self, key: &str) -> Vec<String> {
        match self.fields.get(key) {
            Some(Value::List(list)) => list.clone(),
            _ => Vec::new(),
        }
    }
}
